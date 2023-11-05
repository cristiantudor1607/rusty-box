use std::fs::{remove_dir, remove_dir_all, remove_file};
use std::io::{Error, ErrorKind};

use crate::utils::get_params as get_names;
use crate::utils::set_path_status as set_entry_type;
use crate::utils::PathStatus as EntryType;

#[derive(Debug, PartialEq, Eq)]
pub enum RmOption {
    Recursive,
    EmptyDirs,
    OnlyFiles,
    All,
}

fn rmset_options(args: &Vec<String>) -> RmOption {
    /* Set the return value to implicit option */
    let mut ret = RmOption::OnlyFiles;

    for arg in args {
        let item = arg.as_str();
        match item {
            "-d" | "--dir" => {
                /* If the option was previously set to Recursive, set to All */
                if ret == RmOption::Recursive {
                    ret = RmOption::All;
                } else {
                    ret = RmOption::EmptyDirs;
                };
            }
            "-r" | "-R" | "--recursive" => {
                /* If the option was previously set to EmptyDirs, set to All */
                if ret == RmOption::EmptyDirs {
                    ret = RmOption::All;
                } else {
                    ret = RmOption::Recursive;
                };
            }
            _ => (),
        };
    }

    ret
}

fn check_rmargs(args: &Vec<String>) -> bool {
    let n = args.len();

    /* The "worst case" is when there are both -r and -d flags
    (rustybox rm  -r -d name) -> 5 arguments */
    if n >= 5 {
        return true;
    }

    /* rm requires at least 1 arg */
    if n == 2 {
        return false;
    }

    /* Count the number of options */
    let mut opt_counter = 0;

    for arg in args {
        match arg.as_str() {
            "-d" | "--dir" | "-r" | "-R" | "--recursive" => opt_counter += 1,
            _ => (),
        };
    }

    if n == 3 && opt_counter == 1 {
        return false;
    }

    if n == 4 && opt_counter == 2 {
        return false;
    }

    return true;
}

fn delete_dir(name: &String, opt: &RmOption) -> Result<(), std::io::Error> {
    /* The deletion is made depending on RmOption */
    match opt {
        /* If the option is OnlyFiles, do nothing and return an error */
        RmOption::OnlyFiles => {
            eprintln!("rm: cannot remove '{}': Is a directory", name);
            return Err(Error::from(ErrorKind::Other));
        }
        /* If the option is EmptyDirs, it should call the remove_dir function,
        to delete just the empty directors */
        RmOption::EmptyDirs => {
            match remove_dir(name) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("rm: cannot remove '{}': Directory not empty", name);
                    return Err(e);
                }
            };
        }
        /* If the option is Recursive or All, it should call the remove_dir_all
        function, to delete everything */
        RmOption::Recursive | RmOption::All => {
            match remove_dir_all(name) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("rm: unexpected error: {}", e);
                    return Err(e);
                }
            };
        }
    };

    Ok(())
}

pub fn rm(args: &Vec<String>) -> Result<i32, ()> {
    /* Check the user input */
    if !check_rmargs(args) {
        /* Return an invalid command */
        return Ok(-1);
    }

    /* Set the options */
    let opt = rmset_options(args);
    let names: Vec<String>;

    match opt {
        /* If there are no options, the names starts with the 3rd string in
        list (at index 2) */
        RmOption::OnlyFiles => names = get_names(args, (2, usize::MAX)),
        /* If there is an option, the names starts with the 4th string in
        list (at index 3) */
        RmOption::Recursive | RmOption::EmptyDirs => names = get_names(args, (3, usize::MAX)),
        /* If there are 2 options, the names starts with the 5th string in
        list (at index 4) */
        RmOption::All => names = get_names(args, (4, usize::MAX)),
    };

    /* Variable to remember if there was an error at some point, while
    deleting the entries */
    let mut error = false;

    for name in names {
        match set_entry_type(&name) {
            /* If the entry doesn't exist, set the error variable */
            Ok(EntryType::IsNot) => {
                eprintln!("rm: cannot remove '{}': No such file or directory", name);
                error = true;
            }
            /* If the entry is a file, we can delete it, no matter the options */
            Ok(EntryType::IsFile) => {
                match remove_file(&name) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("rm: unexpected error: {}", e);
                        error = true;
                    }
                };
            }
            /* If the entry is a directory, the deletion depends on the options
            provided */
            Ok(EntryType::IsDir) => {
                match delete_dir(&name, &opt) {
                    Ok(_) => (),
                    /* The error was already printed in the delete_dir
                    function */
                    Err(_) => error = true,
                }
            }

            /* Other errors */
            Err(e) => {
                eprintln!("rm: unexpected error: {}", e);
                error = true;
            }
        };
    }

    if error {
        return Err(());
    }

    return Ok(0);
}
