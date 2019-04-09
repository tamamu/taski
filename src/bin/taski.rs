#[macro_use]
extern crate clap;
use clap::App;

extern crate dirs;
use dirs::home_dir;

use serde_json;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::path::{Path, PathBuf};

use taski::database::{Database, Task};

static TASKI_JSON_PATH: &str = "TASKI_JSON_PATH";

fn ensure_path() -> Result<PathBuf, Error> {
    let mut pathbuf = PathBuf::new();
    match env::var_os(TASKI_JSON_PATH) {
        Some(val) => pathbuf.push(val),
        None => {
            pathbuf.push(home_dir().unwrap());
            pathbuf.push(".taski.json");
        }
    }
    Ok(pathbuf)
}

fn entry() -> Result<Database, Error> {
    let path = ensure_path()?;
    let mut instance: Option<Database> = None;
    if path.exists() {
        let f = File::open(path).expect("file not found");
        let db: Database = serde_json::from_reader(f).expect("error while reading json");
        instance = Some(db);
    } else {
        let db = Database::new();
        save_json(&db);
        instance = Some(db);
    }
    Ok(instance.unwrap())
}

    Ok(())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut db = entry().unwrap();
    if let Some(matches) = matches.subcommand_matches("add") {
        dbg!(matches.args.get("TEXT"));
    } else if let Some(matches) = matches.subcommand_matches("ls") {

    } else if let Some(matches) = matches.subcommand_matches("current") {
    } else if let Some(matches) = matches.subcommand_matches("done") {
    } else if let Some(matches) = matches.subcommand_matches("pause") {
    } else if let Some(matches) = matches.subcommand_matches("remove") {
    } else if let Some(matches) = matches.subcommand_matches("resume") {
    } else if let Some(matches) = matches.subcommand_matches("set") {
    }
}
