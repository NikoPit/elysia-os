use alloc::{collections::btree_map::BTreeMap, string::String, vec::Vec};
use conquer_once::spin::{Once, OnceCell};
use spin::Mutex;

use crate::{
    filesystem::{
        self,
        vfs::{self, Directory, FSResult, FileLike, INode, Vfs},
    },
    os::get_os,
};

pub struct RamFS {
    files: Mutex<Vec<FileLike>>,
    superblock: SuperBlock,
}

pub struct SuperBlock {
    inodes: BTreeMap<u64, INode>,
}

impl SuperBlock {
    pub fn new() -> Self {
        Self {
            inodes: BTreeMap::new(),
        }
    }
}

impl RamFS {
    pub fn new() -> Self {
        Self {
            files: Mutex::new(Vec::new()),
            superblock: SuperBlock::new(),
        }
    }

    pub fn init(&mut self) {
        self.files
            .lock()
            .push(FileLike::new_directory(Directory::new()));
    }

    pub fn get_root(&mut self) -> FSResult<&INode> {
        Ok(self.superblock.inodes.get(&2).expect("awa"))
    }
}

pub struct INode {
    id: u64,
    index: usize,
}

impl INode {
    pub fn new(id: u64, index: usize) -> Self {
        Self { id, index }
    }
}

impl vfs::INode for INode {
    fn get_data(&self) -> FSResult<&vfs::FileLike> {
        if self.id == 2 {
            return Ok(Vfs.fs.files.lock().get(0).unwrap());
        } else {
            Ok(Vfs.fs.files.lock().get(self.index).unwrap())
        }
    }
}
