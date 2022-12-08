use std::{
    env::args,
    io::{Read, Write, self},
};

use tudelft_arm_qemu_runner as runner;

use core::ops::Deref;
use serde::{Serialize, Deserialize};
use postcard::{from_bytes, to_vec};
use heapless::Vec;
use crate::Function::{ADD, DELETE, READ};

/// This is the "runner" side entry point. This will start the emulator, and then
/// communicate with the emulator over uart (once you implement your driver).
/// Later on, for the "interface" and "communication" requirements, you should
/// modify the runner to work with your protocol and your interface.
fn main() {
    // let mut a:u32 = 0xffffffff;
    // for i in 0..4 {
    //     a = a >> (i * 2);
    //     println!("{:#0x}", a);
    // }
    let mut fault = false;
    let mut already_start = false;
    loop {
        if already_start == false {
            println!("
                  ---------------------------------------------------------------------\n
                  |                                                                    |\n
                  |   Dear Runner! Please provide your input! Enter -h/help for help   |\n
                  |                                                                    |\n
                  ---------------------------------------------------------------------\n
                  "); // If random stuff is input, display the instruction to call help
        }
        already_start = true;
        if fault == true {
            println!("Dear Runner! Please provide a valid input! Enter -h for help!");
        }
        fault = false;

        // The code below basically starts the qemu
        let binary = args().nth(1).unwrap();
        // println!("{} is inside the binary", binary);
        let mut runner = runner::Runner::new(&binary).unwrap();
        if runner.testing {
            runner.wait_for_tests();
            return;
        }

        // Get the things user iput from terminal -> user_input
        let mut user_input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut user_input).expect("What you have input is not valid");

        // Now parse the user_input
        let commands = user_input.as_str().split(" ").collect::<std::vec::Vec<&str>>();
        if commands.len() != 2 && !(user_input.as_str() == "help\n" || user_input.as_str() == "-h\n" || user_input.as_str() == "exit\n") {
            println!("Please provide the command in the right format! Enter -h for help!");
            continue;
        }
        let mut buf = [0; 1024];
        match user_input.as_str() {
            "help\n" | "-h\n" => {
                println!("
                          Format of command: [command]  [message] or [ID]\n
                            Commands:   -h/help:        Display help message\n
                                        -a/add:         Add a note\n
                                        -d/delete:      Delete a note\n
                                        -r/read:        Read a note\n
                                        exit:           Exit program\n

                            Message:    Whatever you want to store in the note\n

                            ID:         The ID you get for each note you store\n
                            ");
                continue;
            }
            "exit\n" => {
                break;
            },
            _ => {
                match commands[0] {
                    "-a" | "add" => {
                        NewProtocol::new_to_uart(&mut buf, ADD, String::from(commands[1]), 0);
                    },
                    "-d" | "delete" => {
                        match commands[1].trim().parse::<u8>() {
                            Ok(id) => {
                                NewProtocol::new_to_uart(&mut buf, DELETE, String::from(""), id);
                            },
                            Err(_) => {
                                fault = true;
                                continue;
                            },
                        }
                    },
                    "-r" | "read" => {
                        match commands[1].trim().parse::<u8>() {
                            Ok(id) => {
                                NewProtocol::new_to_uart(&mut buf, READ, String::from(""), id);
                            },
                            Err(_) => {
                                fault = true;
                                continue;
                            },
                        }
                    },
                    _ => {
                        println!("Please provide a valid command. Enter -h/help for help!");
                    },
                }
            },
        }
        // let mut input: Vec<u8, 1024> = heapless::Vec::new();
        // for i in 0..1024{
        //     input.push(buf[i]);
        // }
        // let a: NewProtocol = from_bytes(input.deref()).unwrap();
        // println!("{:#0x}", a.start_num[0]);
        // println!("{:#0x}", a.start_num[1]);
        // println!("{:#0x}", a.function);
        // println!("{:#0x}", a.id);
        // println!("{:#0x}", a.data_len);
        // println!("{}", a.data);
        // println!("{:#0x}", a.check_sum[0]);
        // println!("{:#0x}", a.check_sum[1]);
        // println!("{:#0x}", a.check_sum[2]);
        // println!("{:#0x}", a.check_sum[3]);

        // let my_message = NewProtocol {
        //     start_num: 0x6969,
        //     function: 0x01,
        //     id: 0x01,
        //     data_len: 0x05,
        //     data: "FUCKU",
        //     check_sum: 0x01,
        // };
        // let to_send: NewProtocolCmd;
        // let to_receive: NewProtocolCmd;
        // let mut buf = [0; 1024];
        // to_send.new_to_uart(&mut buf, &my_message);
        //
        // let to_send = [
        //     ExampleProtocol::Number(1),
        //     ExampleProtocol::Text("Hello".to_string()),
        // ];
        // let to_receive = [
        //     ExampleProtocol::Number(2),
        //     ExampleProtocol::Text("World".to_string()),
        // ];
        // let mut v = vec![];
        // for (s, r) in to_send.into_iter().zip(to_receive) {
        //     println!("Now Writing: {:?}", s);
        //     let mut buf = [0; 1024];
        //     let size = s.to_uart(&mut buf).unwrap();
        //     runner.write_all(&buf[..size]).unwrap();
        //     let w = loop {
        //         let mut buf = [0; 255];
        //         let r = runner.read(&mut buf).unwrap();
        //         v.write_all(&buf[..r]).unwrap();
        //         let res = ExampleProtocol::from_uart(&v);
        //         if let Ok((res, read)) = res {
        //             v = v.into_iter().skip(read).collect();
        //             break res;
        //         }
        //     };
        //     if w != r {
        //         println!("Input {:?} failed: got {:?}, expected {:?}", s, w, r);
        //     } else {
        //         println!("Input {:?} succeeded", s);
        //     }
        // }
    }
    println!("
                ---------------------------------------------------------------------\n
                |                                                                    |\n
                |             Program ended, see you next time Runner!               |\n
                |                                                                    |\n
                ---------------------------------------------------------------------\n
                ");
}

/// The different kinds of errors that can occur when working with UART
#[derive(Debug, PartialEq, Eq)]
pub enum UartError {
    NotEnoughBytes,
    TooManyBytes,
}

/// This is an example protocol we already made for you. It's very simple. However,
/// you can use it to test your uart driver *before* you implement your own communications protocol.
///
/// Note that here all serialization is done manually. For the assignment you should use postacrd,
/// which automates complicated deserializing from and to bytes.
/// 
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct NewProtocol<'a> {
    start_num: [u8; 2],
    function: u8,
    id: u8,
    data_len: u16,
    data: &'a str,
    check_sum: [u8; 4],
}

#[derive(Debug, PartialEq, Eq)]
pub enum NewProtocolCmd<'a>{
    ADD(NewProtocol<'a>),
    READ(NewProtocol<'a>),
    DELETE(NewProtocol<'a>),
}

pub enum Function{
    ADD,
    READ,
    DELETE,
}

impl<'a> NewProtocol<'a> {
    pub fn new_to_uart(dest: &mut [u8], function: Function, note: String, id: u8) -> Result<usize, UartError> {
        match function {
            Function::ADD=> {
                let note_str = note.as_str();
                let note_byte = note_str.as_bytes();

                if note_byte.len() > 1014 {
                    return Err(UartError::TooManyBytes);
                }

                let mut sum: u32 = 0x00;
                for i in 0..note_byte.len(){
                    sum += note_byte[i as usize] as u32;
                }
                sum = sum + 0x69 + 0x69 + 0x01 + id as u32 + note.len() as u32;
                let mut check_sum:[u8; 4] = [0, 0, 0, 0];
                for i in 0..4{
                    sum = sum >> i * 4;
                    check_sum[i] = (sum & 0xff) as u8;
                }
                let output = NewProtocol{
                    start_num: [0x69, 0x69],
                    function: 0x01,
                    id,
                    data_len: note_str.len() as u16,
                    data: note_str,
                    check_sum,
                };
                let serial: Vec<u8, 1024> = to_vec(&output).unwrap();
                for i in 0..serial.len(){
                    dest[i] = serial[i];
                }

                Ok(serial.len())
            },
            Function::READ => {
                let mut sum = 0x69 + 0x69 + 0x02 + id as u32;
                let mut check_sum:[u8; 4] = [0, 0, 0, 0];
                for i in 0..4{
                    sum = sum >> i * 4;
                    check_sum[i] = (sum & 0xff) as u8;

                    println!("{:#0x}", check_sum[i]);
                }
                let output = NewProtocol{
                    start_num: [0x69, 0x69],
                    function: 0x02,
                    id,
                    data_len: 0,
                    data: "",
                    check_sum,
                };
                let serial: Vec<u8, 1024> = to_vec(&output).unwrap();
                for i in 0..serial.len(){
                    dest[i] = serial[i];
                }
                Ok(serial.len())
            },
            Function::DELETE => {
                let mut sum = 0x69 + 0x69 + 0x03 + id as u32;
                let mut check_sum:[u8; 4] = [0, 0, 0, 0];
                for i in 0..4{
                    sum = sum >> i * 4;
                    check_sum[i] = (sum & 0xff) as u8;

                    println!("{:#0x}", check_sum[i]);
                }
                let output = NewProtocol{
                    start_num: [0x69, 0x69],
                    function: 0x03,
                    id,
                    data_len: 0,
                    data: "",
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
}




#[derive(Debug, PartialEq, Eq)]
pub enum ExampleProtocol {
    Number(usize),
    Text(String),
}

impl ExampleProtocol {
    /// convert a message in this protocol (self) to bytes (dest) (Serialization)
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

    /// Convert bytes (data) into a message (Self in return). (Deserialization)
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
