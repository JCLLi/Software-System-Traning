use alloc::collections::BTreeMap;
use core::ops::Deref;
use core::ptr::slice_from_raw_parts;
use cortex_m::asm;
use crate::UART;
use nrf51_pac::{interrupt, Interrupt, UART0};
use nrf51_pac::NVIC;
use nrf51_pac::uart0;
use nrf51_pac::uart0::{baudrate, enable, intenclr, intenset, tasks_startrx, tasks_starttx, txd};
use crate::buffer::UartBuffer;
use cortex_m_semihosting::{hprint, hprintln};
use heapless::Vec;
use nrf51_pac::ficr::ER;
use crate::data_format::Function;
use crate::data_format::Function::{DELETE, ERROR, READ};

/// This struct holds all the global state of the uart driver. Generally, there
/// is only one instance of this in the global `UART` variable (see `main.rs`).
///
/// You will need to add fields to this.
pub struct UartDriver {
    pub uart: nrf51_pac::UART0,
    pub buffer: UartBuffer,
    pub tx_filled: bool,
    pub notes: Vec<Option<(u8, [u8; 20])>, 20>,
    //pub notes: [[u8; 2]; 5],
    //pub notes: [Option<[u8; 20]>; 5],
}

impl UartDriver {
    /// Create a new instance `of the UART controller.
    ///
    /// This function can only be called once since UART0 only exists
    /// once and is moved into the driver here.
    pub fn new(uart: nrf51_pac::UART0, nvic: &mut NVIC) -> Self {
        // In this function the following things are done:
        // 1. Enable the UART peripheral
        // 2. Configure the UART peripheral
        // 3. Configure the UART interrupt
        // 5. Set the interrupt priority
        let mut notes: Vec<Option<(u8, [u8; 20])>, 20> = Vec::new();
        for i in 0..20 {
            notes.push(None);
        }
        let mut uart_driver = UartDriver{
            uart,
            buffer: UartBuffer::new(),
            tx_filled: false,
            notes,
        };

        //hprintln!("start: {}", uart_driver.buffer.start);
        //hprintln!("end: {}", uart_driver.buffer.end);

        uart_driver.uart.pselrts.reset();
        uart_driver.uart.pselrts.reset();
        uart_driver.uart.pselrxd.reset();
        uart_driver.uart.pseltxd.reset();

        uart_driver.uart.baudrate.write(|w: &mut baudrate::W| w.baudrate().baud1200());
        uart_driver.uart.config.reset();

        uart_driver.uart.enable.write(|w: &mut enable::W| {
            w.enable().enabled()
        });

        /// The function bits() is unsafe because numbers except 0 or 1 written in tasks_startrx/tx
        /// can cause fatal errors. But here it is necessary and sound. Because the task_startrx/tx
        /// needs to be enabled or triggered to start tx/rx. Write a 0 to it is disabling the task and
        /// a 1 mean the task is enabled. It basically won't cause fatal error.
        ///
        uart_driver.uart.tasks_startrx.write(|w: &mut tasks_startrx::W| unsafe {w.bits(1)});
        uart_driver.uart.tasks_starttx.write(|w: &mut tasks_starttx::W| unsafe {w.bits(1)});

        /// Changing priority levels can break priority-based critical sections. If several kinds of
        /// interrupts are used. Mistakenly changing the priority can cause effect on urgent tasks
        /// that originally required high-priority execution. And when different interrupt are with
        /// same priority, it can cause errors. But because only one uart interrupt is used here so
        /// it can be taken as a safe function.
        ///
        unsafe {
            nvic.set_priority(Interrupt::UART0 ,1);
        }

        /// This function is unsafe because it can break mask-based critical sections. If the program
        /// about several thread rely on the Mutex or something else with critical sections, which
        /// need masked interrupt to prevent share data access. But in our case, no several threads,
        /// the only usage of Mutex is prevent the happening of interrupt when the program is trying
        /// to modify the static value.
        ///
        unsafe {
            NVIC::unmask(Interrupt::UART0);
        }


        uart_driver
    }

    /// This function enables the UART interrupt
    pub fn enable(&self) {
        self.uart.intenset.write(|w: &mut intenset::W| {
            w.rxdrdy().set()
        });
        self.uart.intenset.write(|w: &mut intenset::W| {
            w.txdrdy().set()
        });
    }

    /// Pushes a single byte over uart
    pub fn put_byte(&mut self, byte: u8) {
        loop {
            if !self.buffer.is_full(){break}
        }

        self.buffer.write_byte(byte);
    }

    /// Reads a single byte from the UART. You may need a buffer to implement this.
    pub fn get_byte(&mut self) -> Option<u8> {
        if let Ok(res) = self.buffer.read_byte(){
            return Some(res);
        }
        None
    }

    /// Writes the entire buffer over UART. You may need a buffer to implement this.
    pub fn put_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.put_byte(*byte);
        }
    }

    /// Reads as many bytes as possible from the UART
    pub fn get_bytes(&mut self, bytes: &mut [u8]) -> usize {
        let mut i = 0;
        while let Some(byte) = self.get_byte() {
            bytes[i] = byte;
            i += 1;
            if i == bytes.len() {
                break;
            }
        }
        i
    }

    pub fn save_note(&mut self, note: [u8; 20], len: u8) -> Result<u8, ()>{
        let position = self.notes.iter().position(|x: &Option<(u8, [u8; 20])>| x.is_none());

        match position {
            None => {
                return Err(());
            }
            Some(ID) => {
                 //self.notes.insert(ID, Some(note));
                self.notes[ID] = Some((len, note));
                return Ok(ID as u8);
            }
        };
    }

    pub fn delete_note(&mut self, ID: u8) -> (u8, [u8; 20], Function){
        let mut output: [u8; 20] = [0; 20];
        if ID == 0{
            let error = "Doesn't exist".as_bytes();
            for i in 0..error.len() {
                output[i] = error[i];
            }
            return (error.len() as u8, output, ERROR);
        }
        let ID = ID -1;
        if ID > 19 {
            let error = "Out of scope".as_bytes();
            for i in 0..error.len() {
                output[i] = error[i];
            }
            return (error.len() as u8, output, ERROR);
        }
        if self.notes[ID as usize] == None{
            let error = "No related note ".as_bytes();
            for i in 0..error.len(){
                output[i] = error[i];
            }
            return (error.len() as u8, output, ERROR)
        }
        else {
            self.notes[ID as usize] = None;
            let delete = "Deleted".as_bytes();
            for i in 0..delete.len(){
                output[i] = delete[i];
            }
            return (delete.len() as u8, output, DELETE);
        }


    }

    pub fn read_note(&mut self, ID: u8) -> (u8, [u8; 20], Function) {
        let mut output = [0; 20];
        if ID == 0{
            let error = "Doesn't exist".as_bytes();
            for i in 0..error.len() {
                output[i] = error[i];
            }
            return (error.len() as u8, output, ERROR);
        }
        let ID = ID -1;
        if ID > 19{
            let error = "Out of scope".as_bytes();
            for i in 0..error.len() {
                output[i] = error[i];
            }
            return (error.len() as u8, output, ERROR);
        }
        if let Some((len, note)) = self.notes[ID as usize]{
            for i in 0..20 {
                output[i] = note[i];
            }
            return (len, output, READ);
        } else {
            let error = "Doesn't exist".as_bytes();
            for i in 0..error.len() {
                output[i] = error[i];
            }
            return (error.len() as u8, output, ERROR);
        }
    }
}

#[interrupt]
/// Interrupt handler for UART0
/// It's called when the enabled interrupts for uart0 are triggered
fn UART0() {

    UART.modify(|uart| if uart.uart.events_rxdrdy.read().bits() != 0 {
        uart.uart.events_rxdrdy.reset();
        let byte = uart.uart.rxd.read().bits() as u8;
        if !uart.buffer.is_full() {
            uart.buffer.write_byte(byte).unwrap();
        }else {
            uart.buffer.overwrite(byte);
        }
    });

    UART.modify(|uart| if uart.uart.events_txdrdy.read().bits() != 0 {

        uart.uart.events_txdrdy.reset();
        if !uart.buffer.is_empty() {
            let byte = uart.buffer.read_byte().unwrap();
            /// This one is unsafe because of any unknown possible number can be written into txd
            /// register. But in this case, the size of variable 'byte' input into the txd has a fixed
            /// size of u8 which make it sound.
            ///
            unsafe {uart.uart.txd.write(|w: &mut txd::W| w.txd().bits(byte));}
            uart.tx_filled = true;
        }else {
            uart.tx_filled = false;
        }
        
    });
}
