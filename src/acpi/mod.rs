use acpi::AcpiTables;
use conquer_once::spin::{Once, OnceCell};

use crate::acpi::handler::ACPIHandler;

pub mod handler;
pub mod init;

pub static ACPI_TABLE: OnceCell<AcpiTables<ACPIHandler>> = OnceCell::uninit();
