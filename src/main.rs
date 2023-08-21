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
        if tokens[i].0 == TOKEN_NUM.0 {
            if let Ok(int) = tokens[i].1.parse::<i32>() {
                stack.push((1, int, "".to_string()));
            }
        } else if tokens[i].0 == TOKEN_PLUS.0 {
            let a = get_element_from_stack(&mut stack, "plus").1;
            let b = get_element_from_stack(&mut stack, "plus").1;
            stack.push((1, b + a, "".to_string()));
        } else if tokens[i].0 == TOKEN_MINUS.0 {
            let a = get_element_from_stack(&mut stack, "minus").1;
            let b = get_element_from_stack(&mut stack, "minus").1;
            stack.push((1, b - a, "".to_string()));
        } else if tokens[i].0 == TOKEN_PRINT.0 {
            print!("\x1b[37m\x1b[1mStack\x1b[0m: [");
            for j in 0..stack.len() {
                if j < stack.len() - 1 {print!("{}, ", stack[j].1);
                } else {print!("{}", stack[j].1);}
            }
            print!("]\n");
        } else if tokens[i].0 == TOKEN_CLEAR.0 {
            if debug {
                println!("\x1b[34m\x1b[1mClearing stack\x1b[0m.")
            }
            stack = vec!();
        } else if tokens[i].0 == TOKEN_DUMP.0 {
            let a = get_element_from_stack(&mut stack, "dump").1;
            println!("{}", a)
        } else if tokens[i].0 == TOKEN_MULTIPLY.0 {
            let a = get_element_from_stack(&mut stack, "multiply").1;
            let b = get_element_from_stack(&mut stack, "multiply").1;
            stack.push((1, b * a, "".to_string()));
        } else if tokens[i].0 == TOKEN_DIVIDE.0 {
            let a = get_element_from_stack(&mut stack, "divide").1;
            let b = get_element_from_stack(&mut stack, "divide").1;
            stack.push((1, b / a, "".to_string()));
        } else if tokens[i].0 == TOKEN_BIT_MOVE_LEFT.0 {
            let a = get_element_from_stack(&mut stack, "bit move left").1;
            let b = get_element_from_stack(&mut stack, "bit move left").1;
            stack.push((1, b << a, "".to_string()));
        } else if tokens[i].0 == TOKEN_BIT_MOVE_RIGTH.0 {
            let a = get_element_from_stack(&mut stack, "bit move rigth").1;
            let b = get_element_from_stack(&mut stack, "bit move rigth").1;
            stack.push((1, b >> a, "".to_string()));
        } else if tokens[i].0 == TOKEN_POWER.0 {
            let a = get_element_from_stack(&mut stack, "power").1;
            let b = get_element_from_stack(&mut stack, "power").1;
            stack.push((1, i32::pow(b, a as u32), "".to_string()));
        } else if tokens[i].0 == TOKEN_ROOT.0 {
            let a = get_element_from_stack(&mut stack, "plus").1;
            let b = get_element_from_stack(&mut stack, "plus").1;
            stack.push((1, i32::pow(b, 1 / a as u32), "".to_string()));
        } else {
            error("Interpretation error", format!("Unexpected syntax, has been parsed, but can't be interpreted: {}", tokens[i].0).as_str());
        }
    }
}

fn error(error: &str, text: &str) {
    println!("\x1b[31m\x1b[1m{}: \x1b[0m\x1b[31m{}\x1b[0m", error, text);
    std::process::exit(1);
}

fn check_stack_len(stack: &Vec<(i32, i32, String)>, len: usize, operation: &str) {
    if stack.len() < len {
        error("Stack error", format!("Legth of the stack is smaller than {}, what is needed for {} operation.", len, operation).as_str());
    } else {
        return;
    }
}

fn get_element_from_stack(stack: &mut Vec<(i32, i32, String)>, operation: &str) -> (i32, i32, String) {
    check_stack_len(&stack, 1, operation);
    if let Some((a, b, c)) = stack.pop() {
        return (a, b, c);
    } else {
        error("Stack error", format!("Unable to pop element from stack to do {}.", operation).as_str());
        return (0, 0, "".to_string());
    }
}
