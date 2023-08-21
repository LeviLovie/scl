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
pub const TOKEN_EXIT: (i32, &str) = (15, "exit");
pub const TOKEN_IF: (i32, &str) = (16, "if");
pub const TOKEN_FI: (i32, &str) = (17, "fi");
pub const TOKEN_SWAP: (i32, &str) = (19, "swap");
pub const TOKEN_REPEAT: (i32, &str) = (20, "repeat");
pub const TOKEN_REPEAT_END: (i32, &str) = (21, "rend");

pub const TOKENS_TO_BE_NESTED: [&(i32, &str); 2] = [&TOKEN_IF, &TOKEN_REPEAT];
pub const TOKENS_TO_BE_UNNESTED: [&(i32, &str); 2] = [&TOKEN_FI, &TOKEN_REPEAT_END];

pub fn generate_tokens(code: String) -> Vec<(i32, String, i32)> {
    let splitted = code.split(" ");
    let mut result: Vec<(i32, String, i32)> = vec!();
    let mut iterator = 0;
    let mut repeat_ends: Vec<i32> = vec!();
    while iterator < splitted.clone().count() {
        let split = splitted.clone().nth(iterator).unwrap().to_string();
        if let Ok(int) = split.parse::<f32>() {
            result.push((TOKEN_NUM.0, int.to_string(), 0));
        } else {
            if      split == TOKEN_PLUS.1 {result.push((TOKEN_PLUS.0,  "".to_string(), 0));}
            else if split == TOKEN_MINUS.1 {result.push((TOKEN_MINUS.0, "".to_string(), 0));}
            else if split == TOKEN_DUMP.1 {result.push((TOKEN_DUMP.0,  "".to_string(), 0));}
            else if split == TOKEN_PRINT.1 {result.push((TOKEN_PRINT.0, "".to_string(), 0));}
            else if split == TOKEN_CLEAR.1 {result.push((TOKEN_CLEAR.0, "".to_string(), 0));}
            else if split == TOKEN_MULTIPLY.1 {result.push((TOKEN_MULTIPLY.0, "".to_string(), 0));}
            else if split == TOKEN_DIVIDE.1 {result.push((TOKEN_DIVIDE.0, "".to_string(), 0));}
            else if split == TOKEN_BIT_MOVE_LEFT.1 {result.push((TOKEN_BIT_MOVE_LEFT.0, "".to_string(), 0));}
            else if split == TOKEN_BIT_MOVE_RIGTH.1 {result.push((TOKEN_BIT_MOVE_RIGTH.0, "".to_string(), 0));}
            else if split == TOKEN_POWER.1 {result.push((TOKEN_POWER.0, "".to_string(), 0));}
            else if split == TOKEN_ROOT.1 {result.push((TOKEN_ROOT.0, "".to_string(), 0));}
            else if split == TOKEN_ROUND.1 {result.push((TOKEN_ROUND.0, "".to_string(), 0));}
            else if split == TOKEN_FLOOR.1 {result.push((TOKEN_FLOOR.0, "".to_string(), 0));}
            else if split == TOKEN_EXIT.1 {result.push((TOKEN_EXIT.0, "".to_string(), 0));}
            else if split == TOKEN_IF.1 {
                let mut fi_index = iterator;
                let mut fi_found = false;
                let mut depth = 0;
                while !fi_found {
                    if fi_index >= splitted.clone().count() {
                        error("Structures syntax error", format!("Missing {}", TOKEN_FI.1).as_str());
                    }
                    for variation_index in 0..TOKENS_TO_BE_NESTED.len() {
                        if splitted.clone().nth(fi_index).unwrap() == TOKENS_TO_BE_NESTED[variation_index].1 {
                            depth += 1;
                        } else if splitted.clone().nth(fi_index).unwrap() == TOKENS_TO_BE_UNNESTED[variation_index].1 {
                            if depth != 0 {depth -= 1;}
                        }
                    }
                    if depth < 0 {
                        error("Structures syntax error", format!("Error with founding {}", TOKEN_FI.1).as_str());
                    } else if depth == 0 {fi_found = true;}
                    fi_index += 1;
                }
                result.push((TOKEN_IF.0, format!("{}", fi_index - 1), 0));
            }
            else if split == TOKEN_SWAP.1 {result.push((TOKEN_SWAP.0, "".to_string(), 0));}
            else if split == TOKEN_REPEAT.1 {
                let mut end_index = iterator;
                let mut end_found = false;
                let mut depth = 0;
                while !end_found {
                    if end_index >= splitted.clone().count() {
                        error("Structures syntax error", format!("Missing {}", TOKEN_FI.1).as_str());
                    }
                    for variation_index in 0..TOKENS_TO_BE_NESTED.len() {
                        if splitted.clone().nth(end_index).unwrap() == TOKENS_TO_BE_NESTED[variation_index].1 {
                            depth += 1;
                        } else if splitted.clone().nth(end_index).unwrap() == TOKENS_TO_BE_UNNESTED[variation_index].1 {
                            if depth != 0 {depth -= 1;}
                        }
                    }
                    if depth < 0 {
                        error("Structures syntax error", format!("Error with founding {}", TOKEN_FI.1).as_str());
                    } else if depth == 0 {end_found = true;}
                    end_index += 1;
                }
                result.push((TOKEN_REPEAT.0, format!("{}", -1), -1));
                repeat_ends.push(iterator as i32);
            }
            else if split == TOKEN_REPEAT_END.1 {
                if repeat_ends.len() <= 0 {
                    error("Structures syntax error", "Can't found stact of repeat for it's end");
                } else {
                    result.push((TOKEN_REPEAT_END.0, format!("{}", repeat_ends.pop().unwrap()), 0));
                }
            }
            else {
                if split != TOKEN_FI.1 {
                    println!("\x1b[31m\x1b[1m{}: \x1b[0m\x1b[31m{}\x1b[0m", "Syntax error", format!("Parsing failed: `{}`", split).as_str());
                    std::process::exit(1);
                }
            }
        }
        iterator += 1;
    }
    return result;
}

fn error(error: &str, text: &str) {
    println!("\x1b[31m\x1b[1m{}: \x1b[0m\x1b[31m{}\x1b[0m", error, text);
    std::process::exit(1);
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
