/// \file tree.rs
/// \author https://github.com/mrodda/

use std::{
    borrow::Borrow,
    path::{Path, PathBuf},
    cell::RefCell,
    fs::{DirEntry, ReadDir, read_dir},
    io::Result,
    rc::{Rc, Weak},
};

use Entry::{File, Dir};

enum Entry {
    File(FileDesc),
    Dir(DirDesc),
}
struct DirDesc {
    path: PathBuf,
    entries: Vec<Entry>,
    size: u64,
}
struct FileDesc {
    path: PathBuf,
    size: u64,
}

pub struct Tree {
    root: DirDesc,
}

impl Entry {
    pub fn size(&self) -> u64 {
        match self {
            File(desc) => desc.size,
            Dir(desc) => desc.size,
        }
    }

    fn print_rec(&self, prefix: String, children_prefix: String) {
        match self {
            File(desc) => desc.print_basename(prefix),
            Dir(desc) => desc.print_rec(prefix, children_prefix),
        };
    }
}

impl FileDesc {
    fn print_basename(&self, prefix: String) {
        println!(
            "{}{} ({}B)",
            prefix,
            self
                .path
                .file_name()
                .expect("could not get file_name")
                .to_str()
                .expect("could not convert to string"),
            self.size
        );
    }
}

impl DirDesc {
    fn new<PathType: Into<PathBuf>>(path: PathType) -> Self {
        let mut desc = DirDesc {
            path: path.into(),
            entries: Vec::new(),
            size: 0
        };
        desc.explore();
        desc
    }

    fn explore(&mut self) {
        match read_dir(&self.path) {
            Ok(rd) => {
                let clos = |rd_entry: DirEntry| {
                    let entry: Entry = rd_entry.into();
                    self.size += entry.size();
                    self.entries.push(entry);
                };
                rd.filter_map(|entry| entry.ok()).for_each(clos);
            },
            Err(_) => (),
        };
    }

    fn print_rec(
        &self,
        prefix: String,
        children_prefix: String,
    ) {
        self.print_basename(prefix);
        let last_item = "└─";
        let last_prefix = "  ";
        let not_last_item = "├─";
        let not_last_prefix = "│ ";
        let it = &mut self.entries.iter().peekable();
        while let Some(entry) = &it.next() {
            let (next_prefix, next_children_prefix) = if let Some(_) = it.peek() {
                (children_prefix.to_owned() + &not_last_item, children_prefix.to_owned() + &not_last_prefix)
            } else {
                (children_prefix.to_owned() + &last_item, children_prefix.to_owned() + &last_prefix)
            };
            entry.print_rec(next_prefix.to_string(), next_children_prefix.to_string());
        }
    }

    pub fn print(&self) {
        self.print_basename("".to_string());
        let last_item = "└─";
        let last_prefix = "  ";
        let not_last_item = "├─";
        let not_last_prefix = "│ ";
        let it = &mut self.entries.iter().peekable();
        while let Some(entry) = &it.next() {
            let (prefix, children_prefix) = if let Some(_) = it.peek() {
                (&not_last_item, &not_last_prefix)
            } else {
                (&last_item, &last_prefix)
            };
            entry.print_rec(prefix.to_string(), children_prefix.to_string());
        }
    }

    fn print_basename(&self, prefix: String) {
        println!(
            "{}{} ({}B)",
            prefix,
            self
                .path
                .file_name()
                .expect("could not get file_name")
                .to_str()
                .expect("could not convert to string"),
            self.size
        );
    }
}

impl From<DirEntry> for Entry {
    fn from(dir_entry: DirEntry) -> Entry {
        let metadata = dir_entry.metadata();
        match metadata {
            Ok(meta) => {
                if meta.is_dir() {
                    Dir(DirDesc::new(dir_entry.path()))
                } else {
                    File(FileDesc {
                        path: dir_entry.path(),
                        size: meta.len().into()
                    })
                }
            },
            Err(_) => File(FileDesc {
                path: dir_entry.path(),
                size: 0
            }),
        }
    }
}

impl Tree {
    pub fn explore(path: PathBuf) -> Self {
        Self {
            root: DirDesc::new(path),
        }
    }

    pub fn print(&self) {
        self.root.print();
    }
}
