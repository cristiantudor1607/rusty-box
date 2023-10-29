use std::io::{stdout, Write};
use crate::utils;

pub fn add_newline(args: &Vec<String>) -> bool {
    /* args[0] should be the name of the executable 
       args[1] should be the "echo" command name
       args[2] can be "-n" option */

    if args[2].eq(&String::from("-n")) {
        return false;
    }

    return true;
}

pub fn echo(args: &Vec<String>) -> Result<(), std::io::Error> {
    let items: Vec<String>;
    let newln = add_newline(args);

    if newln == true {
        items = utils::extract_params_inrange(&args, 2, usize::MAX);
    } else {
        items = utils::extract_params_inrange(&args, 3, usize::MAX);
    };

    /* Method to print inspired by https://doc.rust-lang.org/std/macro.print.html */
    let n = items.len() - 1;
    let mut i = 0;
    let mut lock = stdout().lock();

    for item in items {
        /* If the item isn't the last element, it should print a space */
        if i != n {
            match write!(lock, "{} ", item) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        /* If the item is the last element it shouldn't print a space after */
        } else {
            match write!(lock, "{}", item) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }

        i = i + 1;
    }

    if newln {
        print!("\n");
    }

    Ok(())
}

