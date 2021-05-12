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
    //#[structopt(short = "h", long = "help")]
    //help: bool,

    #[structopt(short = "d", long = "debug")]
    debug: bool,

    #[structopt(short = "l", long = "list")]
    list: bool,

    #[structopt(short = "r", long = "remove")]
    remove: bool,

    #[structopt(short = "e", long = "edit")]
    //edit: bool,
    edit: Option<Option<String>>,

    #[structopt(short = "t", long = "types")]
    types: bool,

    #[structopt(short = "x", long = "reload")]
    reload: bool,

    //#[structopt(short = "v", long = "version")]
    //version: bool,

    #[structopt(short = "u", long = "user")]
    user: Option<String>,


}

struct Table {
    name: String,
    path: std::path::PathBuf
}

impl Table {
    fn load(name: Option<String>) -> Table {
        let name = name.unwrap_or(String::from("default"));
        let home = std::env::var("HOME").unwrap();
        let path = Path::new(&home).join(".config/lachy").join(&name);
        Table {name: name, path: path}
    }

    fn edit(&self) -> Result<(), Error> {
        println!("Editing table: {}", self.path.to_str().unwrap());

        let tempfile = NamedTempFile::new()?;
        let tempfile_path = tempfile.into_temp_path();

        println!("Using temp path: {}", tempfile_path.to_str().unwrap());

        match fs::copy(&self.path, &tempfile_path) {
            Ok(s) => s,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => panic!("need to create file {}", e), //todo "no lachtab for $user, using an empty one"
                other => panic!("other error!"),
            },
        };


        let editor = determine_editor();

        let mut child = Command::new(editor)
                        .arg(&tempfile_path)
                        .spawn()
                        .expect("failed to run editor");

        let ecode = child.wait()
                         .expect("failed to wait on editor");

        // todo
        // check tmpfile for correctness, while incorrect keep prompting
        

        match fs::copy(&tempfile_path, &self.path) {
            Ok(s) => s,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => panic!("??"), 
                other => panic!("other error!"),
            },
        };

        // rust boilerplate for a non-returning function that can fail?
        Ok(())
    }
}

fn determine_editor() -> String {
    String::from("vim")
    //match env::var_os("EDITOR") {
    //    Some(val) => println!("{}: {:?}", key, val),
    //    None => println!("{} is not defined in the environment.", key)
    //}
}

fn main() {
    let args = Cli::from_args();

    // edit is an optional arg, with optional value (the file to edit)
    match args.edit {
        Some(maybe_file) => {
            let table = Table::load(maybe_file);
            table.edit(); },
        None => (),
    }
}
