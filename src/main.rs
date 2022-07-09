#![allow(dead_code, unused_imports)]

use std::{
    borrow::Borrow,
    cell::RefCell,
    ffi::OsString,
    fmt::Display,
    fs::{DirEntry, ReadDir},
    path::{Path, PathBuf},
    rc::{Rc, Weak},
    error::Error,
};

use crate::{
    tree::{Tree,},
};

mod tree;

fn main() {
    let mut path = std::env::current_dir().expect("couldn't find current dir");
    for arg in std::env::args().skip(1) {
        match arg {
            _ => path = arg.to_owned().into(),
        };
    };
    let tree = Tree::explore(path);
    tree.print();
}
