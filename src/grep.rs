use regex::Error;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;

use crate::utils::open_file;
use crate::utils::read_file;

fn get_option(args: &Vec<String>) -> Result<bool, ()> {
    if args.len() == 5 {
        if args[2] == "-i" {
            return Ok(true);
        } else {
            return Err(());
        };
    };

    if args.len() == 4 {
        return Ok(false);
    };

    return Err(());
}

fn print_matching_lines(
    buffer: Lines<BufReader<File>>,
    patttern: &String,
    option: bool,
) -> Result<(), regex::Error> {
    /* Inspired by: https://docs.rs/regex/latest/regex/struct.Regex.html#method.is_match */
    let regex_pat: Regex;
    match Regex::new(patttern) {
        Ok(ret) => regex_pat = ret,
        Err(e) => return Err(e),
    };

    for line in buffer {
        match line {
            Ok(text) => {
                if (regex_pat.is_match(&text) && !option) || (!regex_pat.is_match(&text) && option)
                {
                    println!("{}", text);
                }
            },
            Err(_) => return Err(Error::Syntax(String::from("Unknown"))),
        };
    };

    Ok(())
}

pub fn grep(args: &Vec<String>) -> Result<i32, ()> {
    let option: bool;
    match get_option(args) {
        Ok(ret) => option = ret,
        Err(_) => return Ok(-1),
    };

    let pattern: &String;
    let filename: &String;
    match option {
        true => {
            pattern = &args[3];
            filename = &args[4];
        }
        false => {
            pattern = &args[2];
            filename = &args[3];
        }
    };

    let content: Lines<BufReader<File>>;
    match open_file(filename) {
        Ok(file) => {
            content = read_file(file);
        }
        Err(e) => {
            eprintln!("grep: unexpected error: {}", e);
            return Err(());
        }
    };

    match print_matching_lines(content, pattern, option) {
        Ok(_) => return Ok(0),
        Err(e) => {
            eprintln!("grep: unexpected error: {}", e);
            return Err(());
        }
    }
}
