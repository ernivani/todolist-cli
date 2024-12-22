use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::todo::TodoItem;
use crate::json::{parse_json, to_json};

pub struct TodoStorage {
    todo_dir: PathBuf,
}

impl TodoStorage {
    pub fn new() -> Self {
        let home = env::var("HOME").expect("Could not find home directory");
        let todo_dir = PathBuf::from(home).join(".todo");
        TodoStorage { todo_dir }
    }

    fn get_todo_file(&self) -> PathBuf {
        self.todo_dir.join("todo.json")
    }

    fn get_backup_file(&self) -> PathBuf {
        self.todo_dir.join("todo.backup.json")
    }

    fn ensure_todo_dir(&self) -> io::Result<()> {
        if !self.todo_dir.exists() {
            fs::create_dir_all(&self.todo_dir)?;
        }
        Ok(())
    }

    fn create_backup(&self) -> io::Result<()> {
        let todo_file = self.get_todo_file();
        let backup_file = self.get_backup_file();
        
        if todo_file.exists() {
            fs::copy(&todo_file, &backup_file)?;
        }
        Ok(())
    }

    pub fn load_todos(&self) -> Vec<TodoItem> {
        let todo_file = self.get_todo_file();
        
        if !todo_file.exists() {
            return Vec::new();
        }
        
        match fs::read_to_string(&todo_file) {
            Ok(contents) => {
                let todos = parse_json(&contents);
                if todos.is_empty() {
                    println!("Warning: Todo file exists but no tasks were loaded");
                }
                todos
            },
            Err(error) => {
                eprintln!("Error reading todo file: {}", error);
                Vec::new()
            }
        }
    }

    pub fn save_todos(&self, todo_list: &[TodoItem]) -> io::Result<()> {
        self.ensure_todo_dir()?;
        self.create_backup()?;
        
        let todo_file = self.get_todo_file();
        let json = to_json(todo_list);
        
        let temp_file = todo_file.with_extension("tmp");
        fs::write(&temp_file, &json)?;
        
        fs::rename(temp_file, todo_file)?;
        
        Ok(())
    }

    pub fn restore_from_backup(&self) -> io::Result<Vec<TodoItem>> {
        let backup_file = self.get_backup_file();
        if !backup_file.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "No backup file found"));
        }
        
        let contents = fs::read_to_string(&backup_file)?;
        Ok(parse_json(&contents))
    }
} 