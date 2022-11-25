//! This file is mostly the same as runner/src/main.rs. For more complex protocols, you may
//! want to factor this out into its own crate to share this between the two libraries.
use alloc::string::String;

#[derive(Debug, PartialEq, Eq)]
pub enum UartError {
    NotEnoughBytes,
    TooManyBytes,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExampleProtocol {
    Number(usize),
    Text(String),
}

/// For docs: see runner/src/main.rs
impl ExampleProtocol {
    pub fn to_uart(&self, dest: &mut [u8]) -> Result<usize, UartError> {
        match self {
            ExampleProtocol::Number(n) => {
                let n = *n as u32;
                let bytes = n.to_le_bytes();
                dest[0] = 0;
                dest[1..=4].copy_from_slice(&bytes);
                Ok(5)
            }
            ExampleProtocol::Text(s) => {
                let bytes = s.as_bytes();
                if bytes.len() > dest.len() - 1 {
                    return Err(UartError::TooManyBytes);
                }
                dest[0] = 1;
                let len = (bytes.len() as u32).to_le_bytes();
                dest[1..=4].copy_from_slice(&len);
                dest[5..5 + bytes.len()].copy_from_slice(bytes);

                Ok(bytes.len() + 5)
            }
        }
    }

    pub fn from_uart(data: &[u8]) -> Result<(Self, usize), UartError> {
        if data.is_empty() {
            return Err(UartError::NotEnoughBytes);
        }
        match data[0] {
            0 => {
                if data.len() < 5 {
                    return Err(UartError::NotEnoughBytes);
                }
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&data[1..5]);
                let n = u32::from_le_bytes(bytes);
                Ok((ExampleProtocol::Number(n as usize), 5))
            }
            1 => {
                if data.len() < 5 {
                    return Err(UartError::NotEnoughBytes);
                }
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&data[1..5]);
                let len = u32::from_le_bytes(bytes) as usize;
                if data.len() < 5 + len {
                    return Err(UartError::NotEnoughBytes);
                }
                let s = String::from_utf8(data[5..5 + len].to_vec()).unwrap();
                Ok((ExampleProtocol::Text(s), 5 + len))
            }
            _ => Err(UartError::NotEnoughBytes),
        }
    }
}
