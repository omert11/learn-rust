use crate::todo::db::TodoDb;
use crate::todo::todo::Todo;
use crate::utils::ui;

pub fn main() {
    let operations = vec!["Add", "List", "Exit"];

    loop {
        let selection = ui::ui_select("Select an operation", operations.clone());

        match selection {
            0 => add_todo(),
            1 => list_todos(),
            _ => break,
        }
    }
}

fn add_todo() {
    {
        let mut db = TodoDb::new();
        let id = db.get_current_id();
        let title = ui::get_input::<String>("Enter a todo");
        let description = ui::get_input::<String>("Enter a description");
        let due_date = ui::get_date("Enter a due date");
        let todo = Todo::new(id, title, description, due_date, false);
        db.add_todo(&todo);
        println!("Todo added: {}", todo.to_string());
    }
}

fn list_todos() {
    let mut db = TodoDb::new();
    let todos = db.get_todos();
    let todo_strings: Vec<String> = todos.iter().map(|todo| todo.to_string()).collect();
    let todo_strings: Vec<&str> = todo_strings.iter().map(|todo| todo.as_str()).collect();

    let selections = ui::ui_multi_select("Listed todos", todo_strings);

    if selections.len() > 0 {
        let selected_ids = selections
            .iter()
            .map(|&x| todos[x as usize].get_id())
            .collect::<Vec<u32>>();

        let mut operations = vec![
            "Mark as completed",
            "Mark as not completed",
            "Delete",
            "Cancel",
        ];

        if selected_ids.len() == 1 {
            operations.insert(0, "Edit");
        }

        let selection = ui::ui_select("Select an operation", operations.clone());
        let selection_text = operations[selection as usize];

        match selection_text {
            "Edit" => db.update_todo(&edit_todo(selected_ids[0]).unwrap()),
            "Mark as completed" => db.set_completed(selected_ids, true),
            "Mark as not completed" => db.set_completed(selected_ids, false),
            "Delete" => db.delete_todos(selected_ids),
            _ => (),
        }
    }
}

fn edit_todo(id: u32) -> Result<Todo, String> {
    let db = TodoDb::new();
    let todo = db.get_todo(id);
    let title = ui::get_input_with_default::<String>(
        "Enter a title",
        todo.map_or(String::new(), |todo| todo.get_title()),
    );
    let description = ui::get_input_with_default::<String>(
        "Enter a description",
        todo.map_or(String::new(), |todo| todo.get_description()),
    );
    let due_date = ui::get_date_with_default(
        "Enter a due date",
        todo.map_or(String::new(), |todo| todo.get_due_date()),
    );
    let todo = Todo::new(id, title, description, due_date, false);
    println!("Todo updated: {}", todo.to_string());
    Ok(todo)
}
