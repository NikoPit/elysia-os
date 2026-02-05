use core::iter::Map;

use alloc::{
    collections::btree_map::BTreeMap,
    str,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use x86_64::registers::segmentation::FS;

use crate::filesystem::{
    errors::FSError,
    impls::ramfs::{self, RamFS},
    path::Path,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref Vfs: VFS = VFS::new();
}
// INode: pointer to file and file info
// Superblock: basicaly metadata for the partition
// 目录在文件系统中是一种特殊的文件，它的内容是一个列表，列表中的每一项都是一个“目录项”（directory entry），每个目录项记录一个文件名和对应的Inode编号。
//
// Getting a file:
// file path: /home/elysia/file.txt
// Get INode (root) -> FileContent (Directory) -> directoy contents -> INode(to ./elysia)
// INode (to .elysia) -> Elysia -> contents -> INode(file.txt) -> Contents
pub type FSResult<T> = Result<T, FSError>;

pub struct FileData {
    pub content: Vec<u8>,
}

pub struct FileLike {
    pub data: Option<FileData>,
    pub directory: Option<Directory>,
}

impl FileLike {
    pub fn new_directory(directory: Directory) -> Self {
        Self {
            data: None,
            directory: Some(directory),
        }
    }

    pub fn new_data(data: FileData) -> Self {
        Self {
            data: Some(data),
            directory: None,
        }
    }
}

pub struct Directory {
    pub contents: BTreeMap<String, INode>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            contents: BTreeMap::new(),
        }
    }
}

pub trait INode {
    fn get_data(&self) -> FSResult<&FileLike>;
}

pub struct VFS {
    pub root: Option<Arc<&dyn INode>>,
    pub fs: RamFS,
}

impl VFS {
    pub fn new() -> Self {
        Self {
            root: None,
            fs: RamFS::new(),
        }
    }

    pub fn init(&mut self) {
        self.root = Some(Arc::new(self.fs.get_root().unwrap()));
    }

    pub fn create_file(&mut self, path: Path) -> FSResult<()> {
        let mut current_inode = self.root.unwrap().clone();
        for ele in path.get_parts() {
            current_inode = current_inode.get_data()?.directory?.contents.get();
        }
    }

    pub fn create_dir(&mut self, path: Path) -> FSResult<()> {
        unimplemented!()
    }

    pub fn read_file(&mut self, path: Path) -> FSResult<FileData> {
        unimplemented!()
    }

    pub fn write_file(&mut self, path: Path, data: FileData) -> FSResult<()> {
        unimplemented!()
    }

    pub fn delete_file(&mut self, path: Path) -> FSResult<()> {
        unimplemented!()
    }
}
