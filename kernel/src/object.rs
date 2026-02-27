use core::fmt::Debug;

use alloc::sync::Arc;

use crate::object::error::ObjectError;

pub mod error;

pub trait Object: Send + Sync + Debug {
    fn as_writable(self: Arc<Self>) -> Option<Arc<dyn Writable>> {
        None
    }

    fn as_readable(self: Arc<Self>) -> Option<Arc<dyn Readable>> {
        None
    }
}

pub type ObjectResult<T> = Result<T, ObjectError>;

pub trait Writable: Object {
    /// Write the content of [`buffer`] to [`self`]
    fn write(&self, buffer: &[u8]) -> ObjectResult<usize>;
}

pub trait Readable: Object {
    /// Reads the content of [`self`] and write them to [`buffer`]
    fn read(&self, buffer: &mut [u8]) -> ObjectResult<usize>;
}
