use core::ffi::c_long;

use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use conquer_once::spin::{Once, OnceCell};
use spin::Mutex;

use crate::{
    filesystem::{
        self,
        impls::ramfs,
        vfs::{self, Directory, FSResult, File, FileData, FileLike, FileSystem},
    },
    os::get_os,
};

pub struct RamFS {
    root: RamDirectory,
}

pub struct RamDirectory {
    name: String,
    contents: BTreeMap<String, FileLike>,
}

pub struct RamFile {
    pub name: String,
    pub content: FileData,
}

impl RamFile {
    pub fn new(name: String) -> Self {
        Self {
            name,
            content: FileData {
                content: "".to_string(),
            },
        }
    }
}

impl File for RamFile {
    fn name(&self) -> FSResult<String> {
        Ok(self.name.clone())
    }

    fn read(&self) -> FSResult<FileData> {
        Ok(self.content.clone())
    }

    fn write(&mut self, data: FileData) -> FSResult<()> {
        Ok(self.content = data)
    }
}

impl FileSystem for RamFS {
    fn init(&mut self) -> FSResult<()> {
        Ok(())
    }
}

impl Directory for RamDirectory {
    fn name(&self) -> FSResult<String> {
        Ok(self.name.clone())
    }

    fn contents(&self) -> FSResult<&BTreeMap<String, FileLike>> {
        Ok(&self.contents)
    }

    fn new_file(&mut self, name: String) -> FSResult<()> {
        self.contents.insert(
            name.clone(),
            FileLike::File(Arc::new(Mutex::new(RamFile::new(name)))),
        );
        Ok(())
    }

    fn mkdir(&mut self, name: String) -> FSResult<()> {
        let dir = Arc::new(Mutex::new(RamDirectory::new(name.clone())));
        self.contents.insert(name, FileLike::Directory(dir));
        Ok(())
    }
}

impl RamDirectory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            contents: BTreeMap::new(),
        }
    }
}

impl RamFS {
    pub fn new() -> Self {
        Self {
            root: RamDirectory::new("root".to_string()),
        }
    }
}
