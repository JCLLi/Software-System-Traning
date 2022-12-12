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

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};

use heapless::Vec;
use postcard::to_vec;
use crate::data_format::Function::{ADD, DELETE, ERROR, READ};
use crate::data_format::{NewProtocol, UartError};

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
    let mut input: Vec<u8, 29> = Vec::new();
    let data_loss = false;
    let mut data_trans = false;
    let mut wait_cycle = 0;
    loop {
            //hprintln!("here1");
            let r = UART.modify(|uart| uart.get_bytes(&mut buf));
            if r != 0 {
                data_trans = true;
                wait_cycle = 0;
            }
            for i in 0..r{
                input.push(buf[i]);
            }
            match NewProtocol::new_from_uart(&input) {
                Ok(res) =>{
                    let mut buf: [u8; 29] = [0; 29];
                    if res.function == 0x01{
                        if let Ok(ID) = UART.modify(|uart| uart.save_note(res.data, res.data_len)) {
                            let mut note: [u8; 20] = [0; 20];
                            for i in 0.."Done".as_bytes().len(){
                                note[i] = ("Done".as_bytes())[i];
                            }
                            NewProtocol::new_to_uart(&mut buf, ADD, note, ID + 1, "Done".as_bytes().len() as u8);
                        }
                        else {
                            let mut note: [u8; 20] = [0; 20];
                            for i in 0.."Failed, no space".as_bytes().len(){
                                note[i] = ("Failed, no space".as_bytes())[i];
                            }
                            NewProtocol::new_to_uart(&mut buf, ADD, note, 0, "Failed, no space".as_bytes().len() as u8);
                        }
                    }else if res.function == 0x03{
                        let (len, data) = UART.modify(|uart| uart.delete_note(res.id));
                        NewProtocol::new_to_uart(&mut buf, DELETE, data, res.id, len);
                    }else{
                        let (len, data) = UART.modify(|uart| uart.read_note(res.id));
                        NewProtocol::new_to_uart(&mut buf, READ, data, res.id, len);
                    }

                    UART.modify(|uart| {
                        uart.put_bytes(&buf);
                        if !uart.tx_filled{
                            let byte: u8 = uart.buffer.read_byte().unwrap();
                            unsafe {uart.uart.txd.write(|w: &mut nrf51_pac::uart0::txd::W| w.txd().bits(byte));}
                        }
                    });

                    for i in 0..input.len(){ input.pop(); }
                    data_trans = false;
                    wait_cycle = 0;
                }
                Err(err) => {
                    match err {
                        UartError::NotEnoughBytes => {
                            if wait_cycle == 100{
                                for i in 0..buf.len(){
                                    buf[i] = 0;
                                }
                                let error = "Data loss!!".as_bytes();
                                let mut data:[u8; 20] = [0; 20];
                                for i in 0..error.len() {
                                    data[i] = error[i];
                                }
                                let mut output: [u8; 29] = [0; 29];
                                NewProtocol::new_to_uart(&mut output, ERROR, data, 0, error.len() as u8);
                                UART.modify(|uart| {
                                    uart.put_bytes(&output);
                                    if !uart.tx_filled{
                                        let byte: u8 = uart.buffer.read_byte().unwrap();
                                        unsafe {uart.uart.txd.write(|w: &mut nrf51_pac::uart0::txd::W| w.txd().bits(byte));}
                                    }
                                });
                                wait_cycle = 0;
                                data_trans = false;
                                for i in 0..input.len(){ input.pop(); }
                            }
                        }
                        UartError::MessageWrong => {
                            for i in 0..buf.len(){
                                buf[i] = 0;
                            }
                            let error = "Message wrong!!".as_bytes();
                            let mut data:[u8; 20] = [0; 20];
                            for i in 0..error.len() {
                                data[i] = error[i];
                            }
                            let mut output: [u8; 29] = [0; 29];
                            NewProtocol::new_to_uart(&mut output, ERROR, data, 0, error.len() as u8);
                            UART.modify(|uart| {
                                uart.put_bytes(&output);
                                if !uart.tx_filled{
                                    let byte: u8 = uart.buffer.read_byte().unwrap();
                                    unsafe {uart.uart.txd.write(|w: &mut nrf51_pac::uart0::txd::W| w.txd().bits(byte));}
                                }
                            });
                            wait_cycle = 0;
                            data_trans = false;
                            for i in 0..input.len(){ input.pop(); }
                        }
                        _ => ()
                    }
                }
            }

        asm::delay(2500000);
        if data_trans {
            wait_cycle += 1;
        }


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
