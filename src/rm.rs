use std::io::{Error, ErrorKind};
use std::fs::{remove_file, remove_dir, remove_dir_all};
use crate::utils::PathStatus;
use crate::utils::RmOption as RmOption;
use crate::utils::extract_params_inrange as get_names;
use crate::utils::check_dir as check_dir;

pub fn set_options(args: &Vec<String>) -> Option<RmOption> {
    let opt = args[2].as_str();

    if opt == "-r" || opt == "-R" || opt == "--recursive" {
        return Some(RmOption::Recursive);
    };

    if opt == "-d" || opt == "--dir" {
        return Some(RmOption::Dir);
    };

    return None;
}

/* del_dir function analyzes the option provided and delete the directory,
if possible */
pub fn del_dir(name: &String, opt: &Option<RmOption>) -> Result<(), std::io::Error> {

    match opt {
        /* If there isn't an option, the directory can't be deleted */
        None => {
            eprintln!("rm: cannot remove '{}': Is a directory", name);
            return  Err(Error::from(ErrorKind::Other));
        },

        /* Choose what type of deletion should perform */
        Some(opt_type) => {
            match opt_type {
                /* If -d or --dir option is active, it should call the
                remove_dir method */
                RmOption::Dir => {
                    match remove_dir(name) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                },
                /* If -r, -R or --recursive option is active, it should
                call the remove_dir_all method */
                RmOption::Recursive => {
                    match remove_dir_all(name) {
                        Ok(_) => (),
                        Err(e) => return  Err(e),
                    }
                },
            };
        },
    };

    Ok(())
}

pub fn remove_entry(name: &String, opt: &Option<RmOption>) -> Result<(), std::io::Error> {

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
            match del_dir(name, &opt) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        },
        /* It will never reach this point */
        _ => (),
    };


    Ok(())
}

pub fn rm(args: &Vec<String>) -> Result<(), std::io::Error> {
    /* Check the user input */
    if args.len() == 2 {
        eprintln!("rm: missing operand");
        return Err(Error::from(ErrorKind::InvalidInput));
    };

    /* Set the options */
    let opt = set_options(args);
    let names: Vec<String>;


    match opt {
        /* If there is an option, the names starts with the 4th string in
        list */
        Some(_) => {
            names = get_names(args, 3, usize::MAX);
        },
        /* If there are no options, the names starts with the 3rd string in
        list */
        None => {
            names = get_names(args, 2, usize::MAX);
        },
    };

    for name in names {
        match remove_entry(&name, &opt) {
            Ok(_) => (),
            // TODO: save the errors and print them -> DO SOMETHING!!
            // i have to
            Err(e) => return Err(e),
        }
    }

    Ok(())
}