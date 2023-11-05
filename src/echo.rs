use crate::utils::get_params;
use std::io::{stdout, Write};

fn add_newline(args: &Vec<String>) -> bool {
    /* If the user types echo, and the press enter, there will be printed just
    a newline */
    if args.len() == 2 {
        return true;
    };

    if args[2].eq(&String::from("-n")) {
        return false;
    };

    return true;
}

pub fn echo(args: &Vec<String>) -> Result<i32, ()> {
    let newline = add_newline(args);
    let n = args.len();

    if n == 2 {
        println!("");
        return Ok(0);
    }

    /* If the user types echo -n and then press enter */
    if n == 3 && newline == false {
        return Ok(0);
    }

    let items: Vec<String>;

    if newline == true {
        items = get_params(&args, (2, usize::MAX));
    } else {
        items = get_params(&args, (3, usize::MAX));
    };

    /* Method to print inspired by https://doc.rust-lang.org/std/macro.print.html */
    let mut lock = stdout().lock();

    /* I want to use write! macro, and I need to add an extra space before each
    the string, to have spaces between them, but the first one */
    let first = &items[0];
    match write!(lock, "{}", first) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("echo: unexpected error: {}", e);
            return Err(());
        }
    };

    for i in 1..items.len() {
        match write!(lock, " {}", items[i]) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("echo: unexpected error: {}", e);
                return Err(());
            }
        };
    }

    if newline {
        match write!(lock, "\n") {
            Ok(_) => (),
            Err(e) => {
                eprintln!("echo: unexpected error: {}", e);
                return Err(());
            }
        };
    };

    Ok(0)
}
