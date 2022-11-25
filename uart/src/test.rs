use core::panic::PanicInfo;
/// This module is where the test runner lives.
/// It is responsible for running the tests defined in the uart crate
use cortex_m_semihosting::debug::{exit, EXIT_SUCCESS};
use cortex_m_semihosting::hprintln;

/// This function actually runs the tests.
/// We abuse the `no_mangle` attribute to allow the runner
/// to check if we are running tests, or just the regular code.
#[no_mangle]
pub fn test_runner(tests: &[&dyn Fn()]) {
    hprintln!("--- RUNNING {} TESTS ---", tests.len());
    for test in tests {
        test();
    }

    hprintln!("--- ALL TESTS SUCCESSFUL ---");
    exit(EXIT_SUCCESS);
}

#[test_case]
fn test() {
    assert_eq!(3 + 3, 6);
}

#[inline(never)]
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use cortex_m_semihosting::debug::EXIT_FAILURE;

    hprintln!("RUST PANIC: {}", info);

    exit(EXIT_FAILURE);

    loop {}
}
