pub mod context;
pub mod kernel_task;
pub mod manager;
pub mod memory;
pub mod process;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::multitasking::manager::Manager;

lazy_static! {
    pub static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::new());
}
