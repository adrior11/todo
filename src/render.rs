use colored::*;
use chrono::{DateTime, Utc}; 
use crate::config::Config;
use crate::todo::Todo;

/// Renders a single todo item based on configuration settings.
pub fn render_todo(todo: &Todo, _config: &Config, max_indent_count: usize) {
    let indent = " ".repeat(max_indent_count - todo.id.to_string().len());
    let id_display = format!("{}.", todo.id).dimmed();

    let status = if todo.done {
        "[X]".dimmed()
    } else {
        "[ ]".normal()
    };

    let description = if todo.done {
        todo.desc.dimmed()
    } else {
        format!("{} {}", todo.desc, days_since(todo.created_at).dimmed()).normal()
    };

    println!(" {} {} {} {}", indent, id_display, status, description);
}

/// Renders the list of todos.
pub fn render_todo_list(todos: &[Todo], config: &Config) {
    let max_id_width = todos.iter()
        .map(|todo| todo.id)
        .max()
        .unwrap_or(0)
        .to_string()
        .len();

    let status_summary = format_status_summary(todos);
    let title = "Your todos:".underline();
    println!("{} {}", title, status_summary);

    for todo in todos {
        render_todo(todo, config, max_id_width);
    }

    let completion_rate = calculate_completion_rate(todos);
    let completed_string = format!("\n{}% of all todos complete!", completion_rate).dimmed();
    println!("{}", completed_string);
}

fn days_since(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now - date;
    format!("{}d", diff.num_days())
}

fn format_status_summary(todos: &[Todo]) -> ColoredString {
    let done_count = todos.iter().filter(|t| t.done).count();
    format!("[{}/{}]", done_count, todos.len()).dimmed()
}

fn calculate_completion_rate(todos: &[Todo]) -> usize {
    if todos.is_empty() {
        return 0;
    }
    let done_count = todos.iter().filter(|t| t.done).count();
    100 * done_count / todos.len()
}
