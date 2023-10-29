use std::io::Write;
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

    let mut w = std::io::stdout().lock();
    for item in items {
        match write!(&mut w, "{} ", item) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    if newln == true {
        match write!(&mut w, "\n") {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    };

    Ok(())
}

