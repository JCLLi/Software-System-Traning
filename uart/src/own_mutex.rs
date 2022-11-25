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
    // TODO: change this to a type that supports interior mutability.
    _t: T,
}

/// SAFETY: TODO: write here an explanation why this is safe
unsafe impl<T> Sync for OwnMutex<T> {}

impl<T> OwnMutex<T> {
    /// Creates a new mutex with the given content.
    pub const fn new(t: T) -> Self {
        Self { _t: t }
    }

    /// this takes a reference to self, and a function that takes a mutable reference to the inner type.
    /// You need to make sure interrupts can't happen.
    pub fn modify<U>(&self, f: impl FnMut(&mut T) -> U) -> U {
        todo!()
    }
}
