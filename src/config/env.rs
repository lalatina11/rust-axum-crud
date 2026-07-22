use std::env;

use dotenvy::dotenv;

pub fn get_env_var(env_var: &str) -> String {
    // Load .env file (call this once, typically in main.rs)
    dotenv().ok();
    // Read the specific variable
    env::var(env_var).expect(&format!(
        "Error loading env variable!\nCannot find {} in .env",
        env_var
    ))
}

pub fn get_database_url() -> String {
    get_env_var("DATABASE_URL")
}
