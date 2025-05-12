use core::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    id: u32,
    title: String,
    description: String,
    due_date: String,
    completed: bool,
}

impl Todo {
    pub fn new(
        id: u32,
        title: String,
        description: String,
        due_date: String,
        completed: bool,
    ) -> Self {
        Self {
            id,
            title,
            description,
            due_date,
            completed,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_due_date(&self) -> String {
        self.due_date.clone()
    }

    #[allow(dead_code)]
    pub fn get_completed(&self) -> bool {
        self.completed
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.completed {
            write!(f, "{} ({}) [Completed]", self.title, self.due_date)
        } else {
            write!(f, "{} ({})", self.title, self.due_date)
        }
    }
}
