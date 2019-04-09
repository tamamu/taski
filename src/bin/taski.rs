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
        save_json(&db)?;
        instance = Some(db);
    }
    Ok(instance.unwrap())
}

fn save_json(db: &Database) -> Result<(), Error> {
    let path = ensure_path()?;
    let serialized = serde_json::to_string_pretty(&db)?;
    let mut f = BufWriter::new(File::create(path)?);
    f.write(serialized.as_bytes())?;
    Ok(())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut db = entry().unwrap();
    if let Some(matches) = matches.subcommand_matches("add") {
        let content = matches.args.get("TEXT").unwrap().vals[0]
            .clone()
            .into_string()
            .unwrap();
        let task = Task::new(content);
        db.add_task(task);
        save_json(&db).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("ls") {
        for task in db.list_tasks().iter() {
            println!(
                "{}{}[{}] {}",
                if task.tag == db.current_task {
                    ">"
                } else {
                    " "
                },
                if task.done { "✔" } else { " " },
                //&task.tag[0..8],
                &task.tag,
                task.content
            );
        }
    } else if let Some(matches) = matches.subcommand_matches("current") {
    } else if let Some(matches) = matches.subcommand_matches("done") {
        if let Some(arg) = matches.args.get("TAG") {
            let tag = arg.vals[0].clone().into_string().unwrap();
            db.done_task(&tag).unwrap();
        } else {
            db.done_current_task().unwrap();
        }
        save_json(&db).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("pause") {
    } else if let Some(matches) = matches.subcommand_matches("remove") {
    } else if let Some(matches) = matches.subcommand_matches("resume") {
    } else if let Some(matches) = matches.subcommand_matches("set") {
    }
}
