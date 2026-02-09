use core::fmt::Debug;

use alloc::{
    boxed::Box,
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use spin::Mutex;

use crate::filesystem::{errors::FSError, impls::ramfs::RamDirectory, path::Path};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VirtualFS: Mutex<VFS> = Mutex::new(VFS::new());
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

#[derive(Clone, Debug)]
pub struct FileData {
    pub content: String,
}

pub trait File: Send + Sync + Debug {
    fn name(&self) -> FSResult<String>;
    fn read(&self) -> FSResult<FileData>;
    fn write(&mut self, data: FileData) -> FSResult<()>;
}

pub trait Directory: Send + Sync + Debug {
    fn name(&self) -> FSResult<String>;
    fn contents(&self) -> FSResult<&BTreeMap<String, FileLike>>;
    fn new_file(&mut self, name: String) -> FSResult<()>;
    fn mkdir(&mut self, name: String) -> FSResult<()>;

    fn get(&self, name: String) -> FSResult<&FileLike> {
        if self.exists(name.clone()) {
            Ok(self.contents().unwrap().get(&name).unwrap())
        } else {
            Err(FSError::NotFound)
        }
    }
    fn exists(&self, name: String) -> bool {
        self.contents().unwrap().contains_key(&name)
    }

    fn list_contents(&self) -> FSResult<Vec<String>> {
        let mut contents = Vec::new();

        for ele in self.contents()? {
            contents.push(ele.0.clone());
        }

        Ok(contents)
    }
}

pub trait FileSystem: Send + Sync {
    fn init(&mut self) -> FSResult<()>;
}

#[derive(Debug)]
pub enum FileLike {
    File(Arc<Mutex<dyn File>>),
    Directory(Arc<Mutex<dyn Directory>>),
}

pub struct VFS {
    pub root: Arc<Mutex<dyn Directory>>,
    pub filesystems: Vec<Box<Mutex<dyn FileSystem>>>,
}

impl VFS {
    pub fn new() -> Self {
        Self {
            root: Arc::new(Mutex::new(RamDirectory::new("root".to_string()))),
            filesystems: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        for ele in &self.filesystems {
            ele.lock().init();
        }
    }

    fn register_fs(&mut self, fs: impl FileSystem + 'static) {
        self.filesystems.push(Box::new(Mutex::new(fs)));
    }

    pub fn create_file(&mut self, path: Path) -> FSResult<()> {
        let dir = path.navigate(self)?;

        dir.clone().0.lock().new_file(dir.1);

        Ok(())
    }

    pub fn create_dir(&mut self, path: Path) -> FSResult<()> {
        let dir = path.navigate(self)?;

        dir.clone().0.lock().mkdir(dir.1.clone())
    }

    pub fn read_file(&mut self, path: Path) -> FSResult<FileData> {
        let cur_dir = path.navigate(self)?;
        let dir = cur_dir.0.lock();
        let dir_name = cur_dir.1.clone();

        let file_like = dir.get(dir_name)?;
        if let FileLike::File(file) = file_like {
            file.lock().read()
        } else {
            Err(FSError::NotFound)
        }
    }

    pub fn write_file(&mut self, path: Path, data: FileData) -> FSResult<()> {
        let dir = path.navigate(self)?;

        if let Ok(FileLike::File(file)) = dir.0.lock().get(dir.1.clone()) {
            file.lock().write(data);
            Ok(())
        } else {
            Err(FSError::NotFound)
        }
    }

    pub fn delete_file(&mut self, _path: Path) -> FSResult<()> {
        unimplemented!("Just dont create files that your gonna delete lmao its not my problem")
    }

    pub fn list_contents(&self, path: Path) -> FSResult<Vec<String>> {
        let dir = path.navigate(self)?;
        let bindind = dir.0.lock();
        let dir = bindind.get(dir.1.clone());

        if let Ok(FileLike::Directory(dir)) = dir {
            Ok(dir.lock().list_contents()?)
        } else {
            Err(FSError::NotFound)
        }
    }
}
