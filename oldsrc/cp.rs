use std::io::{Error, ErrorKind};
use std::fs::{self, copy};

use crate::utils::check_dir as set_entry_type;
use crate::utils::PathStatus as PathStatus;

#[derive(Debug, PartialEq, Eq)]
pub enum CpType {
    Recursive,
    NonRecursive,
}

/* get_cp_option function will be called after we make sure that there are 
at least 4 strings in the Vector (rustybox, cp, source, dest)*/
pub fn cpget_option(args: &Vec<String>) -> CpType {
    /* -r should be on the 2nd positions */
    let  opt = &args[2];
    if opt == "-r" || opt == "-R" || opt  == "--recursive" {
        return CpType::Recursive;
    };

    return CpType::NonRecursive;
}

pub fn cpget_src_and_dest(args: &Vec<String>, opt: &CpType) -> (String, String) {
    let src: &String;
    let dst: &String;

    match opt {
        /* If there isn't an option specified, the src is at 2nd index, and dst
        at 3rd index */
        CpType::NonRecursive => {
            src = &args[2];
            dst = &args[3];     
        },
        /* If the recursive option is specified, then src should be at 3rd
        element, and dst at 4th element */
        CpType::Recursive => {
            src = &args[3];
            dst = &args[4];
        },
    };

    return(src.to_string(), dst.to_string());
}

pub fn parse_destination(dest: &String) -> Result<Option<String>, std::io::Error> {
    match set_entry_type(dest) {
        Err(e) => return Err(e),
        Ok(ret) => {
            /* comm */
            match ret {
                PathStatus::IsNot => return Ok(Some(dest.to_string())),
                PathStatus::IsFile => return Ok(None),
                PathStatus::IsDir => return Ok()
            }
        }
    }
}

pub fn cp(args: &Vec<String>) -> Result<(), std::io::Error> {
    /* Check the number of given parameters */
    let n = args.len();
    if n == 1 || n == 2 || n == 3 {
        eprintln!("Invalid command");
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    /* Variable to store if the recursive option is active */
    let opt = cpget_option(args);
    if n == 4 && opt == CpType::Recursive {
        eprintln!("Invalid command");
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    /* Get the relevant strings from the list */
    let (src, dst) = cpget_src_and_dest(args, &opt);

    /* Variable to store if the path doesn't exist, is a file or is a
    directory */
    let entry_type: PathStatus;
    match set_entry_type(&src) {
        Ok(stat) => entry_type = stat,
        Err(e) => return Err(e),
    };

    match entry_type {
        /* If the specified path doesn't exist in the filesystem, return an error */
        PathStatus::IsNot => return Err(Error::from(ErrorKind::NotFound)),

        /* If the specified path is a file, copy the file using copy method */
        PathStatus::IsFile => {
            match copy(src, dst) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        },

        PathStatus::IsDir => (),
    };

    Ok(())
}