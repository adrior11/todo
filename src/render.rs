use colored::*;
use crate::config::Config;
use crate::todo::Todo;

/// Renders a single todo item based on configuration settings.
pub fn render_todo(todo: &Todo, _config: &Config, max_indent_count: usize) {
    let indent = " ".repeat(max_indent_count - todo.id.to_string().len());
    let bold_id = { todo.id.to_string() + "." }.dimmed();

    if todo.done {
        println!(" {} {} {} {}", indent, bold_id, "[X]".purple(), todo.desc.dimmed());
    } else {
        println!(" {} {} [ ] {}", indent, bold_id, todo.desc);
    }
}

/// Renders the list of todos.
pub fn render_todo_list(todos: &[Todo], config: &Config) {
    let max_indent_count = todos.iter()
        .map(|todo| todo.id)
        .max()
        .unwrap_or(0)
        .to_string()
        .len();

    let todos_done = todos.iter().filter(|todo| todo.done).count();
    let todos_total = todos.len();
    let status_string = format!("[{}/{}]", todos_done, todos_total).dimmed();

    let title = "Your todos:".underline();
    println!("{} {}", title, status_string);

    for todo in todos {
        render_todo(todo, config, max_indent_count);
    }

    let todos_completed = if todos_total > 0 {
        (todos_done * 100) / todos_total
    } else {
        0
    };

    let completed_string = format!("\n{}% of all todos complete", todos_completed).dimmed();
    println!("{}", completed_string);
}

