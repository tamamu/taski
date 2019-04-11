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
                .collect::<Vec<Box<database::Task>>>()
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
    fn add_child_task() {
        let mut parent_task = database::Task::new("A".to_owned());
        let child_task = database::Task::new("B".to_owned());
        parent_task.add_child(child_task);
        assert_eq!(parent_task.children[0].content, "B");
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

    #[test]
    fn done_task() {
        let mut task = database::Task::new("todo".to_owned());
        task.mark_as_done();
        assert_eq!(task.done, true);
    }

    #[test]
    fn done_task_in_db() {
        let task = database::Task::new("todo".to_owned());
        let mut db = database::Database::new();
        db.add_task(task);
        let tag = db.tasks[0].tag.clone();
        assert!(db.done_task(&tag).is_ok());
        assert!(db.tasks[0].done)
    }

    #[test]
    fn set_current_task() {
        let mut db = database::Database::new();
        let task = database::Task::new("todo".to_owned());
        db.add_task(task);
        let tag = db.tasks[0].tag.clone();
        assert!(db.set_current_task(&tag).is_ok());
        assert_eq!(db.current_task, tag);
    }

    #[test]
    fn done_current_task() {
        let mut db = database::Database::new();
        let task = database::Task::new("todo".to_owned());
        db.add_task(task);
        let tag = db.tasks[0].tag.clone();
        db.set_current_task(&tag).ok();
        assert!(db.done_current_task().is_ok());
        assert!(db.tasks[0].done);
    }

    #[test]
    fn get_current_task() {
        let mut db = database::Database::new();
        let task = database::Task::new("todo".to_owned());
        db.add_task(task);
        let tag = db.tasks[0].tag.clone();
        db.set_current_task(&tag).ok();
        let current_task = db.get_current_task().unwrap();
        assert_eq!(current_task.content, "todo");
    }
}
