
# Rust Todo Application

This project is a command-line Todo application written in Rust. It allows users to add, view, update, and delete tasks, as well as mark tasks as completed. This application is structured in a modular way, with task management logic abstracted into a library.

## Features

- **Add Task:** Add a new task to the todo list.
- **View Tasks:** Display all tasks with their status (done or pending).
- **Update Task:** Modify the name of an existing task.
- **Mark as Done:** Mark a task as completed.
- **Delete Task:** Remove a task from the list.
- **Help Command:** Display available commands and usage.
- **Exit:** Quit the application.

## Usage

Once you run the application, you can input commands using the following format:

```
command [arguments]
```

### Supported Commands

- **add `task_name`**
  - Adds a new task to the todo list.
  - Example: `add BuyMilk`

- **show**
  - Displays all tasks in the todo list.
  - Example: `show`

- **delete `task_id`**
  - Deletes a task by its ID.
  - Example: `delete 1`

- **update `task_id` `new_task_name`**
  - Updates the name of a task by its ID.
  - Example: `update 1 GoForWalk`

- **done `task_id`**
  - Marks a task as completed by its ID.
  - Example: `done 1`

- **exit**
  - Exits the application.
  - Example: `exit`

- **help**
  - Displays a help message with all available commands.
  - Example: `help`

## How to Run

1. Clone this repository:

    ```bash
    git clone https://github.com/yourusername/todo-app.git
    cd todo-app
    ```

2. Build the project:

    ```bash
    cargo build
    ```

3. Run the application:

    ```bash
    cargo run
    ```

4. Use the interactive command-line prompt to manage your tasks.

## Example Interaction

```text
(todo list) > add ReadBook
Task ReadBook added successfully
id: 1,  name: ReadBook,  done: false
(todo list) > done 1
(todo list) > show
id: 1,  name: ReadBook,  done: true
(todo list) > delete 1
(todo list) > exit
```

## Project Structure

- `main.rs`: Contains the entry point and the command-line prompt loop.
- `lib.rs`: Implements the core logic of the todo list, including task management and command parsing.

## License

This project is licensed under the MIT License.
