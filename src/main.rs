mod raw;
use raw::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug: bool = false;
    let mut file_name: String = "".to_string();
    if args.len() < 2 {
        println!("Please provide app name as an arg");
        std::process::exit(1);
    }
    for arg_index in 0..args.len() {
        if arg_index > 0 {
            if args[arg_index] == "-d" {
                debug = true;
            } else {
                file_name = args[arg_index].clone();
            }
        }
    }

    let tokens = generate_tokens(read_code(file_name));
    if debug {println!("\x1b[34m\x1b[1mParsed code\x1b[0m: {:?}", tokens);}

    let mut stack: Vec<(i32, i32, String)> = vec!();
    for i in 0..tokens.len() {
        if tokens[i].0 == TOKEN_INT.0 {
            if let Ok(int) = tokens[i].1.parse::<i32>() {
                stack.push((0, int, "".to_string()));
            }
        } else if tokens[i].0 == TOKEN_PLUS.0 {
            if stack.len() < 2 {
                error("Stack error", "Trying to get 2 elements from the stack to add them, but stack doen't have enought elements.");
            } else {
                if let Some((_, a, _)) = stack.pop() {
                    if let Some((_, b, _)) = stack.pop() {
                        stack.push((0, a + b, "".to_string()));
                    } else {
                        error("Stack error", "Unable to pop B from the stack to do the add operation.")
                    }
                } else {
                    error("Stack error", "Unable to pop A from the stack to do the add operation.")
                }
            }
        } else if tokens[i].0 == TOKEN_MINUS.0 {
            if stack.len() < 2 {
                error("Stack error", "Trying to get 2 elements from the stack to subtract them, but stack doen't have enought elements.");
            } else {
                if let Some((_, a, _)) = stack.pop() {
                    if let Some((_, b, _)) = stack.pop() {
                        stack.push((0, a - b, "".to_string()));
                    } else {
                        error("Stack error", "Unable to pop B from the stack to do the minus operation.")
                    }
                } else {
                    error("Stack error", "Unable to pop A from the stack to do the minus operation.")
                }
            }
        } else if tokens[i].0 == TOKEN_PRINT.0 {
            print!("\x1b[37m\x1b[1mStack\x1b[0m: [");
            for j in 0..stack.len() {
                if j < stack.len() - 1 {
                    print!("{}, ", stack[j].1);
                } else {
                    print!("{}", stack[j].1);
                }
            }
            print!("]\n");
        } else if tokens[i].0 == TOKEN_CLEAR.0 {
            if debug {
                println!("\x1b[34m\x1b[1mClearing stack\x1b[0m.")
            }
            stack = vec!();
        } else if tokens[i].0 == TOKEN_DUMP.0 {
            if stack.len() < 1 {
                error("Stack error", "Trying to dump element from the stack, but stack doesn't have enought elements.");
            } else {
                if let Some((_, int, _)) = stack.pop() {
                    println!("{}", int)
                } else {
                    error("Stack error", "Unable to dump element from the stack.")
                }
            }
        } else {
            error("Interpretation error", format!("Unexpected syntax, has been parsed, but can't be interpreted: {}", tokens[i].0).as_str());
        }
    }
}

fn error(error: &str, text: &str) {
    println!("\x1b[31m\x1b[1m{}: \x1b[0m\x1b[31m{}\x1b[0m", error, text);
    std::process::exit(1);
}
