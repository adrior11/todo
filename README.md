# Rust CLI Todo Application

A simple and flexible command-line todo application built with Rust. 
This application allows you to manage your todo lists directly from the terminal, with support for multiple todo items in a single command.

## Features

- Add, list, mark as done, and remove todo items
- Support for multiple todo items in a single command using `::` as a delimiter
- Sort todos by ID, creation date, or completion status
- Reset the entire todo list
- Create, manage, and access backup files
- **New**: User-configurable options via a Lua file 

## Planned Features

- [X] Implement a backup solution to prevent accidental deletion of todos
- [X] Add a feature to edit existing todo items
- [ ] Add restore command for backup retrieval
- [ ] Implement a filter for todo items
- [ ] Implement multiple todo lists
- [ ] Improve the user interface
- [ ] Add due dates and reminders for todo items

## Installation

> [!IMPORTANT]  
> There is no official release of this application yet.

To run the application, follow these steps:

1. **Clone the repository**:
    ```sh
    git clone https://github.com/adrior11/todo.git
    cd todo
    ```

2. **Build and run the application using Cargo**:
    ```sh
    cargo run -- <COMMAND> [OPTIONS]
    ```

    For example, to add a new todo:
    ```sh
    cargo run -- add Buy milk::Clean the house::Water plants
    ```

    To list all todos:
    ```sh
    cargo run -- list
    ```

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system before running the available commands.

## Configuration

The application now supports user-configurable options via a Lua configuration file. The configuration file is located at:

```
// Lin: Some(/home/user/.config/todo_app/config.lua)
// Win: Some(C:\Users\user\AppData\Roaming\todo_app\config.lua)
// Mac: Some(/Users/user/Library/Application Support/todo_app/config.lua)
```

### Available Configuration Options

- `backup_on_reset`: Specifies whether a backup should be created automatically when the todo list is reset. (default: `true`)

### Exmaple Configuration 

Here is an example of a `config.lua` file:

```
config = { 
    backup_on_reset = true,
}
```

You can edit this file to customize the behavior of the application.

## Usage

### List all todos (default)
```sh
todo list
```

### Add a new todo

Add one or more todos, separated by `::`.

```sh
todo add <TODO_DESCRIPTION>
```

Example:

```sh
todo add Buy milk::Clean the house::Water plants
```

### Edit an existing todo item
```sh 
todo edit <TODO_ID> <NEW_DESCRIPTION>
```

Example:
```sh 
todo edit 1 Buy almond milk
```

### Mark a todo as done

Mark one or more todos as done by their IDs.

```sh
todo done <TODO_ID>
```

Example:

```sh
todo done 1 2 3
```

### Remove a todo

Remove one or more todos by their IDs.

```sh
todo rm <TODO_ID>
```

Example:
```sh
todo rm 1 2 3
```

### Reset the todo list

Reset a todo list, clearing all todos. This action will automatically create a backup file.

```sh 
todo reset
```

### Backup the todo list

#### Create a new backup 

```sh 
todo backup create
```

#### Show the contents of a specific backup file by its timestamp.

```sh 
todo backup show <TIMESTAMP>
```

#### Delete existing backups 

##### Delete all backups 

```sh 
todo backup delete all 
```

##### Delete a specific backup by timestamp 

```sh 
todo backup delete timestamp <TIMESTAMP>
```

#### List all backups (default)

```sh 
todo backup list
```

### Sort todos

#### Sort by ID 

```sh 
todo sort id
```

#### Sort by creation date

```
todo sort date
```

#### Sort by completion status (default)

```sh 
todo sort done
```

