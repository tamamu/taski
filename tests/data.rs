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
}
