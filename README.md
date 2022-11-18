This is assignment 1, about embedded concurrency. You can find a description of this assignment
on [the website](https://software-fundamentals.pages.ewi.tudelft.nl/software-systems/website/part-1/assignments/concurrency.html)

# Assignment 1 for Software Systems

## Baseline requirements
- [x] Be able to search a file or directory for all occurrences of a regex pattern.
- [x] Print output while it is searching. (It shouldn't buffer the results and print them at the end)
- [x] Print the results only using the provided `Display` implementation. Results should be put into `GrepResult` structs and printed. This will make sure their formatting is consistent.
- [x] Make sure the `search_ctr` numbers during printing are in increasing order.
- [x] Make sure the output is well-formed.
- [x] Be concurrent, that is, should use multiple threads.
- [x] Be thread safe. Any pattern that may work for a limited input, or which depends on being lucky with how the scheduler schedules your threads is considered incorrect.
- [x] Use at least one `Mutex` or `RwLock` in a non-trivial way (i.e. you don't simply construct a `Mutex` and never use it).
- [x] Use at least one non-trivial channel.
- [x] Not use any other libraries besides regex and clap without explicit permissions from a TA.
- [x] Not use any unsafe code.
- [x] Compile on stable rust.
- [x] Use threads, and not rust's support for async and asynchronous programming.
- [x] Dynamically determine how many threads it should use based on the amount of cores the system has. You must not spawn many times more threads than your system has cores. On large folders, you should not spawn millions of threads.

## Extra requirements
### If your application is faster than grep: 1.5 points
- [x] This is easier to achieve than you may expect, since grep is single-threaded.
- [x] We will test this by running both your tool and grep, and asking them to find the word torvalds in the linux source code. The tests will be run on the same computer, with multiple cores available.
### Good error handling: 1.0 points
- [x] Your application needs to provide a user-friendly output when an unexpected situation occurs.
- [ ] For example, the user may provide an invalid regex, the program may encounter files that it does not have permission to read, or a file is deleted while the program is executing. This is not a full list.
- [ ] The program should not contain any unwraps or expects that may fail. Provide user-friendly outputs.
### Implement path filtering: 1.0 points
- [ ] The program should only return results in files for which the file path matches the specified regex.
- [ ] In order to do this, you will need to use the `filter` parameter in the `Args` struct.
- [ ] For example, the command `cargo run -- -f ".*/file[12].txt" banana ./examples/example2` should only return the result in file 1, not in file 3:
### Analyse the performance of your implementation: 1.0 points
- [x] Submit a document of at most two pages, in which you present a performance analysis of your program.
- [x] Include a plot that shows the performance of the program over the number of threads that it was given.
- [x] Explain how you measured the performance, and how you prevented random fluctuations from affecting the result.
- [x] Include relevant information, such as the number of cores of the machine this was executed on and the dataset that was used.
- [x] Include the same command executed with `grep` as a baseline.
### Support for binary files: 1.0 points
- [x] Not all files in the directory may contain utf-8 text. The program should work correctly in these scenarios.
- [x] Note that a String can only contain utf-8 text, so you will have to use a Vec<u8> instead.
- [x] You need to use the bytes module of the regex crate for this. See https://docs.rs/regex/latest/regex/bytes/index.html for more.
- [x] When printing, non-utf8 bytes should be replaced by the unicode replacement character. The String::from_utf8_lossy function does this. The Display implementation of GrepResult already uses this, so you don't need to change this.
- [x] In the example3 directory, you can find an example of a file with non-utf8 bytes. Note that your text editor may not enjoy these bytes. cargo run banana ./examples/example3/ should match the file. The expected result is:
`>>> (#0) ../template/examples/example3/test.bin`
`�banana�`
` ^^^^^^`
