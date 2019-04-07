#[cfg(test)]
mod tests {
    use taski::database;

    #[test]
    fn parse_json() {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open("tests/taski.json").unwrap();
        let reader = BufReader::new(file);

        let u: database::Database = serde_json::from_reader(reader).unwrap();

        assert!(
            u.tasks
                .iter()
                .filter(|&task| task.tag == u.current_task)
                .cloned()
                .collect::<Vec<database::Task>>()
                .len()
                == 1
        );

        assert_eq!(u.tasks[0].time, 500000);
        assert_eq!(u.tasks[0].content, "TODO1");
        assert_eq!(u.tasks[0].done, false);
        assert_eq!(u.tasks[0].children[0].content, "TODO2");
        assert_eq!(u.tasks[0].children[1].content, "TODO3");
        assert_eq!(u.tasks[1].content, "TODO4");
    }

    #[test]
    fn create_task() {
        let task = database::Task::new("new todo".to_owned());
        println!("{}", task.tag);
        assert_eq!(task.tag.len(), 40);
        assert_eq!(task.time, 0);
        assert_eq!(task.content, "new todo");
        assert_eq!(task.done, false);
        assert_eq!(task.children.len(), 0);
    }

    #[test]
    fn add_task() {
        let mut db = database::Database::new();
        let task = database::Task::new("new todo".to_owned());
        db.add_task(task);
        assert_eq!(db.tasks[0].content, "new todo");
    }
}
