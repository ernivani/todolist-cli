mod todo;
mod storage;
mod json;

use std::env;
use todo::TodoItem;
use storage::TodoStorage;

fn main() {
    let storage = TodoStorage::new();
    let mut todo_list = storage.load_todos();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        display_tasks(&todo_list);
        return;
    }

    match args[1].as_str() {
        "help" | "--help" | "-h" => {
            println!("Todo List Manager - Available Commands:");
            println!("  add <task>        Add a new task");
            println!("  done <id/name>    Mark a task as completed");
            println!("  remove <id/name>  Remove a task");
            println!("  restore           Restore from backup");
            println!("  help              Show this help message");
            println!("\nExample:");
            println!("  todo add \"Buy groceries\"");
            println!("  todo done 1");
        },
        "add" => {
            if args.len() < 3 {
                println!("Usage: todo add <task1> [task2] [task3] ...");
                return;
            }
            for task in &args[2..] {
                todo_list.push(TodoItem::new(task));
                println!("Added task: {}", task);
            }
            if let Err(e) = storage.save_todos(&todo_list) {
                eprintln!("Error saving todos: {}", e);
            }
        },
        "done" => {
            if args.len() < 3 {
                println!("Usage: todo done <task_id or task_name>");
                return;
            }
            
            if let Ok(id) = args[2].parse::<usize>() {
                if id > 0 && id <= todo_list.len() {
                    todo_list[id - 1].mark_completed();
                    println!("Marked task {} as complete!", id);
                    if let Err(e) = storage.save_todos(&todo_list) {
                        eprintln!("Error saving todos: {}", e);
                    }
                    return;
                }
            }
            
            if let Some(pos) = todo_list.iter().position(|item| item.matches_description(&args[2])) {
                todo_list[pos].mark_completed();
                println!("Marked task '{}' as complete!", args[2]);
                if let Err(e) = storage.save_todos(&todo_list) {
                    eprintln!("Error saving todos: {}", e);
                }
            } else {
                println!("Task not found: {}", args[2]);
            }
        },
        "remove" => {
            if args.len() < 3 {
                println!("Usage: todo remove <task_id or task_name>");
                return;
            }
            if let Ok(id) = args[2].parse::<usize>() {
                if id > 0 && id <= todo_list.len() {
                    todo_list.remove(id - 1);
                    println!("Removed task {}!", id);
                    if let Err(e) = storage.save_todos(&todo_list) {
                        eprintln!("Error saving todos: {}", e);
                    }
                    return;
                }
            }
            println!("Task not found: {}", args[2]);
        },
        "restore" => {
            match storage.restore_from_backup() {
                Ok(restored_list) => {
                    todo_list = restored_list;
                    println!("Successfully restored from backup!");
                    if let Err(e) = storage.save_todos(&todo_list) {
                        eprintln!("Error saving restored todos: {}", e);
                    }
                    display_tasks(&todo_list);
                },
                Err(e) => eprintln!("Error restoring from backup: {}", e),
            }
        },
        _ => display_tasks(&todo_list),
    }
}

fn display_tasks(todo_list: &[TodoItem]) {
    if todo_list.is_empty() {
        println!("No tasks in the list.");
        return;
    }
    
    println!("\nTodo List:");
    for (index, item) in todo_list.iter().enumerate() {
        let status = if item.is_completed() { "✓" } else { " " };
        println!("{}. [{}] {}", index + 1, status, item.description());
    }
}
