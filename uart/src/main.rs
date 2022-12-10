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

use heapless::Vec;
use postcard::to_vec;
use crate::data_format::Function::ADD;
use crate::data_format::NewProtocol;

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
    loop {
            //hprintln!("here1");
            let r = UART.modify(|uart| uart.get_bytes(&mut buf));


            for i in 0..r{
                input.push(buf[i]);
            }

            if let Ok(res) = NewProtocol::new_from_uart(&input){
                let mut buf: [u8; 29] = [0; 29];

                if res.function == 0x01{

                   if let Ok(ID) = UART.modify(|uart| uart.save_note(res.data)) {
                       //UART.modify(|uart| hprintln!("len: {}", uart.notes.len()));

                       let mut note: [u8; 20] = [0; 20];
                       for i in 0.."Done".as_bytes().len(){
                           note[i] = ("Done".as_bytes())[i];
                       }

                       NewProtocol::new_to_uart(&mut buf, ADD, note, ID, 4);

                       /////

                       // let mut sum: u32 = 0x00;
                       // for i in 0..4{
                       //     sum += note[i as usize] as u32;
                       // }
                       // sum = sum + 0x69 + 0x69 + 0x01 + ID as u32 + 4 as u32;
                       // let mut check_sum:[u8; 4] = [0, 0, 0, 0];
                       // for i in 0..4{
                       //     check_sum[i] = (sum & 0xff) as u8;
                       //     sum = sum >> 8;
                       // }
                       // let output = NewProtocol{
                       //     start_num: [0x69, 0x69],
                       //     function: 0x01,
                       //     id: ID,
                       //     data_len: 4,
                       //     data: note,
                       //     check_sum,
                       // };
                       // let serial: Vec<u8, 29> = to_vec(&output).unwrap();
                       // for i in 0..serial.len(){
                       //     buf[i] = serial[i];
                       // }
                       // UART.modify(|uart| hprintln!("{}", uart.buffer.end));
                       // UART.modify(|uart| hprintln!("len {}", uart.notes.len()));

                       ///////
                       UART.modify(|uart| {
                           uart.put_bytes(&buf);
                           hprintln!("{}", uart.buffer.is_empty());
                           if !uart.tx_filled{
                               hprintln!("Nothing to send on TXD");
                               let byte: u8 = uart.buffer.read_byte().unwrap();
                               hprintln!("Nothing");

                               unsafe {uart.uart.txd.write(|w: &mut nrf51_pac::uart0::txd::W| w.txd().bits(byte));}
                           }
                       });
                   }
                   else {
                       hprintln!("Note full, saving note unsuccessful");
                   }

                }else if res.function == 0x03{
                    //let data = UART.modify(|uart| uart.delete_note(res.id));
                    todo!()
                }else{
                    // let data = UART.modify(|uart| uart.read_note(res.id));
                    // NewProtocol::new_to_uart()
                    todo!()
                }
                // UART.modify(|uart| {
                //     uart.save_note(res.data);
                //     for i in 0..20{
                //         if let Some((id, note)) = uart.notes[0]{
                //             hprintln!("{}", note[i]);
                //         }
                //
                //     }
                // });

                // hprintln!("Header: {}{}", res.start_num[0], res.start_num[1]);
                // hprintln!("Function: {}", res.function);
                // hprintln!("ID: {}", res.id);
                // hprintln!("Length: {}", res.data_len);
                // for i in 0..res.data_len{
                //     hprintln!("Data: {}", res.data[i as usize]);
                // }
                // for i in 0..4{
                //     hprintln!("Checksum: {}", res.check_sum[i]);
                // }
                for i in 0..input.len(){
                    input.pop();
                }
            }
        // {
        //     let r = UART.modify(|uart| uart.get_bytes(&mut buf));
        //
        //     if let Ok((s, size)) = ExampleProtocol::from_uart(&buf[..r]) {
        //         buf.rotate_left(size);
        //
        //         let mut b = [0u8; 64];
        //         let len = match s {
        //             ExampleProtocol::Number(n) => {
        //                 let v = ExampleProtocol::Number(n + 1);
        //                 //hprintln!("iam here");
        //                 v.to_uart(&mut b).unwrap()
        //             }
        //             ExampleProtocol::Text(s) => {
        //                 if s == "Hello" {
        //                     ExampleProtocol::Text("World".to_string())
        //                         .to_uart(&mut b)
        //                         .unwrap()
        //                 } else {
        //                     ExampleProtocol::Text("Hello".to_string())
        //                         .to_uart(&mut b)
        //                         .unwrap()
        //                 }
        //             }
        //         };
        //         UART.modify(|uart| {
        //             uart.put_bytes(&b[0..len]);
        //             if !uart.tx_filled{
        //                 hprintln!("Nothing to send on TXD");
        //                 let byte: u8 = uart.buffer.read_byte().unwrap();
        //                 unsafe {uart.uart.txd.write(|w: &mut nrf51_pac::uart0::txd::W| w.txd().bits(byte));}
        //             }
        //         });
        //     }
        // }

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
