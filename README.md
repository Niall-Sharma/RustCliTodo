# 📝 Rust ToDo CLI

A simple command-line to-do list application written in Rust. Manage your tasks directly from your terminal—add, list, complete, and remove them with ease!

## 🚀 Features

- Add tasks with a description
- List all tasks with their status (done/undone)
- Mark tasks as done
- Delete tasks by ID
- Save tasks to a local JSON file

## 📦 Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/rust-todo-cli.git
    cd rust-todo-cli
    ```

2. Build the project using Cargo:

    ```bash
    cargo build --release
    ```

3. Run the app:

    ```bash
    cargo run -- [COMMAND]
    ```

    Or use the compiled binary in `target/release`.

## ⚙️ Usage

```bash
cli-todolist add "<TASK NAME>" #adds task to list
cli-todolist remove <TASK ID> #removes task from list
cli-todolist done <TASK ID> #marks task as completed
cli-todolist list #lists all tasks in todo list
```
Told you it was simple...
