//! This file is mostly the same as runner/src/main.rs. For more complex protocols, you may
//! want to factor this out into its own crate to share this between the two libraries.
use alloc::string::String;
use core::ops::Deref;
use cortex_m_semihosting::hprintln;

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
    pub data_len: u8,
    pub data: [u8; 20],
    pub check_sum: [u8; 4],
}

pub enum Function{
    ADD,
    READ,
    DELETE,
}

impl NewProtocol{
    pub fn new_to_uart(dest: &mut [u8], function: Function, note: [u8; 20], id: u8, data_len: u8){
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
                let serial: Vec<u8, 29> = to_vec(&output).unwrap();
                for i in 0..serial.len(){
                    dest[i] = serial[i];
                }
            },
            Function::READ => {
                let mut sum = 0x69 + 0x69 + 0x02 + id as u32 + data_len as u32;
                for i in 0..20{
                    sum += note[i] as u32;
                }
                let mut check_sum:[u8; 4] = [0, 0, 0, 0];
                for i in 0..4{
                    check_sum[i] = (sum & 0xff) as u8;
                    sum = sum >> 8;
                }
                let output = NewProtocol{
                    start_num: [0x69, 0x69],
                    function: 0x02,
                    id,
                    data_len,
                    data: note,
                    check_sum,
                };
                let serial: Vec<u8, 29> = to_vec(&output).unwrap();
                for i in 0..serial.len(){
                    dest[i] = serial[i];
                }
            },
            Function::DELETE => {
                let mut sum = 0x69 + 0x69 + 0x03 + id as u32 + data_len as u32;
                for i in 0..20{
                    sum += note[i] as u32;
                }
                let mut check_sum:[u8; 4] = [0, 0, 0, 0];
                for i in 0..4{
                    check_sum[i] = (sum & 0xff) as u8;
                    sum = sum >> 8;
                }
                let output = NewProtocol{
                    start_num: [0x69, 0x69],
                    function: 0x03,
                    id,
                    data_len,
                    data: note,
                    check_sum,
                };
                let serial: Vec<u8, 29> = to_vec(&output).unwrap();
                for i in 0..serial.len(){
                    dest[i] = serial[i];
                }
            },
        }

    }
    pub fn new_from_uart(input: &Vec<u8, 29>) -> Result<NewProtocol, UartError>{
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
