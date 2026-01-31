use lazy_static::lazy_static;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{CS, Segment},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};

use crate::tss;

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, GDTSelectors)= {
        let mut gdt = GlobalDescriptorTable::new();

        // a selector is just a fancy way of saying index. it stores the index and
        // other stuffs about the GDT entry
        let kernel_code_selector = gdt.append(Descriptor::kernel_code_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(&tss::TSS));

        (gdt, GDTSelectors { code_selector: kernel_code_selector, tss_selector: tss_selector })
    };
}

struct GDTSelectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init_gdt() {
    GDT.0.load();

    unsafe {
        // updates the CS so that it knows the gdt stuff or
        // whatever have changed.
        CS::set_reg(GDT.1.code_selector);
        // load the tss from the gdt entry
        load_tss(GDT.1.tss_selector);
    }
}
