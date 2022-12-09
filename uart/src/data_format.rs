//! This file is mostly the same as runner/src/main.rs. For more complex protocols, you may
//! want to factor this out into its own crate to share this between the two libraries.
use alloc::string::String;
use core::ops::Deref;

use serde::{Serialize, Deserialize};
use postcard::{from_bytes, to_vec};
use heapless::Vec;

#[derive(Debug, PartialEq, Eq)]
pub enum UartError {
    NotEnoughBytes,
    TooManyBytes,
    MessageWrong
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct NewProtocol {
    pub start_num: [u8; 2],
    pub function: u8,
    pub id: u8,
    pub data_len: u16,
    pub data: [u8; 20],
    pub check_sum: [u8; 4],
}

pub enum Function{
    ADD,
    READ,
    DELETE,
}

impl NewProtocol{
    pub fn new_to_uart(dest: &mut [u8], function: Function, note: [u8; 20], id: u8, data_len: u16) -> Result<usize, UartError> {
        match function {
            Function::ADD=> {
                let mut sum: u32 = 0x00;
                for i in 0..data_len{
                    sum += note[i as usize] as u32;
                }
                sum = sum + 0x69 + 0x69 + 0x01 + id as u32 + data_len as u32;
                let mut check_sum:[u8; 4] = [0, 0, 0, 0];
                for i in 0..4{
                    check_sum[i] = (sum & 0xff) as u8;
                    sum = sum >> 8;
                }
                let output = NewProtocol{
                    start_num: [0x69, 0x69],
                    function: 0x01,
                    id,
                    data_len,
                    data: note,
                    check_sum,
                };
                let serial: Vec<u8, 1024> = to_vec(&output).unwrap();
                for i in 0..serial.len(){
                    dest[i] = serial[i];
                }

                Ok(serial.len())
            },
            Function::READ => {
                let mut data: [u8; 20] = [0; 20];
                let mut sum = 0x69 + 0x69 + 0x02 + id as u32;
                let mut check_sum:[u8; 4] = [0, 0, 0, 0];
                for i in 0..4{

                    check_sum[i] = (sum & 0xff) as u8;
                    sum = sum >> 8;
                }
                let output = NewProtocol{
                    start_num: [0x69, 0x69],
                    function: 0x02,
                    id,
                    data_len: 0,
                    data,
                    check_sum,
                };
                let serial: Vec<u8, 1024> = to_vec(&output).unwrap();
                for i in 0..serial.len(){
                    dest[i] = serial[i];
                }
                Ok(serial.len())
            },
            Function::DELETE => {
                let mut data: [u8; 20] = [0; 20];
                let mut sum = 0x69 + 0x69 + 0x03 + id as u32;
                let mut check_sum:[u8; 4] = [0, 0, 0, 0];
                for i in 0..4{

                    check_sum[i] = (sum & 0xff) as u8;
                    sum = sum >> 8;
                }
                let output = NewProtocol{
                    start_num: [0x69, 0x69],
                    function: 0x03,
                    id,
                    data_len: 0,
                    data,
                    check_sum,
                };
                let serial: Vec<u8, 1024> = to_vec(&output).unwrap();
                for i in 0..serial.len(){
                    dest[i] = serial[i];
                }
                Ok(serial.len())
            },
        }

    }
    pub fn new_from_uart(input: &Vec<u8, 1024>) -> Result<NewProtocol, UartError>{
        if input.len() < 29 { return Err(UartError::NotEnoughBytes); }
        let input_data: NewProtocol = from_bytes(input.deref()).unwrap();
        if input_data.start_num[0] != 0x69 || input_data.start_num[1] != 0x69{ return Err(UartError::MessageWrong); }

        let mut sum: u32 = 0;
        for i in 0..input_data.data_len{
            sum = sum + input_data.data[i as usize] as u32;
        }
        sum = sum + input_data.start_num[0] as u32 + input_data.start_num[1] as u32 + input_data.function as u32 + input_data.id as u32
            + input_data.data_len as u32;
        let mut sum_check: [u8; 4] = [0, 0, 0, 0];
        for i in 0..4{

            sum_check[i] = (sum & 0xff) as u8;
            sum = sum >> 8;
        }
        if sum_check != input_data.check_sum { return Err(UartError::MessageWrong); }
        Ok(input_data)
    }
}

///Old
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///

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
