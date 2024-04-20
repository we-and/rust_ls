use std::env;
use crate::constants::default_output_width;
pub fn get_env_columns() -> usize {
    // Check if an environment variable for width is set
    env::var("COLUMNS")
        .ok()
        .and_then(|c| c.parse::<usize>().ok())
        .unwrap_or(default_output_width)  // Default to 80 if not set
}
