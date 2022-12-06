#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

extern crate alloc;
extern crate nrf51_hal;
mod buffer;
mod data_format;
mod once_cell;
mod own_mutex;
mod uart_driver;

#[cfg(test)]
mod test;

use alloc::string::ToString;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
#[cfg(not(test))]
use core::panic::PanicInfo;
use cortex_m::asm;
use once_cell::OnceCell;
use own_mutex::OwnMutex;
use uart_driver::UartDriver;

use data_format::ExampleProtocol;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 4096; // in bytes

pub static UART: OwnMutex<OnceCell<UartDriver>> = OwnMutex::new(OnceCell::new());

#[entry]
fn main() -> ! {
    // initialize the allocator
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    // get the peripherals from cortex_m
    let mut pc = cortex_m::Peripherals::take().unwrap();

    // modify the global uart driver to initialize it
    UART.modify(|uart| {
        // get the peripherals from the pac
        let pn = nrf51_hal::pac::Peripherals::take().unwrap();

        // actually initialize the uart driver (through the OnceCell)
        uart.initialize(UartDriver::new(pn.UART0, &mut pc.NVIC));
        // and start it
        uart.enable();
    });

    // if compiling for testing, run tests here
    #[cfg(test)]
    test_main();

    // then infinitely listen for messages. Read them into buf, deserialize them, create
    // a response, serialize the response and write it back to the UART.
    let mut buf = [0u8; 64];
    let mut a = 0;

    loop {
        a = a + 1;
        {   //hprintln!("{}",a);
            let r = UART.modify(|uart| uart.get_bytes(&mut buf));

            if let Ok((s, size)) = ExampleProtocol::from_uart(&buf[..r]) {
                buf.rotate_left(size);

                let mut b = [0u8; 64];
                let len = match s {
                    ExampleProtocol::Number(n) => {
                        let v = ExampleProtocol::Number(n + 1);
                        //hprintln!("iam here");
                        v.to_uart(&mut b).unwrap()
                    }
                    ExampleProtocol::Text(s) => {
                        if s == "Hello" {
                            ExampleProtocol::Text("World".to_string())
                                .to_uart(&mut b)
                                .unwrap()
                        } else {
                            ExampleProtocol::Text("Hello".to_string())
                                .to_uart(&mut b)
                                .unwrap()
                        }
                    }
                };
                UART.modify(|uart| {
                    uart.put_bytes(&b[0..len]);
                    if !uart.tx_filled{
                        hprintln!("Nothing to send on TXD");
                        let byte: u8 = uart.buffer.read_byte().unwrap();
                        unsafe {uart.uart.txd.write(|w: &mut nrf51_pac::uart0::txd::W| w.txd().bits(byte));}
                    }
                });
            }
        }

        // Wait a bit for the next uart message. This is just so we don't overflow the UART's buffers
        // by sending too many messages. In reality, you may want to do this more accurately, like waiting
        // for the buffers to empty, or waiting for an exact number of (milli)seconds.
        asm::delay(2500000);
    }
}

// called when memory allocation failed. Required to import `alloc`
#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("Alloc error! {layout:?}");
}

// called when a panic happens
#[inline(never)]
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use cortex_m_semihosting::hprintln;
    hprintln!("RUST PANIC: {}", info);

    // infinitely loops
    loop {}
}
