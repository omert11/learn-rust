use serde_json;

use crate::{
    todo::todo::Todo,
    utils::io::{read_to_string_or_default, write_to_file},
};

pub struct TodoDb {
    path: String,
    todos: Vec<Todo>,
}

fn read_todos(path: &str) -> Vec<Todo> {
    let todos: Vec<Todo> =
        serde_json::from_str(&read_to_string_or_default(path, "[]").unwrap()).unwrap();
    todos
}

impl TodoDb {
    pub fn new(path: Option<&str>) -> Self {
        let path = path.unwrap_or("db/todos.json").to_string();
        let todos = read_todos(&path);
        Self { path, todos }
    }

    pub fn add_todo(&mut self, todo: &Todo) {
        if self.todos.iter().any(|t| t.get_id() == todo.get_id()) {
            return;
        }
        self.todos.push(todo.clone());
    }

    pub fn get_current_id(&self) -> u32 {
        self.todos.last().map_or(0, |todo| todo.get_id()) + 1
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        self.todos.clone()
    }

    pub fn get_todo(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.get_id() == id)
    }
    pub fn update_todo(&mut self, todo: &Todo) {
        if let Some(index) = self.todos.iter().position(|t| t.get_id() == todo.get_id()) {
            self.todos[index] = todo.clone();
        }
    }
    pub fn set_completed(&mut self, ids: Vec<u32>, completed: bool) {
        for id in ids {
            self.todos
                .iter_mut()
                .find(|todo| todo.get_id() == id)
                .unwrap()
                .set_completed(completed);
        }
    }

    pub fn delete_todos(&mut self, ids: Vec<u32>) {
        self.todos.retain(|todo| !ids.contains(&todo.get_id()));
    }
}

impl Drop for TodoDb {
    fn drop(&mut self) {
        let todos = serde_json::to_string(&self.todos).unwrap();
        write_to_file(&self.path, &todos).unwrap();
    }
}
