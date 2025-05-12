#[cfg(test)]
mod tests {
    use std::fs;

    use super::super::db::TodoDb;
    use super::super::todo::Todo;

    fn create_test_todo(id: u32) -> Todo {
        Todo::new(
            id,
            format!("Test Todo {}", id),
            format!("Description for todo {}", id),
            "2024-12-31".to_string(),
            false,
        )
    }

    fn cleanup_test_db(path: &str) {
        fs::remove_file(path).unwrap_or(());
    }

    #[test]
    fn test_todo_creation() {
        let todo = create_test_todo(1);
        assert_eq!(todo.get_id(), 1);
        assert_eq!(todo.get_title(), "Test Todo 1");
        assert_eq!(todo.get_description(), "Description for todo 1");
        assert_eq!(todo.get_due_date(), "2024-12-31");
        assert!(!todo.get_completed());
    }

    #[test]
    fn test_todo_completion() {
        let mut todo = create_test_todo(1);
        assert!(!todo.get_completed());

        todo.set_completed(true);
        assert!(todo.get_completed());

        todo.set_completed(false);
        assert!(!todo.get_completed());
    }

    #[test]
    fn test_todo_display() {
        let todo = create_test_todo(1);
        assert_eq!(format!("{}", todo), "Test Todo 1 (2024-12-31)");

        let mut completed_todo = create_test_todo(2);
        completed_todo.set_completed(true);
        assert_eq!(
            format!("{}", completed_todo),
            "Test Todo 2 (2024-12-31) [Completed]"
        );
    }

    #[test]
    fn test_tododb_creation() {
        let path = "db/test_creation.json";
        {
            let db = TodoDb::new(Some(path));
            assert_eq!(db.get_todos().len(), 0);
        }
        cleanup_test_db(path);
    }

    #[test]
    fn test_tododb_add_and_get() {
        let path = "db/test_add_and_get.json";
        {
            let mut db = TodoDb::new(Some(path));

            let todo1 = create_test_todo(1);
            let todo2 = create_test_todo(2);

            db.add_todo(&todo1);
            db.add_todo(&todo2);

            let todos = db.get_todos();
            assert_eq!(todos.len(), 2);
            assert_eq!(todos[0].get_id(), 1);
            assert_eq!(todos[1].get_id(), 2);
        }
        cleanup_test_db(path);
    }

    #[test]
    fn test_tododb_get_current_id() {
        let path = "db/test_get_current_id.json";
        {
            let mut db = TodoDb::new(Some(path));

            assert_eq!(db.get_current_id(), 1);

            let todo1 = create_test_todo(1);
            db.add_todo(&todo1);
            assert_eq!(db.get_current_id(), 2);
        }
        cleanup_test_db(path);
    }

    #[test]
    fn test_tododb_get_todo() {
        let path = "db/test_get_todo.json";
        {
            let mut db = TodoDb::new(Some(path));

            let todo = create_test_todo(1);
            db.add_todo(&todo);

            let retrieved_todo = db.get_todo(1).unwrap();
            assert_eq!(retrieved_todo.get_id(), 1);
            assert_eq!(retrieved_todo.get_title(), "Test Todo 1");

            assert!(db.get_todo(999).is_none());
        }
        cleanup_test_db(path);
    }

    #[test]
    fn test_tododb_update_todo() {
        let path = "db/test_update_todo.json";
        {
            let mut db = TodoDb::new(Some(path));

            let mut todo = create_test_todo(1);
            db.add_todo(&todo);

            todo.set_completed(true);
            db.update_todo(&todo);

            let updated_todo = db.get_todo(1).unwrap();
            assert!(updated_todo.get_completed());
        }
        cleanup_test_db(path);
    }

    #[test]
    fn test_tododb_set_completed() {
        let path = "db/test_set_completed.json";
        {
            let mut db = TodoDb::new(Some(path));

            let todo1 = create_test_todo(1);
            let todo2 = create_test_todo(2);
            db.add_todo(&todo1);
            db.add_todo(&todo2);

            db.set_completed(vec![1, 2], true);

            let todos = db.get_todos();
            assert!(todos[0].get_completed());
            assert!(todos[1].get_completed());
        }
        cleanup_test_db(path);
    }

    #[test]
    fn test_tododb_delete_todos() {
        let path = "db/test_delete_todos.json";
        {
            let mut db = TodoDb::new(Some(path));

            let todo1 = create_test_todo(1);
            let todo2 = create_test_todo(2);
            let todo3 = create_test_todo(3);

            db.add_todo(&todo1);
            db.add_todo(&todo2);
            db.add_todo(&todo3);

            db.delete_todos(vec![1, 3]);

            let todos = db.get_todos();
            assert_eq!(todos.len(), 1);
            assert_eq!(todos[0].get_id(), 2);
        }
        cleanup_test_db(path);
    }

    #[test]
    fn test_tododb_persistence() {
        let path = "db/test_persistence.json";
        {
            let mut db = TodoDb::new(Some(path));
            let todo = create_test_todo(1);
            db.add_todo(&todo);
        } // db is dropped here, should save to file

        {
            let db = TodoDb::new(Some(path));
            let todos = db.get_todos();
            assert_eq!(todos.len(), 1);
            assert_eq!(todos[0].get_id(), 1);
        }

        cleanup_test_db(path);
    }
}
