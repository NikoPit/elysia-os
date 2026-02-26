use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadID(pub u64);

impl Default for ThreadID {
    fn default() -> Self {
        static TID: AtomicU64 = AtomicU64::new(0);
        Self(TID.fetch_add(1, Ordering::Relaxed))
    }
}
