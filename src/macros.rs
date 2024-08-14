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

