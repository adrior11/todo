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

