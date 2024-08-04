# Rust CLI Todo Application

A simple and flexible command-line todo application built with Rust. 
This application allows you to manage your todo lists directly from the terminal, with support for multiple todo items in a single command.

## Features

- Add, list, mark as done, and remove todo items
- Support for multiple todo items in a single command using `::` as a delimiter
- Sort todos by ID, creation date, or completion status
- Reset the entire todo list
- Create, manage, and access backup files

## Planned Features

- [X] Implement a backup solution to prevent accidental deletion of todos
- [X] Add a feature to edit existing todo items
- [ ] Add restore command for backup retrieval
- [ ] Implement a filter for todo items
- [ ] Implement multiple todo lists
- [ ] Improve the user interface
- [ ] Add due dates and reminders for todo items

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

Reset a todo list, clearing all todos. This action will automatically create backup file.

```sh 
todo reset
```

### Backup the todo list

#### Create a new backup 

```sh 
todo backup create
```

#### Show the contents of a backup 

Show the contents of a specific backup file by its timestamp.

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

