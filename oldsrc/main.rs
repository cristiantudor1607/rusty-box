use std::env;
use std::process::exit;
use std::usize;

mod utils;
mod pwd;
mod echo;
mod cat;
mod mkdir;
mod rmdir;
mod mv;
mod rm;
mod ln;
mod cp;


/* All the methods I use are taken from doc.rust-lang.org */

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "pwd" => {
            pwd::pwd();
            exit(0);
        },
        "cat" => {
            let params = utils::extract_params_inrange(&args, 2, usize::MAX);
            
            /* I used a NOT because I want the error variable to be true when
            there is an error  */
            let error = !cat::cat(params);
            if error == true {
                exit(-20);
            } else {
                exit(0);
            };
        },
        "mkdir" => {
            let dirs = utils::extract_params_inrange(&args, 2, usize::MAX);
            match mkdir::mkdir(dirs) {
                Ok(()) => exit(0),
                Err(_) => exit(-30),
            };
        },
        "rmdir" => {
            let dirs = utils::extract_params_inrange(&args, 2, usize::MAX);
            match rmdir::rmdir(dirs) {
                Ok(_) => exit(0),
                Err(_) => exit(-60),
            };
        },
        "echo" => {
            match echo::echo(&args) {
                Ok(_) => exit(0),
                Err(e) => {
                    println!("echo: unexpected error: {}", e);
                    exit(-10);
                },
            };
        },
        "mv" => {
            match mv::mv(&args) {
                Ok(_) => exit(0),
                Err(_) => exit(-40),
            };
        },
        "rm" => {
            match rm::rm(&args) {
                Ok(_) => exit(0),
                Err(my_code) => exit(my_code),
            };
        },
        "ln" => {
            match ln::ln(&args) {
                Ok(_) => exit(0),
                Err(_) => exit(-50), 
            };
        },
        "cp" => {
            match cp::cp(&args) {
                Ok(_) => exit(0),
                Err(e) => {
                    eprintln!("unexpected error: {}", e);
                    exit(-90)
                },
            };
        },
        _ => {
            println!("Invalid command");
            exit(-1);
        },
    };

}