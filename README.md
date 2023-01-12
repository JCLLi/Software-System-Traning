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
- [x] For example, the user may provide an invalid regex, the program may encounter files that it does not have permission to read, or a file is deleted while the program is executing. This is not a full list.
- [x] The program should not contain any unwraps or expects that may fail. Provide user-friendly outputs.
### Implement path filtering: 1.0 points
- [x] The program should only return results in files for which the file path matches the specified regex.
- [x] In order to do this, you will need to use the `filter` parameter in the `Args` struct.
- [x] For example, the command `cargo run -- -f ".*/file[12].txt" banana ./examples/example2` should only return the result in file 1, not in file 3:
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

## Discussion
The analysis is done with a wide range of threads launched in the program. However, by default, the number of threads launched running the program differs among machines. It is equal to the maximum number of threads the machine could handle. In this case, 8 threads for MacBook and 16 threads for Dell.

Besides the baseline, most of the extra requirements are met in our program. However the `Good error handling` is not certain whether it is fully realized, due to lack of verification.
For most functions implemented, error handling with user friendly messages are included.

Another issue regarding the performance of the program in respect to the number of threads launched in the program requires further analysis. The peak performance of the program reaches its peak performance on both machines with different operating systems when 128 and 64 threads are launched respectively. However, according to the CPU hardware parameters, the CPU in two machines should only be able to handle 16 threads and 8 threads at the same time. The answer to this issue remains unknown and requires further analysis.

The only two concurrency primitives used in the program are a counter of type Mutex which counts the numbers of files that match the regex, and a channel sending `GrepResult` between two threads. The counter of type Mutex is necessary because it guarantees the safety of the value in counter. Due to the fact that Mutex could only be accessed when it is locked, and block threads waiting for the lock to become available. Hence, only one thread is adding to the counter each time. The second concurrency primitive is a channel which sends `GrepResult` found in every file to another thread for printing out the result. This is due to the requirement of the assignment - `Use at least one non-trivial channel.`. In our case, two different type of threads are created. One only consists of a thread which loops and prints out the result in a provided format once it receives `GrepResult` via the channel. The other one consists of several threads which reads a file respectively and sends the `GrepResult` via the channel once the single file is done reading and matches are found.

## Conclusion
Overall, the program is faster than the baseline throughout all our tests (when number of threads launched are the maximum that the machine could handle). The speedup is 136.03% on Dell (with 16 threads launched in the program) and 642.96% on MacBook (with 8 threads launched in the program). Most of the requirements are fully realized. Though answers to certain issues remain unknown, they are discovered and reasonable guesses for the cause are given. 
