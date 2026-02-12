pub mod blocked;
pub mod context;
pub mod exit;
pub mod kernel_task;
pub mod manager;
pub mod memory;
pub mod process;
pub mod switch;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::multitasking::manager::Manager;

lazy_static! {
    pub static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::new());
}
