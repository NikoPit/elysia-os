use crate::memory::page_table_wrapper::PageTableWrapped;

#[derive(Default, Debug)]
pub struct AddrSpace {
    pub page_table: PageTableWrapped,
}

impl AddrSpace {
    pub fn load(&self) {
        self.page_table.load();
    }
}
