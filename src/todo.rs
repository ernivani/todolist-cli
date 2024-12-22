#[derive(Debug)]
pub struct TodoItem {
    description: String,
    completed: bool,
}

impl TodoItem {
    pub fn new(description: &str) -> Self {
        TodoItem {
            description: description.to_string(),
            completed: false,
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }

    pub fn matches_description(&self, desc: &str) -> bool {
        self.description == desc
    }
} 