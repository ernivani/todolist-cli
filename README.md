# Rust Todo List CLI

A simple command-line todo list manager written in Rust. Features include task management, automatic backups, and file persistence.

## Features

- Add multiple tasks
- Mark tasks as completed
- Remove tasks
- Restore from backup
- Persistent storage in JSON format
- Automatic backups
- Safe file operations

## Installation

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone this repository:
```bash
git clone https://github.com/ernivani/todolist
cd todolist
```
3. Build the project:
```bash
cargo build --release
```
4. Install the binary:
```bash
# Create local bin directory if it doesn't exist
mkdir -p ~/.local/bin
# Copy the binary
cp target/release/todolist ~/.local/bin/todo
# Make it executable
chmod +x ~/.local/bin/todo
# Add to PATH if not already added (add this to your ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"
```

## Usage

```bash
# Show all commands
todo help

# Add tasks
todo add "Buy groceries"
todo add "Call mom" "Pay bills"

# Mark task as completed (by ID or description)
todo done 1
todo done "Buy groceries"

# Remove a task
todo remove 1

# Restore from backup
todo restore

# List all tasks
todo
```

## Storage

Tasks are stored in `~/.todo/todo.json` with automatic backups at `~/.todo/todo.backup.json`.

## License

[MIT License](LICENSE)