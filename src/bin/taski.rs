#[macro_use]
extern crate clap;
use clap::App;

extern crate dirs;
use dirs::home_dir;

use serde_json;

use std::collections::VecDeque;
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
        let task = Task::new(content.clone());
        if let Some(matches) = matches.args.get("parent") {
            let tag = matches.vals[0].clone().into_string().unwrap();
            db.add_child_task(&tag, task);
        } else {
            db.add_task(task);
        }
        save_json(&db).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("ls") {
        for printable in db.list_tasks().iter() {
            println!(
                "{: >level$} [{}] {}",
                if printable.done_parent || printable.task.done {
                    "✔"
                } else if printable.task.tag == db.current_task {
                    ">"
                } else {
                    " "
                },
                //&task.tag[0..8],
                &printable.task.tag,
                printable.task.content,
                level = printable.level * 4
            );
        }
    } else if let Some(matches) = matches.subcommand_matches("current") {
        if let Some(matches) = matches.subcommand_matches("ls") {
            let task = db.get_current_task();
            task.map(|taken| {
                let mut taskq: VecDeque<(&Box<Task>, usize, bool, bool)> = VecDeque::new();
                let mut selected = false;
                for task in taken.children.iter() {
                    if selected {
                        taskq.push_back((task, 0, false, false));
                    } else if !task.done {
                        taskq.push_back((task, 0, false, true));
                        selected = true;
                    }
                }
                while !taskq.is_empty() {
                    let (task, level, done, parent_selected) = taskq.pop_front().unwrap();
                    let mut selected = false;
                    // For ordering
                    let mut tmpq: VecDeque<(&Box<Task>, usize, bool, bool)> = VecDeque::new();
                    for child in task.children.iter() {
                        if parent_selected && !selected && !child.done {
                            tmpq.push_front((child, level + 1, done, true));
                            selected = true;
                        } else {
                            tmpq.push_front((child, level + 1, done, false));
                        }
                    }
                    while let Some(child) = tmpq.pop_front() {
                        taskq.push_front(child);
                    }
                    println!(
                        "{: >level$} [{}] {}",
                        if done || task.done {
                            "✔"
                        } else if parent_selected {
                            ">"
                        } else {
                            " "
                        },
                        &task.tag,
                        task.content,
                        level = level * 4
                    );
                }
            });
        } else if let Some(matches) = matches.subcommand_matches("time") {
        } else {
            let task = db.get_current_task();
            task.map(|taken| println!("{}", taken.content));
        }
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
        let tag = matches.args.get("TAG").unwrap().vals[0]
            .clone()
            .into_string()
            .unwrap();
        db.set_current_task(&tag).unwrap();
        save_json(&db).unwrap();
    }
}
