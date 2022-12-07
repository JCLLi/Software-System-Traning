use core::ops::Deref;
use core::ptr::slice_from_raw_parts;
use cortex_m::asm;
use crate::UART;
use nrf51_pac::{interrupt, Interrupt, UART0};
use nrf51_pac::NVIC;
use nrf51_pac::uart0;
use nrf51_pac::uart0::{baudrate, enable, intenclr, intenset, tasks_startrx, tasks_starttx, txd};
use crate::buffer::UartBuffer;
use cortex_m_semihosting::hprintln;

/// This struct holds all the global state of the uart driver. Generally, there
/// is only one instance of this in the global `UART` variable (see `main.rs`).
///
/// You will need to add fields to this.
pub struct UartDriver {
    pub uart: nrf51_pac::UART0,
    pub buffer: UartBuffer,
    pub tx_filled: bool,
}

impl UartDriver {
    /// Create a new instance of the UART controller.
    ///
    /// This function can only be called once since UART0 only exists
    /// once and is moved into the driver here.
    pub fn new(uart: nrf51_pac::UART0, nvic: &mut NVIC) -> Self {
        // In this function the following things are done:
        // 1. Enable the UART peripheral
        // 2. Configure the UART peripheral
        // 3. Configure the UART interrupt
        // 5. Set the interrupt priority

        let uart_driver = UartDriver{
            uart,
            buffer: UartBuffer::new(),
            tx_filled: false,
        };


        uart_driver.uart.pselrts.reset();
        uart_driver.uart.pselrts.reset();
        uart_driver.uart.pselrxd.reset();
        uart_driver.uart.pseltxd.reset();

        uart_driver.uart.baudrate.write(|w: &mut baudrate::W| w.baudrate().baud1200());
        uart_driver.uart.config.reset();

        uart_driver.uart.enable.write(|w: &mut enable::W| {
            w.enable().enabled()
        });

        uart_driver.uart.tasks_startrx.write(|w: &mut tasks_startrx::W| unsafe {w.bits(1)});
        uart_driver.uart.tasks_starttx.write(|w: &mut tasks_starttx::W| unsafe {w.bits(1)});
        unsafe {
            nvic.set_priority(Interrupt::UART0 ,7);
        }
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
        //self.uart.txd.write(|w: &mut txd::W| unsafe{w.txd().bits(0)});
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
}

#[interrupt]
/// Interrupt handler for UART0
/// It's called when the enabled interrupts for uart0 are triggered
unsafe fn UART0() {
    cortex_m::interrupt::disable();
    hprintln!("The interrupt occured");

    UART.modify(|uart| if uart.uart.events_rxdrdy.read().bits() != 0 {
        uart.uart.events_rxdrdy.reset();
        let byte = uart.uart.rxd.read().bits() as u8;
        if !uart.buffer.is_full() {
            uart.buffer.write_byte(byte).unwrap();
            hprintln!("Now reading {} from RXD into ringbuffer", byte as char);
        }else {
            uart.buffer.overwrite(byte);
            hprintln!("Now reading {} from RXD and OVERwriting into ringbuffer", byte as char);
        }
    });

    UART.modify(|uart| if uart.uart.events_txdrdy.read().bits() != 0 {
        uart.uart.events_txdrdy.reset();
        if !uart.buffer.is_empty() {
            let byte = uart.buffer.read_byte().unwrap();
            unsafe {uart.uart.txd.write(|w: &mut txd::W| w.txd().bits(byte));}
            uart.tx_filled = true;
            hprintln!("Now sending {} from ringbuffer via TXD", byte as char);
        }else {
            uart.tx_filled = false;
        }
        
    });
}
