use std::{env, fs};

pub fn load_file_from_args() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();

    let path = args
        .get(1)
        .ok_or_else(|| "Please provide a file".to_string())?;

    fs::read_to_string(path).map_err(|e| format!("failed to read file:{e}"))
}
