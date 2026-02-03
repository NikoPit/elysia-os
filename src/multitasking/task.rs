use core::{
    pin::Pin,
    task::{Context, Poll},
};

use alloc::boxed::Box;

// the reason that the future doesnt return anything
// is because this future is only supposed to be polled
// for the effect of polling, not the return value
pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            future: Box::pin(future),
        }
    }

    pub fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
