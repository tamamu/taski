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
    fn add_task_at_root() {
        let mut db = database::Database::new();
        let task = database::Task::new("new todo".to_owned());
        db.add_task(task);
        assert_eq!(db.tasks[0].content, "new todo");
    }

    #[test]
    fn add_task_in_a_task() {
        let mut db = database::Database::new();
        let parent_task = database::Task::new("todo1".to_owned());
        db.add_task(parent_task);
        let parent_tag = db.tasks[0].tag.clone();
        let child_task = database::Task::new("todo2".to_owned());
        assert!(db.add_child_task(&parent_tag, child_task).is_ok());
        assert_eq!(db.tasks[0].children[0].content, "todo2");
    }

    #[test]
    fn list_tags() {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open("tests/taski.json").unwrap();
        let reader = BufReader::new(file);

        let u: database::Database = serde_json::from_reader(reader).unwrap();
        assert_eq!(
            u.list_tasks()
                .iter()
                .map(|task| task.tag.clone())
                .collect::<Vec<String>>(),
            vec!["foo", "bar", "hoge", "baz"]
        );
    }
}
