use std::io::{Error, ErrorKind};
use std::fs::{remove_file, remove_dir, remove_dir_all};
use crate::utils::PathStatus;
use crate::utils::RmOption as RmOption;
use crate::utils::extract_params_inrange as get_names;
use crate::utils::check_dir as check_dir;

// TODO: Add messages when errors are encountered
pub fn check_rmargs(args: &Vec<String>) -> Result<(), std::io::Error> {
    /* If it has more than 4 strings in list, then the command should be
    valid */
    let n = args.len();
    if n > 4 {
        return Ok(());
    };

    if n == 2 {
        println!("Invalid command");
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    /* Count the number of options */
    let mut opt_counter = 0;
    
    for arg in args {
        match arg.as_str() {
            "-d" | "--dir" | "-r" | "-R" | "--recursive" =>
                opt_counter += 1,
            _ => (),
        };
    };

    if n == 3 && opt_counter == 1 {
        println!("Invalid command");
        return Err(Error::from(ErrorKind::InvalidInput));
    };

    if n == 4 && opt_counter == 2 {
        println!("Invalid command");
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    Ok(())
}

pub fn set_options(args: &Vec<String>) -> RmOption {
    
    /* Set the return variable to the implicit option */
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
            },
            "-r" | "-R" | "--recursive" => {
                /* If the option was previously set to EmptyDirs, set to All */
                if ret == RmOption::EmptyDirs {
                    ret = RmOption::All;
                } else {
                    ret = RmOption::Recursive;
                };
            },
            _ => (),
        };
    };

    return ret;
}

/* delete_dir function analyzes the option provided and delete the directory,
if possible */
pub fn delete_dir(name: &String, opt: &RmOption) -> Result<(), std::io::Error> {
    match opt {
        /* If the option is OnlyFiles, do nothing and return an error*/
        // TODO: Add message
        RmOption::OnlyFiles =>
            return Err(Error::from(ErrorKind::Other)),
        
        /* If the option is EmptyDirs, it should call the remove_dir function,
        to delete just the empty directors */
        RmOption::EmptyDirs => {
            match remove_dir(name) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        },

        /* If the option is Recursive or All, it should call the remove_dir_all
        function, to delete everything */
        RmOption::Recursive | RmOption::All => {
            match remove_dir_all(name) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        },
    }

    Ok(())
}

pub fn rmentry(name: &String, opt: &RmOption) -> Result<(), std::io::Error> {
    /* Determine if the name describes a file or a directory in the system */
    let entry_type: PathStatus;
    match check_dir(name) {
        /* Abort in case of error */
        Err(e) => return Err(e),
        
        Ok(ret) => {
            match ret {
                /* If the path doesn't exist, stop here and return an error */
                PathStatus::IsNot => {
                    eprintln!("rm: cannot remove '{}': No such file or directory", name);
                    return Err(Error::from(ErrorKind::NotFound));
                },
                /* Set the entry_type variable */
                PathStatus::IsFile => entry_type = PathStatus::IsFile,
                PathStatus::IsDir => entry_type = PathStatus::IsDir,
            };
        },
    };

    /* Delete the entries if possible */
    match entry_type {
        /* If the entry is a file, it should be deleted, regardless of the
        options */
        PathStatus::IsFile => {
            match remove_file(name) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        },
        /* If the entry is a directory, we should check the options */
        PathStatus::IsDir => {
            match delete_dir(name, opt) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        },
        /* It will never reach this point */
        _ => (),
    };
    
    Ok(())
}

pub fn rm(args: &Vec<String>) -> Result<(), i32> {
    /* Check the user input */
    match check_rmargs(&args) {
        Ok(_) => (),
        Err(_) => return Err(-1),
    };

    /* Set the options */
    let opt = set_options(args);
    let names: Vec<String>;

    match opt {
        /* If there are no options, the names starts with the 3rd string in
        list (at index 2) */
        RmOption::OnlyFiles =>
            names = get_names(args, 2, usize::MAX),
        /* If there is an option, the names starts with the 4th string in
        list (at index 3) */
        RmOption::Recursive | RmOption::EmptyDirs =>
            names = get_names(args, 3, usize::MAX),
        /* If there are 2 options, the names starts with the 5th string in
        list (at index 4) */
        RmOption::All => 
            names = get_names(args, 4, usize::MAX),
    };

    /* Variable to remember if there was an error at some point, while
    deleting the entries */
    let mut error = false;
    
    for name in names {
        match rmentry(&name, &opt) {
            Ok(_) => (),
            Err(_) => error = true,
        };
    }

    if error {
        return Err(-70);
    };

    Ok(())
}