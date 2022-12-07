use std::{
    env::args,
    io::{Read, Write, self},
};

use tudelft_arm_qemu_runner as runner;

/// This is the "runner" side entry point. This will start the emulator, and then
/// communicate with the emulator over uart (once you implement your driver).
/// Later on, for the "interface" and "communication" requirements, you should
/// modify the runner to work with your protocol and your interface.
fn main() {
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
        let commands = user_input.as_str().split("-").collect::<Vec<&str>>();
        if commands.len() != 3 && !(user_input.as_str() == "help\n" || user_input.as_str() == "-h\n" || user_input.as_str() == "exit\n") {
            println!("Please provide the command in the right format! Enter -h for help!");
            continue;
        }

        match user_input.as_str() {
            "help\n" => {
                println!("
                          Format of command: [command]-[message]-[ID]\n
                            Commands:   -h/help:        Display help message\n
                                        -a/add:         Add a note\n
                                        -d/delete:      Delete a note\n
                                        -r/read:        Read a note\n

                            Message:    Whatever you want to store in the note\n

                            ID:         The ID you get for each note you store\n
                            ");
                continue;
            }
             "-h\n" => {
                println!("
                          Format of command: [command]-[message]-[ID]\n
                            Commands:   -h/help:        Display help message\n
                                        -a/add:         Add a note\n
                                        -d/delete:      Delete a note\n
                                        -r/read:        Read a note\n

                            Message:    Whatever you want to store in the note\n

                            ID:         The ID you get for each note you store\n
                            ");
                continue;
            },
            "exit\n" => {
                panic!("
                ---------------------------------------------------------------------\n
                |                                                                    |\n
                |             Program ended, see you next time Runner!               |\n
                |                                                                    |\n
                ---------------------------------------------------------------------\n
                ");
            },
            _ => {
                // println!("Please provide a valid command");
                fault = true;
                continue;
            },
        }

        let to_send = [
            ExampleProtocol::Number(1),
            ExampleProtocol::Text("Hello".to_string()),
        ];
        let to_receive = [
            ExampleProtocol::Number(2),
            ExampleProtocol::Text("World".to_string()),
        ];
        let mut v = vec![];
        for (s, r) in to_send.into_iter().zip(to_receive) {
            println!("Now Writing: {:?}", s);
            let mut buf = [0; 1024];
            let size = s.to_uart(&mut buf).unwrap();
            runner.write_all(&buf[..size]).unwrap();
            let w = loop {
                let mut buf = [0; 255];
                let r = runner.read(&mut buf).unwrap();
                v.write_all(&buf[..r]).unwrap();
                let res = ExampleProtocol::from_uart(&v);
                if let Ok((res, read)) = res {
                    v = v.into_iter().skip(read).collect();
                    break res;
                }
            };
            if w != r {
                println!("Input {:?} failed: got {:?}, expected {:?}", s, w, r);
            } else {
                println!("Input {:?} succeeded", s);
            }
        }
    }
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

const MAGIC_NUMBER: u32 = 0x6969;
pub struct NewProtocol {
    start_num: u32,
    address: u8,
    function: u8,
    ID: u8,
    data_len:u8,
    data: String,
    valid: u8,
    end_num: u32,
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
