# Rust CLI Todo Application

A simple and flexible command-line todo application built with Rust. 
This application allows you to manage your todo lists directly from the terminal, with support for multiple todo items in a single command.

## ✨ Features

- Add, list, star, mark as done, and remove todo items
- Support for multiple todo items in a single command using `::` as a delimiter
- Filter todo items using a query or multiple queries separated by `::`
- Sort todos by ID, creation date, or completion status
- Reset the entire todo list
- Create, manage, and access backup files
- User-configurable options via a Lua file 

## 🛠️ Planned Features

- [X] Implement a backup solution to prevent accidental deletion of todos
- [X] Add a feature to edit existing todo items
- [X] Add restore command for backup retrieval
- [X] Implement a filter for todo items
- [ ] Implement multiple todo lists using boards
- [ ] Improve the user interface
- [ ] Add further configuration options
- [ ] Add due dates and reminders for todo items

## ⚙️ Installation

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

## 🚀 Usage 

```sh
$ todo --help

  Usage
    $ todo [<options> ...]

    Options
        none              List all todos (default action)
      --list, -l          List all todos
      --add, -a           Add a new todo item(s)
      --edit, -e          Edit an existing todo item
      --filter, -f        Filter todos by a query string
      --done, -d          Mark a todo(s) as done
      --undone, -u        Mark a todo(s) as not done
      --star, -s          Star/Unstar a todo item(s)
      --rm, -r            Remove a todo item(s)
      --reset             Reset the todo list
      --sort, -S          Sort todos by specified criteria [possible values: id, date, done]
      --backup, -b        Backup and manage todo backups

    Backup Options
        none              List all backups (default action)
        create, c         Create a new backup
        open, o           Open a backup to view its contents
        delete, D         Delete backups
        restore, R        Restore a todo item(s) from a backup
        list, l           List all backups

    Delete Options
        all, A            Delete all backups
        timestamp, t      Delete a specific backup by timestamp

    Examples
      $ todo
      $ todo --list
      $ todo --add Hello World!
      $ todo --add Buy milk::Clean the house::Water plants
      $ todo --edit 1 Buy almond milk
      $ todo --filter plants
      $ todo --filter plants::milk
      $ todo --done 1 2 3
      $ todo --undone 1 2 3
      $ todo --star 1 2 3
      $ todo --rm 1 2 3
      $ todo --reset
      $ todo --sort
      $ todo --sort id
      $ todo --backup
      $ todo --backup list
      $ todo --backup create
      $ todo --backup open 1723823802
      $ todo --backup restore 1723823802 1 2
      $ todo --backup delete all
      $ todo --backup delete timestamp 1723823802
```

## 🔧 Configuration

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

```lua
config = { 
    backup_on_reset = true,
}
```

You can edit this file to customize the behavior of the application.

## 📝️ Command Details

### List all todos (default)
```sh
todo list
```

### Add a new todo

Add one or more todos, separated by `::`.
Note that leading and trailing spaces in the todo descriptions will be automatically removed, so a description like `  Buy milk :: Water plants  ` will be treated as `Buy milk` & `Water plants`.

```sh
todo add [TODO_DESCRIPTION]...

# Example:
todo add Hello World!
todo add Buy milk::Clean the house::Water plants
```

### Edit an existing todo item
```sh 
todo edit <TODO_ID> <NEW_DESCRIPTION>

# Example:
todo edit 1 Buy almond milk
```

### Filter todo items

Filter your todo list by a specific query or multiple queries separated by `::`.
Note that leading and trailing spaces in the queries will be automatically removed, so a query like `  Sweet  ` will match both `HomeSweetHome` and `Home Sweet Home`.

```sh
todo filter [QUERY]...

# Example:
todo filter plants
todo filter plants::milk
```

### Mark a todo as done

Mark one or more todos as done by their IDs.

```sh
todo done [TODO_ID]...

# Example:
todo done 1 2 3
```

### Mark a todo as not done

Mark one or more todos as not done by their IDs.

```sh
todo undone [TODO_ID]...

# Example:
todo undone 1 2 3
```

### Star todo items 

Highlight one or more important todos by marking them as `star`. If an item is already starred, running this command again on the same ID will `unstar` it, effectively toggling the star status.

```sh
todo star [TODO_ID]...

# Example:
todo star 1 2 3
```

### Remove a todo

Remove one or more todos by their IDs.

```sh
todo rm [TODO_ID]...

# Example:
todo rm 1 2 3
```

### Reset the todo list

Reset a todo list, clearing all todos. This action will automatically create a backup file.

```sh 
todo reset
```

### Sort todos

Sort your todos with an optional sorting rule.

```sh
# Possible values:
# - id
# - date
# - done (default)

todo sort [SORT_BY]...

# Example:
todo sort date
```

## 💾 Backup Commands

### List all backups (default)

```sh 
todo backup list
```

### Create a new backup 

```sh 
todo backup create
```

### Show the contents of a specific backup file by its timestamp.

```sh 
todo backup open <TIMESTAMP>

# Example:
todo backup open 1723823802
```

### Restore todo items from a backup

```sh
todo backup restore <TIMESTAMP> [TODO_ID]...

# Example:
todo backup restore 1723065962 1 2 3
```

### Delete existing backups 

#### Delete all backups 

```sh 
todo backup delete all 
```

#### Delete a specific backup by its timestamp 

```sh 
todo backup delete timestamp <TIMESTAMP>

# Example:
todo backup delete timestamp 1723823802
```
