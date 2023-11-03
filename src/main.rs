use std::env;
use std::process::exit;

mod utils;
mod pwd;
use pwd::pwd as pwd;
mod cat;
use cat::cat as cat;
mod mkdir;
use mkdir::mkdir as mkdir;
mod rmdir;
use rmdir::rmdir as rmdir;
mod echo;
use echo::echo as echo;
mod mv;
use mv::mv as mv;
mod rm;
use rm::rm as rm;
mod ln;
use ln::ln as ln;
mod cp;
use cp::cp as cp;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args[1].as_str() {
        "pwd" => {
            pwd();
            exit(0);
        },
        "cat" => {
            match cat(&args) {
                Ok(_) => exit(0),
                Err(_) => exit(-20),
            };
        },
        "mkdir" => {
            match mkdir(&args) {
                Ok(code) => {
                    match code {
                        -1 => { println!("Invalid command"); exit(-1); },
                        0 => exit(0),
                        _ => (),
                    };
                },
                Err(_) => exit(-30),
            };
        },
        "rmdir" => {
            match rmdir(&args) {
                Ok(code) => {
                    match code {
                        -1 => { println!("Invalid command"); exit(-1); },
                        0 => exit(0),
                        _ => (),
                    };
                },
                Err(_) => exit(-60),
            }   
        },
        "echo" => {
            match echo(&args) {
                /* In this case the code can only be 0 */
                Ok(code) => exit(code),
                Err(_) => exit(-10),
            };
        },
        "mv" => {
            match mv(&args) {
                Ok(code) => {
                    match code {
                        -1 => { println!("Invalid command"); exit(-1); },
                        0 => exit(0),
                        /* It will never go to this arm */
                        _ => (),
                    };
                },
                Err(_) => exit(-40),
            };
        },
        "rm" => {
            match rm(&args) {
                Ok(code) => {
                    match code {
                        -1 => { println!("Invalid command"); exit(-1); },
                        0 => exit(0),
                        _ => (),
                    };
                },

                Err(_) => exit(-70),
            };
        },
        "ln" => {
            match ln(&args) {
                Ok(code) => {
                    match code {
                        -1 => { println!("Invalid command"); exit(-1); },
                        0 => exit(0),
                        _ => (),
                    };
                },
                Err(_) => exit(-50),
            };
        },
        "cp" => {
            match cp(&args) {
                Ok(code) => {
                    match code {
                        -1 => { println!("Invalid command"); exit(-1); },
                        0 => exit(0),
                        _ => (),
                    };
                },
                Err(_) => exit(-90),
            };
        },
        _=> {
            println!("Invalid command");
            exit(-1);
        }
    }
}