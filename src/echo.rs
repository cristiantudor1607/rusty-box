use std::io::{stdout, Write};
use crate::utils;

pub fn add_newline(args: &Vec<String>) -> bool {
    /* args[0] should be the name of the executable 
       args[1] should be the "echo" command name
       args[2] can be "-n" option */

    /* If the user type just "echo" */
    if args.len() == 2 {
        return true;
    };

    if args[2].eq(&String::from("-n")) {
        return false;
    };

    return true;
}

pub fn echo(args: &Vec<String>) -> Result<(), std::io::Error> {
    let newln = add_newline(args);
    let n = args.len();

    /* If the user type just "echo" */
    if n == 2 {
        println!("");
        return Ok(());
    };

    /* If the user type just "echo -n" */
    if n == 3 && !newln {
        return Ok(());
    };
    
    let items: Vec<String>;
   

    if newln == true {
        items = utils::extract_params_inrange(&args, 2, usize::MAX);
    } else {
        items = utils::extract_params_inrange(&args, 3, usize::MAX);
    };

    /* Method to print inspired by https://doc.rust-lang.org/std/macro.print.html */
    let mut lock = stdout().lock();

    /* I want to use write! macro, and I need to add an extra space before
    the string, but the first one */
    let first = &items[0];
    match write!(lock, "{}", first) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    /* I need a classic iterator to skip the first element in the for loop */
    let mut i = 0;

    for item in items {
        if i == 0 {
            i += 1;
            continue;
        };

        match write!(lock, " {}", item) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }


    if newln {
        print!("\n");
    }

    Ok(())
}

