/// The UART driver can only send one byte at a time, but a print may produce more than one byte.
/// The bytes that cannot yet be sent must be buffered. This struct is responsible for that.
///
/// You are free to implement this in any way you like, but we recommend using a RingBuffer.
/// https://en.wikipedia.org/wiki/Circular_buffer
///
/// The buffer should have a maximum capacity (since we don't have infinite memory), in practice a 256-byte capacity works fine.
pub struct UartBuffer {}

/// These are the errors that can happen during the operation of the buffer. You may add some as necessary.
#[derive(Debug)]
pub enum UartBufferError {}

pub type UartBufferResult<T> = Result<T, UartBufferError>;

impl UartBuffer {
    pub fn new() -> Self {
        todo!()
    }

    /// Obtain the next byte from the buffer
    pub fn read_byte(&mut self) -> UartBufferResult<u8> {
        todo!()
    }

    /// Write a byte to the buffer
    pub fn write_byte(&mut self, byte: u8) -> UartBufferResult<()> {
        todo!()
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
