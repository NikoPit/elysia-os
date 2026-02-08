use alloc::string::ToString;

use crate::{
    filesystem::{
        path::Path,
        vfs::{FileData, VirtualFS},
    },
    test,
};

pub mod errors;
pub mod impls;
pub mod path;
pub mod vfs;

test!("VFS Basic", || {
    let a_txt = Path::new("/test/a.txt");
    VirtualFS.lock().create_file(a_txt.clone()).unwrap();
    VirtualFS
        .lock()
        .write_file(
            a_txt.clone(),
            FileData {
                content: "abc".to_string(),
            },
        )
        .unwrap();
    let content = VirtualFS.lock().read_file(a_txt.clone()).unwrap().content;

    assert_eq!(content, "abc");
});
