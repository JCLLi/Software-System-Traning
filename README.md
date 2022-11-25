# How to measure temperature without a temperature sensor and read it via UART
This exercise has two main parts: implement a small UART driver for the NRF51822 
and create a fake temperature 

## Running the project not on linux
Due to the fact that we use both arm and x86 code we had to force
the runner to use the `x86_64-unknown-linux-gnu` target triple. 
If you manage to get the project to work on macOS or Windows you need
to change this to the native target triple, which is probably one of 
the following:
* `x86_64-pc-windows-gnu` for Windows
* `x86_64-pc-windows-msvc` for windows (using Microsoft's C compiler)
* `x86_64-apple-darwin` for non-M1 macs
* `aarch64-apple-darwin` for M1 macs

Note that this project is largely untested on anything but Linux. Chances are it will not work
as expected on other operating systems. It may work on WSL2, though there you should be able to
keep the line as it is.
