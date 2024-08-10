use std::env;
use std::fs;

pub fn commands() -> Vec<String> {
    let path = config_file_path();
    match path {
        Some(s) => parse_commands(s),
        None => Vec::new(),
    }
}

fn parse_commands(path: String) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap_or("".to_string());
    let mut result = Vec::new();

    for line in contents.lines() {
        let com = line.to_string();

        if com.len() > 0 {
            result.push(com);
        }
    }

    return result;
}

fn config_file_path() -> Option<String> {
    let my_var = env::var_os("HOME");

    match my_var {
        Some(value) => {
            let path = value.to_str().unwrap().to_string();
            Some(format!("{}{}", path, "/.config/yaisd/config"))
        }
        None => None,
    }
}
