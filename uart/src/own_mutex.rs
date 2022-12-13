use core::cell::UnsafeCell;
use cortex_m::interrupt::free;
use nrf51_pac::interrupt;
use crate::once_cell::OnceCell;
use crate::UART;
use crate::uart_driver::UartDriver;

/// In this module you'll have to implement a custom mutex.
/// Sadly the standard library is not available on most embedded systems,
/// and thus you must create your own Mutex to supplement it.
///
/// The mutex should not be too advanced, just making sure it
/// can stop interrupts is enough.
///

/// The mutex.
///
/// The contents of the mutex should be accessible from an immutable reference.
/// See the defined functions in the impl block to see what you need to implement.
pub struct OwnMutex<T> {
    uart_driver: UnsafeCell<T>,
}

/// SAFETY: TODO: write here an explanation why this is safe
/// sync traits enables OwnMutex to be shared between threads, which is unsafe with static variables.
/// However, firstly, the chip is single core, hence it would not create multiple threads.
/// Second, when an interrupt occurs the access to OwnMutex is locked, hence it would not raise data race issues.
/// When interupts occur, the process is also non-preemptive, hence the data is safe to access. More importantly,
/// OwnMutex is locked before access.
unsafe impl<T> Sync for OwnMutex<T> {}

impl<T> OwnMutex<T> {
    /// Creates a new mutex with the given content.
    pub const fn new(t: T) -> Self {
        return Self {
            uart_driver: UnsafeCell::new(t),
        };
    }

    /// this takes a reference to self, and a function that takes a mutable reference to the inner type.
    /// You need to make sure interrupts can't happen.
    pub fn modify<U>(&self, mut f: impl FnMut(&mut T) -> U) -> U {
        // cortex_m::interrupt::disable();
        // let r = f(unsafe {self.uart_driver.get().as_mut().unwrap()});
        // unsafe {cortex_m::interrupt::enable();}
        // r

        /// The use of unsafe code is sound because all interrupts are disabled before accessing uart_driver
        /// The goal of making uart_driver wrapped by mutex is to prevent its value from changing while other interrupts occurs.
        /// Hence, the use of the unsafe code is sound.
        free(|_| f(unsafe { self.uart_driver.get().as_mut() }.unwrap()))
    }
}
