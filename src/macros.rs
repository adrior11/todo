/// Generates a Lua configuration string from a list of key-value pairs.
///
/// # Parameters
///
/// - `$key`: The key for the configuration option, which should be a string.
/// - `$value`: The value for the configuration option, which will be formatted into the Lua string. 
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate todo;
/// let lua_config = generate_lua_config!(
///     "first_option" => true,
///     "second_option" => 42,
/// );
/// # assert_eq!(lua_config, "config = {\n    first_option = true,\n    second_option = 42,\n}\n");
/// ```
#[macro_export]
macro_rules! generate_lua_config {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut lua_config = String::from("config = {\n");
            $(
                lua_config.push_str(&format!("    {} = {},\n", $key, $value));
            )*
            lua_config.push_str("}\n");
            lua_config
        }
    };
}

/// Retrieves a value from a Lua configuration table with a fallback to a default value.
///
/// This macro attempts to get a value associated with a key from a Lua table. 
/// If the key does not exist or if the retrieval fails, it returns a provided default value.
///
/// # Parameters
///
/// - `$config`: The Lua table from which to retrieve the value.
/// - `$key`: The key for the configuration option.
/// - `$default`: The default value to return if the key does not exist or retrieval fails.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate todo;
/// let first_option: bool = get_config_value!(config, "first_option", Config::default().first_option);
/// let second_option: bool = get_config_value!(config, "second_option", Config::default().second_option);
/// ```
#[macro_export]
macro_rules! get_config_value {
    ($config:expr, $key:expr, $default:expr) => {
        if let Ok(value) = $config.get::<_, Option<bool>>($key) {
            value.unwrap_or($default)
        } else {
            $default
        }
    };
}

/// Applies an action to a list of todo items identified by their IDs.
///
/// This macro iterates over a list of IDs, finds the corresponding todo items in the `TodoList`,
/// and applies the specified action to each item. If any ID does not match a todo item, the macro 
/// returns an error.
///
/// # Parameters
///
/// - `$self`: A mutable reference to the `TodoList` struct containing the todos.
/// - `$ids`: An expression that evaluates to a list of IDs (`Vec<usize>`) of the todo items to be modified.
/// - `$action`: An expression that specifies the action to be performed on each matched todo item. 
///   This expression is passed a mutable reference to the matched `Todo` struct.
///
/// # Returns
///
/// - `Result<()>`: Returns `Ok(())` if all todo items are successfully modified. If any ID is not found,
///   returns an `Err` containing a message with the missing ID.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate todo;
/// # use todo::{TodoList, Todo};
/// # use anyhow::Result;
/// #
/// # fn main() -> Result<()> {
/// #     let mut todo_list = TodoList::default();
/// #     todo_list.add(vec!["Task 1".to_string()]);
/// #     todo_list.add(vec!["Task 2".to_string()]);
/// #     let ids = vec![1, 2];
///     
///     // Mark todos as complete
///     modify_todos!(todo_list, ids, |todo: &mut Todo| {
///         todo.is_complete = true;
///     });
///
/// #     Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! modify_todos {
    ($self:ident, $ids:expr, $action:expr) => {
        for id in $ids {
            if let Some(todo) = $self.todos.iter_mut().find(|todo| todo.id == id) {
                $action(todo);
            } else {
                return Err(anyhow!("ID {} not found", id));
            }
        }
    };
}

/// Toggles the value of a boolean variable.
///
/// # Parameters
///
/// - `$var`: A mutable reference to a boolean variable that will be toggled.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate todo;
/// let mut my_bool = true;
/// toggle_bool!(my_bool);
/// # assert_eq!(my_bool, false);
/// toggle_bool!(my_bool);
/// # assert_eq!(my_bool, true);
/// ```
#[macro_export]
macro_rules! toggle_bool {
    ($var:expr) => {
        $var = !$var;
    };
}

