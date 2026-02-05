use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::filesystem::vfs::{FSResult, INode, Vfs};

pub enum PathPart {
    // [TODO] CurrentDir,
    // [TODO] ParentDir,
    Root,
    Normal(String),
}

pub struct Path(Vec<PathPart>);

impl Path {
    pub fn new(path: &str) -> Self {
        Self(Self::parse(path))
    }

    fn parse(path: &str) -> Vec<PathPart> {
        let mut buf = String::new();
        let mut vec = Vec::new();

        if path.chars().nth(0) == Some('/') {
            vec.push(PathPart::Root);
        }

        for ch in path.chars() {
            match ch {
                '/' => {
                    if buf.is_empty() {
                        continue;
                    }
                    vec.push(PathPart::Normal(buf));
                    buf.clear()
                }
                _ => buf.push(ch),
            }
        }

        vec.push(PathPart::Normal(buf));

        vec
    }

    pub fn get_inode(&self) -> FSResult<INode> {
        unimplemented!()
    }

    pub fn get_parts(&mut self) -> Vec<PathPart> {
        self.0
    }
}
