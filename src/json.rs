use crate::todo::TodoItem;

pub fn parse_json(input: &str) -> Vec<TodoItem> {
    let mut todos = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '[' { break; }
    }
    
    while let Some(&c) = chars.peek() {
        if c == ']' { break; }
        if c == ',' { chars.next(); continue; }
        if c.is_whitespace() { chars.next(); continue; }
        
        if c == '{' {
            chars.next();
            let mut description = String::new();
            let mut completed = false;
            
            while let Some(&c) = chars.peek() {
                if c == '}' { chars.next(); break; }
                if c.is_whitespace() { chars.next(); continue; }
                if c == ',' { chars.next(); continue; }
                
                if c == '"' {
                    chars.next();
                    let mut field = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == '"' { chars.next(); break; }
                        field.push(chars.next().unwrap());
                    }
                    
                    while let Some(&c) = chars.peek() {
                        if c == ':' { chars.next(); break; }
                        chars.next();
                    }
                    
                    while let Some(&c) = chars.peek() {
                        if c.is_whitespace() { chars.next(); continue; }
                        
                        match field.as_str() {
                            "description" => {
                                if c == '"' {
                                    chars.next();
                                    while let Some(&c) = chars.peek() {
                                        if c == '"' { chars.next(); break; }
                                        description.push(chars.next().unwrap());
                                    }
                                }
                            },
                            "completed" => {
                                let mut value = String::new();
                                while let Some(&c) = chars.peek() {
                                    if c == ',' || c == '}' { break; }
                                    if !c.is_whitespace() {
                                        value.push(chars.next().unwrap());
                                    } else {
                                        chars.next();
                                    }
                                }
                                completed = value == "true";
                            },
                            _ => {
                                while let Some(&c) = chars.peek() {
                                    if c == ',' || c == '}' { break; }
                                    chars.next();
                                }
                            }
                        }
                        break;
                    }
                }
            }
            
            if !description.is_empty() {
                todos.push(TodoItem::new(&description));
                if completed {
                    todos.last_mut().unwrap().mark_completed();
                }
            }
        }
    }
    
    todos
}

pub fn to_json(todos: &[TodoItem]) -> String {
    let mut json = String::from("[\n");
    
    for (i, todo) in todos.iter().enumerate() {
        json.push_str("  {\n");
        json.push_str(&format!("    \"description\": \"{}\",\n", 
            todo.description().replace('\"', "\\\"")));
        json.push_str(&format!("    \"completed\": {}\n", todo.is_completed()));
        json.push_str("  }");
        if i < todos.len() - 1 {
            json.push(',');
        }
        json.push('\n');
    }
    
    json.push_str("]\n");
    json
} 