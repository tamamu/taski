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

use taski::database;

fn entry() -> Result<(), Error> {
    let key = "TASKI_JSON_PATH";
    let mut pathbuf = PathBuf::new();
    match env::var_os(key) {
        Some(val) => pathbuf.push(val),
        None => {
            pathbuf.push(home_dir().unwrap());
            pathbuf.push(".taski.json");
        }
    }
    let path = Path::new(&pathbuf);
    if !path.exists() {
        let instance = database::Database::new();
        let serialized = serde_json::to_string(&instance)?;
        let mut f = BufWriter::new(File::create(path)?);
        f.write(serialized.as_bytes())?;
    }
    Ok(())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    entry().unwrap();
}
