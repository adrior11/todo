use colored::*;
use chrono::{DateTime, Utc}; 
use crate::config::Config;
use crate::todo::Todo;

/// Renders a single todo item based on configuration settings.
pub fn render_todo(todo: &Todo, _config: &Config, max_indent_count: usize) {
    let indent = " ".repeat(max_indent_count - todo.id.to_string().len());
    let id_display = format!("{}.", todo.id).dimmed();

    let status = if todo.is_complete {
        "[✔]".dimmed()
    } else {
        "[ ]".normal()
    };

    let description = if todo.is_complete {
        todo.desc.dimmed()
    } else {
        format!("{} {}", todo.desc, days_since(todo.timestamp).dimmed()).normal()
    };

    let star = if todo.is_starred {
        "􀆿".yellow()
    } else {
        "".normal()
    };

    println!(" {} {} {} {} {}", indent, id_display, status, description, star);
}

/// Renders the list of todos.
pub fn render_todo_list(todos: &[&Todo], config: &Config) {
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

/// Calculates the number of days since the given date.
fn days_since(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now - date;
    format!("{}d", diff.num_days())
}

/// Formats a summary of the todo list's completion status.
fn format_status_summary(todos: &[&Todo]) -> ColoredString {
    let done_count = todos.iter().filter(|t| t.is_complete).count();
    format!("[{}/{}]", done_count, todos.len()).dimmed()
}

/// Calculates the completion rate of the todo list.
fn calculate_completion_rate(todos: &[&Todo]) -> usize {
    if todos.is_empty() {
        return 0;
    }
    let done_count = todos.iter().filter(|t| t.is_complete).count();
    100 * done_count / todos.len()
}
