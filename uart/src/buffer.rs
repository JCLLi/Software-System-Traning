use cortex_m_semihosting::hprintln;

/// The UART driver can only send one byte at a time, but a print may produce more than one byte.
/// The bytes that cannot yet be sent must be buffered. This struct is responsible for that.
///
/// You are free to implement this in any way you like, but we recommend using a RingBuffer.
/// https://en.wikipedia.org/wiki/Circular_buffer
///
/// The buffer should have a maximum capacity (since we don't have infinite memory), in practice a 256-byte capacity works fine.
pub struct UartBuffer {
    buffer: [u8; 257],
    start: usize,
    pub end: usize
}

/// These are the errors that can happen during the operation of the buffer. You may add some as necessary.
#[derive(Debug)]
pub enum UartBufferError {
    Empty_Buffer,
    Full_Buffer,
}

pub type UartBufferResult<T> = Result<T, UartBufferError>;

impl UartBuffer {
    pub fn new() -> Self {
        UartBuffer { buffer: [0; 257], start: 0, end: 0 }
    }

    /// Obtain the next byte from the buffer
    pub fn read_byte(&mut self) -> UartBufferResult<u8> {
        if self.is_empty() {
            return Err(UartBufferError::Empty_Buffer);
        }
        //self.start = (self.start + 1) % 256;
        //hprintln!("{}", self.start);
        let value = self.buffer.get(self.start).unwrap().clone();
        self.tick_start();
        Ok(value)
    }

    /// Write a byte to the buffer
    pub fn write_byte(&mut self, byte: u8) -> UartBufferResult<()> {
        if self.is_full() {
            return Err(UartBufferError::Full_Buffer);
        }
        self.buffer[self.end] = byte;
        self.tick_end();
        Ok(())        
    }

    pub fn clear(&mut self) {
        self.start = 0;
        self.end = 0;
    }

    pub fn overwrite(&mut self, byte: u8) {
        if self.is_full(){
            self.tick_start();
        }
        self.buffer[self.end] = byte;
        self.tick_end();
    }

    pub fn is_empty(&mut self) -> bool {
        self.start == self.end
    }

    pub fn tick_start(&mut self) {
        self.start = (self.start + 1) % 256;
    }

    pub fn is_full(&mut self) -> bool {
        (self.end + 1) % 256 == self.start
    }

    pub fn tick_end(&mut self) {
        self.end = (self.end + 1) % 256;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_write_byte() {
        let mut buffer = UartBuffer::new();
        buffer.write_byte(0x42).unwrap();
        assert_eq!(buffer.read_byte().unwrap(), 0x42);
    }

    // You can add more tests here if you want, but you don't have to.
}
