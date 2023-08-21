use std::fs::File;
use std::io::Read;
use regex::Regex;

pub const TOKEN_NUM: (i32, &str) = (1, "");
pub const TOKEN_PLUS:(i32, &str) = (2, "+");
pub const TOKEN_MINUS: (i32, &str) = (3, "-");
pub const TOKEN_DUMP: (i32, &str) = (4, ".");
pub const TOKEN_PRINT: (i32, &str) = (5, "!");
pub const TOKEN_CLEAR: (i32, &str) = (6, "§");
pub const TOKEN_MULTIPLY: (i32, &str) = (7, "*");
pub const TOKEN_DIVIDE: (i32, &str) = (8, "/");
pub const TOKEN_BIT_MOVE_LEFT: (i32, &str) = (9, "<<");
pub const TOKEN_BIT_MOVE_RIGTH: (i32, &str) = (10, ">>");
pub const TOKEN_POWER: (i32, &str) = (11, "**");
pub const TOKEN_ROOT: (i32, &str) = (12, "^");
pub const TOKEN_ROUND: (i32, &str) = (13, "rond");
pub const TOKEN_FLOOR: (i32, &str) = (14, "flr");

pub fn generate_tokens(code: String) -> Vec<(i32, String)> {
    let splitted = code.split(" ");
    let mut result: Vec<(i32, String)> = vec!();
    for split in splitted {
        if let Ok(int) = split.parse::<f32>() {
            result.push((TOKEN_NUM.0, int.to_string()));
        } else {
            if      split == TOKEN_PLUS.1 {result.push((TOKEN_PLUS.0,  "".to_string()));}
            else if split == TOKEN_MINUS.1 {result.push((TOKEN_MINUS.0, "".to_string()));}
            else if split == TOKEN_DUMP.1 {result.push((TOKEN_DUMP.0,  "".to_string()));}
            else if split == TOKEN_PRINT.1 {result.push((TOKEN_PRINT.0, "".to_string()));}
            else if split == TOKEN_CLEAR.1 {result.push((TOKEN_CLEAR.0, "".to_string()));}
            else if split == TOKEN_MULTIPLY.1 {result.push((TOKEN_MULTIPLY.0, "".to_string()));}
            else if split == TOKEN_DIVIDE.1 {result.push((TOKEN_DIVIDE.0, "".to_string()));}
            else if split == TOKEN_BIT_MOVE_LEFT.1 {result.push((TOKEN_BIT_MOVE_LEFT.0, "".to_string()));}
            else if split == TOKEN_BIT_MOVE_RIGTH.1 {result.push((TOKEN_BIT_MOVE_RIGTH.0, "".to_string()));}
            else if split == TOKEN_POWER.1 {result.push((TOKEN_POWER.0, "".to_string()));}
            else if split == TOKEN_ROOT.1 {result.push((TOKEN_ROOT.0, "".to_string()));}
            else if split == TOKEN_ROUND.1 {result.push((TOKEN_ROUND.0, "".to_string()));}
            else if split == TOKEN_FLOOR.1 {result.push((TOKEN_FLOOR.0, "".to_string()));}
            else {
                println!("\x1b[31m\x1b[1m{}: \x1b[0m\x1b[31m{}\x1b[0m", "Syntax error", format!("Parsing failed: `{}`", split).as_str());
                std::process::exit(1);
            }
        }
    }
    return result;
}

pub fn read_code(name: String) -> String {
    let mut code: String = read_file_as_string(name);
    code = clear_comments(code);
    code = delete_new_lines(code);
    code = remove_useless_tabs(code);
    code = remove_useless_spaces(code);
    return code;
}

pub fn read_file_as_string(name: String) -> String {
    let mut file = File::open(name).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    return contents;
}

pub fn clear_comments(data: String) -> String {
    let re = Regex::new(r"//.*\n").unwrap();
    let result = re.replace_all(&data, "");
    return result.to_string();
}

pub fn delete_new_lines(data: String) -> String {
    return data.replace("\n", " ");
}

pub fn remove_useless_spaces(data: String) -> String {
    let re = Regex::new(r"\s+").unwrap();
    let replaced = re.replace_all(&data, " ");
    let result: &str;
    if replaced.ends_with(' ') {
        let mut chars = replaced.chars();
        chars.next_back();
        result = chars.as_str();
    } else {
        result = &replaced;
    }
    return result.to_string();
}

pub fn remove_useless_tabs(data: String) -> String {
    let re = Regex::new(r"\t+").unwrap();
    let result = re.replace_all(&data, " ");
    return result.to_string();
}
