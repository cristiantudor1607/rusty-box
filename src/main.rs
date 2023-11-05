use std::env;
use std::process::exit;

mod pwd;
mod utils;
use pwd::pwd;
mod cat;
use cat::cat;
mod mkdir;
use mkdir::mkdir;
mod rmdir;
use rmdir::rmdir;
mod echo;
use echo::echo;
mod mv;
use mv::mv;
mod rm;
use rm::rm;
mod ln;
use ln::ln;
mod cp;
use cp::cp;
mod touch;
use touch::touch;
mod chmod;
use chmod::chmod;
mod ls;
use ls::ls;
mod grep;
use grep::grep;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "pwd" => {
            pwd();
            exit(0);
        }
        "cat" => {
            match cat(&args) {
                Ok(_) => exit(0),
                Err(_) => exit(-20),
            };
        }
        "mkdir" => {
            match mkdir(&args) {
                Ok(code) => {
                    match code {
                        -1 => {
                            println!("Invalid command");
                            exit(-1);
                        }
                        0 => exit(0),
                        _ => (),
                    };
                }
                Err(_) => exit(-30),
            };
        }
        "rmdir" => match rmdir(&args) {
            Ok(code) => {
                match code {
                    -1 => {
                        println!("Invalid command");
                        exit(-1);
                    }
                    0 => exit(0),
                    _ => (),
                };
            }
            Err(_) => exit(-60),
        },
        "echo" => {
            match echo(&args) {
                /* In this case the code can only be 0 */
                Ok(code) => exit(code),
                Err(_) => exit(-10),
            };
        }
        "mv" => {
            match mv(&args) {
                Ok(code) => {
                    match code {
                        -1 => {
                            println!("Invalid command");
                            exit(-1);
                        }
                        0 => exit(0),
                        /* It will never go to this arm */
                        _ => (),
                    };
                }
                Err(_) => exit(-40),
            };
        }
        "rm" => {
            match rm(&args) {
                Ok(code) => {
                    match code {
                        -1 => {
                            println!("Invalid command");
                            exit(-1);
                        }
                        0 => exit(0),
                        _ => (),
                    };
                }

                Err(_) => exit(-70),
            };
        }
        "ln" => {
            match ln(&args) {
                Ok(code) => {
                    match code {
                        -1 => {
                            println!("Invalid command");
                            exit(-1);
                        }
                        0 => exit(0),
                        _ => (),
                    };
                }
                Err(_) => exit(-50),
            };
        }
        "cp" => {
            match cp(&args) {
                Ok(code) => {
                    match code {
                        -1 => {
                            println!("Invalid command");
                            exit(-1);
                        }
                        0 => exit(0),
                        _ => (),
                    };
                }
                Err(_) => exit(-90),
            };
        }
        "touch" => match touch(&args) {
            Ok(code) => {
                match code {
                    -1 => {
                        println!("Invalid command");
                        exit(-1);
                    }
                    0 => exit(0),
                    _ => (),
                };
            }

            Err(_) => exit(-100),
        },
        "chmod" => match chmod(&args) {
            Ok(-1) => {
                println!("Invalid command");
                exit(-1);
            }
            Ok(0) => exit(0),
            _ => exit(-25),
        },
        "ls" => match ls(&args) {
            Ok(0) => exit(0),
            _ => exit(-80),
        },
        "grep" => match grep(&args) {
            Ok(-1) => {
                println!("Invalid command");
                exit(-1);
            }
            Ok(0) => exit(0),
            _ => exit(2),
        },
        _ => {
            println!("Invalid command");
            exit(-1);
        }
    }
}
