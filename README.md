# 📝 Rust ToDo CLI
![License](https://img.shields.io/github/license/Niall-Sharma/RustCliTodo)
![Issues](https://img.shields.io/github/issues/Niall-Sharma/RustCliTodo)
![Forks](https://img.shields.io/github/forks/Niall-Sharma/RustCliTodo?style=social)
![Stars](https://img.shields.io/github/stars/Niall-Sharma/RustCliTodo?style=social)
![Last Commit](https://img.shields.io/github/last-commit/Niall-Sharma/RustCliTodo)
![Build Status](https://github.com/Niall-Sharma/RustCliTodo/actions/workflows/rust.yml/badge.svg)

A simple command-line to-do list application written in Rust. Manage your tasks directly from your terminal—add, list, complete, and remove them with ease!

## 🚀 Features

- Add tasks to list
- List all tasks with their status (done/undone)
- Mark tasks as done
- Delete tasks by task number
- Save tasks to a local JSON file

## 📦 Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/RustCliTodo.git
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

## 🤝 Contributing

Contributions are welcome! If you have an idea to improve the CLI, fix a bug, or add a feature, feel free to jump in.

1. Fork the repository
2. Create a new branch:

    ```bash
    git checkout -b your-feature
    ```

3. Make your changes

4. Commit your changes:

    ```bash
    git commit -m "Add some feature"
    ```

5. Push to your branch:

    ```bash
    git push origin your-feature
    ```

6. Open a Pull Request

Whether it's code, feedback, or ideas—every bit helps.  
Thanks for helping make this tiny tool better!
