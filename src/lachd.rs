use quicli::prelude::*;
use structopt::StructOpt;

use std::path::Path;
use tempfile::NamedTempFile;
use std::io::{self, Write};
use std::fs;
use std::env;

use std::io::ErrorKind;

use fork::{fork, Fork};
use std::process::Command;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", long = "foreground")]
    foreground: bool,

    #[structopt(short = "k", long = "kill")]
    kill: bool,
}

struct Daemon {
    locked: bool
}

impl Daemon {
    fn new() -> Daemon {
        Daemon { locked : false }
    }

    fn lock(&mut self) -> () {
        self.locked = true;
    }
}

struct Table {
    path: std::path::PathBuf,
}

impl Table {
    fn new(pb: std::path::PathBuf) -> Table {
        Table { path : pb }
    }
}

fn load_tables() -> Vec<Table> {
    // todo source from config file?
    let dirs = vec![ "/etc/lach.d" ];

    // todo fix rust crimes
    dirs.into_iter().map(|dir| fs::read_dir(dir).ok())
        .map(|file_list| file_list.unwrap().map(|entry| entry.ok().unwrap().path()).collect())
        .map(|path_buffer| Table::new(path_buffer))
        .collect()
}

fn main() {
    let args = Cli::from_args();
    
    let mut daemon = Daemon::new();
    daemon.lock();
    if ! daemon.locked {
        panic!("can't get lock!");
    }
    
    // load lachtables
    let tables = load_tables();

    // setup inotify

    // loop { poll }
}
