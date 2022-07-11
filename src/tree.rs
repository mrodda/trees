/// \file tree.rs
/// \author https://github.com/mrodda/

use crate::FileSizeUnit;
use Entry::{File, Dir};
use std::{
    path::PathBuf,
    fs::{DirEntry, read_dir},
};

const LAST_ITEM: &str = "└─";
const LAST_PREFIX: &str = "  ";
const NOT_LAST_ITEM: &str = "├─";
const NOT_LAST_PREFIX: &str = "│ ";

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

    fn print_rec(&self, prefix: String, children_prefix: String, hide_size: bool, unit: FileSizeUnit) {
        match self {
            File(desc) => desc.print_basename(prefix, hide_size, unit),
            Dir(desc) => desc.print_rec(prefix, children_prefix, hide_size, unit),
        };
    }
}

impl FileDesc {
    fn print_basename(&self, prefix: String, hide_size: bool, unit: FileSizeUnit) {
        if hide_size {
            println!(
                "{}{}",
                prefix,
                self
                    .path
                    .file_name()
                    .expect("could not get file_name")
                    .to_str()
                    .expect("could not convert string")
            );
        } else {
            println!(
                "{}{} ({}{})",
                prefix,
                self
                    .path
                    .file_name()
                    .expect("could not get file_name")
                    .to_str()
                    .expect("could not convert string"),
                self.size,
                unit.to_str()
            );
        }
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
        hide_size: bool,
        unit: FileSizeUnit
    ) {
        self.print_basename(prefix, hide_size, unit);
        let it = &mut self.entries.iter().peekable();
        while let Some(entry) = &it.next() {
            let (next_prefix, next_children_prefix) = if let Some(_) = it.peek() {
                (children_prefix.to_owned() + &NOT_LAST_ITEM, children_prefix.to_owned() + &NOT_LAST_PREFIX)
            } else {
                (children_prefix.to_owned() + &LAST_ITEM, children_prefix.to_owned() + &LAST_PREFIX)
            };
            entry.print_rec(next_prefix.to_string(), next_children_prefix.to_string(), hide_size, unit);
        }
    }

    pub fn print(&self, hide_size: bool, unit: FileSizeUnit) {
        self.print_basename("".to_string(), hide_size, unit);
        let it = &mut self.entries.iter().peekable();
        while let Some(entry) = &it.next() {
            let (prefix, children_prefix) = if let Some(_) = it.peek() {
                (&NOT_LAST_ITEM, &NOT_LAST_PREFIX)
            } else {
                (&LAST_ITEM, &LAST_PREFIX)
            };
            entry.print_rec(prefix.to_string(), children_prefix.to_string(), hide_size, unit);
        }
    }

    fn print_basename(&self, prefix: String, hide_size: bool, unit: FileSizeUnit) {
        if hide_size {
            println!(
                "{}{}",
                prefix,
                self
                    .path
                    .file_name()
                    .expect("could not get file_name")
                    .to_str()
                    .expect("could not convert string")
            );
        } else {
            println!(
                "{}{} ({}{})",
                prefix,
                self
                    .path
                    .file_name()
                    .expect("could not get file_name")
                    .to_str()
                    .expect("could not convert string"),
                self.size,
                unit.to_str()
            );
        }
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

    pub fn print(&self, hide_size: bool, unit: FileSizeUnit) {
        self.root.print(hide_size, unit);
    }
}
