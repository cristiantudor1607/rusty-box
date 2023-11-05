use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Lines;
use std::usize;

use crate::utils;
use crate::utils::open_file;
use crate::utils::read_file;

fn typing_loop() {
    loop {
        /* Check the link from README */
        let mut fake_input = String::new();
        match io::stdin().read_line(&mut fake_input) {
            Ok(_) => print!("{}", fake_input),

            Err(e) => {
                println!("cat: unexpected error: {}", e);
                break;
            }
        }
    }
}

fn print_contents(buffer: Lines<BufReader<File>>) -> Result<(), std::io::Error> {
    /* For each line in the buffer print it's characters if you can*/
    for line in buffer {
        match line {
            Ok(text) => println!("{}", text),
            Err(e) => {
                eprintln!("cat: unexpected error: {}", e);
                return Err(e);
            }
        };
    }

    Ok(())
}

pub fn cat(args: &Vec<String>) -> Result<i32, ()> {
    /* If the user types "cat" in terminal it enters in an infinte loop, where
    you can type strings, and after \n is pressed, the strings are printed on
    the next line */
    if args.len() == 2 {
        typing_loop();
    };

    /* Get the filenames */
    let files = utils::get_params(args, (2, usize::MAX));

    /* If one of the files doesn't exist, the code will be set to false, and
    the function will drop an error at the end, after printing the contents of
    the existing files */
    let mut code: bool = true;

    for file in files {
        /* Open each file and read it's contents, if possible */
        match open_file(&file) {
            Ok(fileobj) => {
                let content = read_file(fileobj);
                match print_contents(content) {
                    Ok(_) => (),
                    Err(_) => code = false,
                };
            }
            Err(e) => {
                eprintln!("cat: unexpected error: {}", e);
                code = false;
            }
        }
    }

    if !code {
        return Err(());
    };

    Ok(0)
}
