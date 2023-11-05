use std::fs;
use std::fs::{read_dir, ReadDir};
use std::io::{self, Error, ErrorKind};

use crate::mkdir::create_newdir;
use crate::utils::get_string;
use crate::utils::set_path_status as set_entry_type;
use crate::utils::PathStatus as EntryType;

#[derive(Debug, PartialEq, Eq)]
pub enum CpOption {
    Recursive,
    NonRecursive,
}

fn cpget_option(args: &Vec<String>) -> CpOption {
    /* The option should be at index 2 */
    let opt = &args[2];
    if opt == "-r" || opt == "-R" || opt == "--recursive" {
        return CpOption::Recursive;
    };

    return CpOption::NonRecursive;
}

fn cpget_newname(from: &String, to: &String) -> String {
    /* Split the path */
    let splitted: Vec<&str> = from.split('/').collect();
    let n = splitted.len();

    /* Get the last name from the path */
    let s = match splitted[n - 1] {
        "" => splitted[n - 2],
        _ => splitted[n - 1],
    };

    let mut newname = to.clone();
    /* Put '/' at the end if it isn't there */
    match newname.chars().last().unwrap() {
        '/' => (),
        _ => newname.push('/'),
    };

    /* Add rest od the name */
    newname.push_str(s);

    return newname;
}

fn cpparse_dest(src: &String, dest: &String) -> Option<String> {
    let dest_type: EntryType;

    match set_entry_type(dest) {
        Ok(ret) => dest_type = ret,
        Err(e) => {
            eprintln!("cp: unexpected error: {}", e);
            return None;
        }
    };

    if dest_type == EntryType::IsNot {
        return Some(dest.clone());
    };

    if dest_type == EntryType::IsFile {
        return Some(dest.clone());
    };

    /* If dest is a directory, then it should be renamed directory/src, but
    not all of src, just the last name after a '/' */
    let words: Vec<&str> = src.split("/").collect();

    /* If the dest has a "/" at the end, it should use the last but one */
    let s = match words[words.len() - 1] {
        "" => words[words.len() - 2],
        _ => words[words.len() - 1],
    };

    let mut new_name = dest.clone();

    /* Before adding s to the end of dest, we should check is dest has '/' at
    the end */
    match dest.chars().last().unwrap() {
        '/' => (),
        _ => new_name.push('/'),
    };

    /* Add the extra name */
    new_name.push_str(s);

    return Some(new_name);
}

fn copy_dir(from: &String, to: &String) -> io::Result<()> {
    /* Base case: When it reaches a file */
    println!("{}", from);
    let from_type = set_entry_type(from);
    match from_type {
        /* Return the error */
        Err(e) => return Err(e),

        /* Check if the "from" entry is a file */
        Ok(ret) => {
            if ret == EntryType::IsFile {
                /* Copy the contents of the file */
                match fs::copy(from, to) {
                    /* If the operation was successfully done, return from
                    recursion */
                    Ok(_) => return Ok(()),
                    /* Otherwise, return from the recursion with an error */
                    Err(e) => return Err(e),
                };
            };
        }
    };

    /* Load the directory */
    let dir: ReadDir;
    match read_dir(from) {
        Ok(struc) => dir = struc,
        Err(e) => return Err(e),
    };

    // TODO: Get rid of panics
    /* Go through all of it's contents */
    for entry in dir {
        let item = entry?;

        /* Create the new name for the file */
        let filename: String;

        match item.path().to_str() {
            Some(name) => filename = name.to_string(),
            None => return Err(Error::from(ErrorKind::Other)),
        };

        let new_filename = cpget_newname(&filename, to);
        println!("{} -> {}", filename, new_filename);
        /* If the item is a directory, create it before recursion */
        if item.file_type()?.is_dir() {
            match create_newdir(&new_filename) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        };

        match copy_dir(&filename, &new_filename) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }

    Ok(())
}

pub fn cp(args: &Vec<String>) -> Result<i32, ()> {
    let n = args.len();

    if n < 4 {
        return Ok(-1);
    };

    /* Set the options */
    let opt = cpget_option(args);

    /* If recursive is specified, but no destination, then it is an invalid
    command */
    if opt == CpOption::Recursive && n == 4 {
        return Ok(-1);
    };

    let src: String;
    let dest: String;

    // TODO: consider refactor this
    match opt {
        CpOption::NonRecursive => {
            match get_string(args, 2) {
                Some(s) => src = s,
                /* It will never be None, but the compiler will think I use
                src and name uninitialised, so return something irrelevant */
                None => return Err(()),
            };

            match get_string(args, 3) {
                Some(s) => dest = s,
                /* Same as above */
                None => return Err(()),
            };
        }

        CpOption::Recursive => {
            match get_string(args, 3) {
                Some(s) => src = s,
                /* Return something irrelevant */
                None => return Err(()),
            };

            match get_string(args, 4) {
                Some(s) => dest = s,
                /* Return something irrelevant */
                None => return Err(()),
            };
        }
    };

    let src_type: EntryType;
    match set_entry_type(&src) {
        Ok(t) => src_type = t,
        Err(e) => {
            eprintln!("cp: unexpected error: {}", e);
            return Err(());
        }
    };

    /* If the src isn't in the filesystem, cp stops here */
    if src_type == EntryType::IsNot {
        eprintln!("cp: cannot stat '{}': No such file or directory", src);
        return Err(());
    };

    let name: String;
    match cpparse_dest(&src, &dest) {
        Some(ret) => name = ret,
        /* None is returned only if en error was encountered */
        None => return Err(()),
    };

    match src_type {
        /* If the source is a file we can copy it */
        EntryType::IsFile => {
            match fs::copy(src, name) {
                Ok(_) => return Ok(0),
                Err(_) => return Err(()),
            };
        }
        EntryType::IsDir => {
            if opt == CpOption::NonRecursive {
                eprintln!("cp: -r not specified; omitting directory '{}'", src);
                return Err(());
            };

            /* Create the copy of the root source directory in the destination
            directory */
            match create_newdir(&name) {
                Ok(_) => (),
                Err(e) => {
                    println!("cp: unexpected error: {}", e);
                    return Err(());
                }
            };

            match copy_dir(&src, &name) {
                Ok(_) => return Ok(0),
                Err(e) => {
                    println!("cp: unexpected error: {}", e);
                    return Err(());
                }
            }
        }
        _ => (),
    };

    Ok(0)
}
