use clap::{ArgEnum, Parser};
use crate::{
    tree::Tree,
    FileSizeFormat::{ Byte, Kilo, Mega, Giga, Tera }
};
use std::{path::PathBuf, process::exit};

mod tree;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum FileSizeFormat {
    /// Bytes.
    Byte,
    /// Kilobytes.
    Kilo,
    /// Megabytes.
    Mega,
    /// Gigabytes.
    Giga,
    /// Terabytes.
    Tera,
}

impl FileSizeFormat {
    fn to_string(&self, size: u64) -> String {
        match self {
            Byte => "B".into(),
            Kilo => (size / 1_000).to_string() + "KB",
            Mega => (size / 1_000_000).to_string() + "MB",
            Giga => (size / 1_000_000_000).to_string() + "GB",
            Tera => (size / 1_000_000_000_000).to_string() + "TB",
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path
    #[clap(value_parser)]
    path: Option<String>,
    /// Don't show the size for files and directories.
    #[clap(short = 'H', long = "hide-size", action)]
    hide_size: bool,
    /// Unit
    #[clap(arg_enum, short, long, value_parser, default_value_t = FileSizeFormat::Kilo)]
    unit: FileSizeFormat,
}

impl Args {
    fn get_path(&self) -> PathBuf {
        if let Some(input) = &self.path {
            input.into()
        } else {
            match std::env::current_dir() {
                Ok(cwd) => cwd,
                Err(err) => {
                    eprintln!("Could not read current directory. Error: {}", err);
                    exit(1)
                }
            }
        }
    }
}

fn main() {
    let cli = Args::parse();
    let tree = Tree::explore(cli.get_path());
    tree.print(cli.hide_size, cli.unit);
}