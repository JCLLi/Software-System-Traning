use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use crate::panic;
use crate::uart_driver::UartDriver;

/// A cell that can be written to only once.
pub struct OnceCell<T> {
    // this is a placeholder, it's a special type that simply holds a reference to
    // the type parameter. It's used to make the compiler happy. You'll have to replace
    // this with something else.
    uart_driver: Option<T>,
}

impl<T> OnceCell<T> {
    /// Creates a new empty cell.
    /// It's const to allow you to use it in static contexts. (like in global variables, hint hint)
    pub const fn new() -> Self {
        Self { uart_driver: None }
    }

    /// Sometimes you want to create a cell that is already initialized.
    pub fn new_with(_v: T) -> Self {
        todo!()
    }

    /// This can initialize an empty cell.
    /// If the cell is already initialized, it should panic.
    pub fn initialize(&mut self, v: UartDriver) {
        if self.uart_driver.is_some() { panic!("UART driver can only be initialized once") }
        self.uart_driver = Some(v);
    }
}

/// The Deref trait is used to allow easy access to the inner type of the OnceCell.
/// Make sure to implement it correctly!
impl<T> Deref for OnceCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> DerefMut for OnceCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}
