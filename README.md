# Assignment 3 SS
**Note that there is a manual for the program in this README, please read it!**

## Checklist
UART Driver:
- You must provide a safe abstraction for global mutable state (Mutex)
- [x] You need to somehow create a way of preventing two threads executing at once, except there are no threads in the system you are implementing the driver for, what can cause code to run twice
- [x] This may need unsafe code, remember to add an explanation on why your implementation is actually safe.
- You must provide a safe abstraction for late initialization of global variables (OnceCell)
- [x] You can implement this using atomics, but with the way that you are using it (wrapped in the mutex from above) you don't have to.
- [x] This may need unsafe code, remember to add an explanation on why your implementation is actually safe.
- You must provide buffering such that long messages can be sent and received.
- [x] The buffer must be fixed-size, and must be sized appropriately for the messages that are sent over them.
- [x] If the buffer fills up, the program may not crash.
- [x] In the buffer.rs there are more hints on how to implement this buffer.

Communication:
- While the emulator runs, notes should be saved by the system
- [x] You should be able to add new notes by sending a message. The system should return an ID associated with the newly created note
- [x] Using the ID (by sending messages to the system):
    Notes can be requested to read (returning the note)
    Notes can be deleted
- [x] You should use checksums to verify the integrity of your messages
- [x] If messages are dropped, or corrupted, you should make sure that your system can continue reading future messages. Test this.
- [x] You must use serde and postcard

Interface:
- [x] You should add a simple command line interface, something which allows you to read a command from stdin and writing the result using println!() is sufficient. This interface should make it easy to send and receive notes from the emulator.
    You can make it as fancy as you want, but remember to ask a TA if you want to use any libraries not already allowed.

General: 
- [x] Every use of unsafe code must be sound (i.e. we will assess your use of unsafe code and check that what you are doing is sound).
- [x] Every use of unsafe code should have a comment layout the entire reasoning required to understand why that block of unsafe code is indeed sound.

## Program manual
### Features
This is a simple program that saves, reads and deletes notes.
The maximum number of notes that it could save is **20**, and the length for each note is **20** bytes.
Note that the program currently only supports saving notes without whitespace included in the note, meaning:
- "**-a Hi, I am saving this note**" is not allow, it will be rejected by the program.
- "**-a Hi_I_am_saving**" is allowed, the program will allow it.

Errors that may occur in the program is handled in a user-friendly way, specific messages about the errors will be displayed.
A user-friendly CLI is also made.

***Hope you would have fun with our notes-saving program.***

### Commands
- Run ```cargo run/ cargo run --release``` to start the program.```

Message below will be displayed:

```
---------------------------------------------------------------------
|                                                                    |
|   Dear Runner! Please provide your input! Enter -h/help for help   |
|                                                                    |
---------------------------------------------------------------------
```

- Run ```help``` or ```-h``` for help message displayed below:
```
Format of command: [command]  [message] or [ID]\n
        Commands:   -h/help:        Display help message\n
                    -a/add:         Add a note\n
                    -d/delete:      Delete a note\n
                    -r/read:        Read a note\n
                    -t1/test1:      Test the program on data loss\n
                    -t2/test2:      Test the program on corrupted data\n
                    exit:           Exit program\n

        Message:    Whatever you want to store in the note\n

        ID:         The ID you get for each note you store\n
```

- Run ```exit``` to exit the program:
```
---------------------------------------------------------------------
|                                                                    |
|             Program ended, see you next time Runner!               |
|                                                                    |
---------------------------------------------------------------------
```

- Run ```-a [message]``` or ```add [message]``` to add messages:
This is a demonstration below:
(Note again that the maximum length of the message is 20 bytes)
```
-a Hi-Johnnathon-Group-59
Your note has more than 20 bytes, too long!
-a Hi-Johnnathon
///////ID: 1 ..........Message from UART      : Done
add Hi-Johnnathon
///////ID: 2 ..........Message from UART      : Done
```

- Run ```-r [ID]``` or ```read [ID]``` to read messages:
This is a demonstration below:
```
-r 1
///////ID: 1 ..........Note is                : Hi-Johnnathon
read 1
///////ID: 1 ..........Note is                : Hi-Johnnathon
```

- Run ```-d [ID]``` or ```delete [ID]``` to delete messages:
This is a demonstration below:
```
-d 1
///////ID: 1 ..........Message from UART      : Deleted
delete 1
///////ID: 0 ..........Error message from UART: No related note 
-r 1
///////ID: 0 ..........Error message from UART: Doesn't exist
read 1
///////ID: 0 ..........Error message from UART: Doesn't exist
```

- Run ```-t1 [random]``` or ```test1 [random]``` to run the test on data loss:
Note that due to our program lack a bit of user-friendliness, please add some random things behind this command to make it run.
This is a demonstration below:
```
-t1 whatever 
test data loss
///////ID: 0 ..........Error message from UART: Data loss!!
```

- Run ```-t2 [random]``` or ```test2 [random]``` to run the test on corrupted data:
Note that due to our program lack a bit of user-friendliness, please add some random things behind this command to make it run.
This is a demonstration below:
```
-t2 whatever
test data doesn't follow the protocol
///////ID: 0 ..........Error message from UART: Message wrong!!
```

- Error Handling
As shown in the demonstrations above, errors are handled in our program, they will print user-friendly messages as well.
The erorrs we are handling are listed below:
```
1. Invalid input from the user
2. Command from the user in wrong format
3. Message recieved / sent corrupted (Does not match our protocol)
4. Message recieved / sent experience data loss
5. Saving message too long
6. Maximum capacity of saved notes reached
7. Checksum is wrong
8. Reading a note with ID that does not exist / out of scope
9. Deleting a note with ID that does not exist / out of scope
```

## Protocol Design
We change the way of sending or receiving data. Rather than using the method of "hand shaking", we use a data pack which contains the following part:
```
pub struct NewProtocol {
    start_num: [u8; 2],//0x69, 0x69
    function: u8,
    id: u8,
    data_len: u8,
    data: [u8; 20],
    check_sum: [u8; 4],
}
```
- Start_num (header) is our magic num stating that this is the start of the message as well as a signature to verify whether valid or not.
- Function is a single byte stating the functionality, ``ADD (01)``, ```READ (02)``` or ```DELETE (03)```.
- id is the ID of every note. When id is 0, this means the message is an error message
- data_len is the number of useful bytes in the data array. Except the useful bytes, other bytes are all 0.
- data contains the message users input.
- check_sum is the sum of all bytes above. It has 4 bytes, if the sum is more than 5 bytes, the least significant 4 bytes is taken as the checksum. And also, in the check_sum array, the first byte of it is in fact the lowest byte of the real sum. Example: real sum: 0x69 0x78 0x54 0x12, check_sum: [0x12, 0x54, 0x78, 0x69];


## Test Explaination

### Test 1
As mentioned before, two tests are implemented. Test 1 shows when the data loss happens, the system can keep running. In our codes, runner or uart driver turn into 'data_recv' state since they start to get bytes. And loops are used for runner/server to keep listening. There is also a value called 'wait_cycle' which means how many cycles the runner/server has already waited before they get next bytes. It is added with one in every loop, and reset to zero if they receive new bytes. When the cycle is over 100, it is considered as time out. And there will be error message about data loss.

***With our design, all data packs have same size after serialized which is 29 bytes.*** 

So in the test a message with only 28 bytes is sent to prove the system can keep running after the data loss.

### Test 2
This test can show when system receive the message which is not following the protocol, it still can run. An message with 29 bytes but doesn't follow the protocol is sent to server. The server will check the header and checksum of the data pack and return an error message if one of them is wrong.


## Comment for unsafe code
It is confident that uses of unsafe code are sound.
For the assumptions of how unsafe code could go wrong, and why they are safe to use. Please refer to our source code in detail.
