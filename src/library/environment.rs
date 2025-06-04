use std::env;

#[doc = "read an shell env var and return it"]
pub fn read_env_var(var_name: &str) -> String {
    let value = env::var(&var_name).expect(&format!("Couldn't read env var {}", var_name));

    value
}
