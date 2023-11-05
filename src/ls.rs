use std::fs::DirEntry;
use std::fs::{read_dir, ReadDir};
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ListingType {
    Default,
    Recursive,
    All,
    RecursiveAll,
}

fn get_listing_type(args: &Vec<String>) -> ListingType {
    if args.len() == 2 {
        return ListingType::Default;
    };

    let mut lstype: ListingType = ListingType::Default;
    for arg in args {
        match arg.as_str() {
            "-a" | "-all" => {
                if lstype == ListingType::Recursive {
                    lstype = ListingType::RecursiveAll;
                } else {
                    lstype = ListingType::All;
                };
            }
            "-R" | "-recursive" => {
                if lstype == ListingType::All {
                    lstype = ListingType::RecursiveAll;
                } else {
                    lstype = ListingType::Recursive;
                };
            }
            _ => (),
        };
    }

    return lstype;
}

fn get_pathname(args: &Vec<String>, lstype: ListingType) -> Result<String, std::io::Error> {
    let n = args.len();

    /* If the command has at least one option and a file/directoy
    specified or it has no options and a directory */
    if (lstype != ListingType::Default && n > 3) || (lstype == ListingType::Default && n == 3) {
        return Ok(args[n - 1].clone());
    };

    match std::env::current_dir() {
        Ok(path) => {
            match path.to_str() {
                Some(name) => return Ok(name.to_string()),
                None => return Err(Error::from(ErrorKind::Other)),
            };
        }

        Err(e) => return Err(e),
    };
}

fn get_relative_path(abosulte: &String) -> String {
    let components: Vec<&str> = abosulte.split('/').collect();

    let n = components.len();
    return components[n - 1].to_string();
}

fn default_listing(pathname: &String, lstype: ListingType) -> Result<(), Error> {
    /* Check if the path is a file */
    match Path::new(pathname).is_file() {
        true => {
            println!("{}", pathname);
            return Ok(());
        }
        false => (),
    };

    let curr_dir: ReadDir;
    match read_dir(pathname) {
        Ok(d) => curr_dir = d,
        Err(e) => return Err(e),
    };

    /* Print . and .. if neccessary */
    if lstype == ListingType::All {
        println!(".");
        println!("..");
    };

    for pos_entry in curr_dir {
        let entry: DirEntry;
        match pos_entry {
            Ok(direntry) => entry = direntry,
            Err(e) => return Err(e),
        };

        let name: String;
        match entry.path().to_str() {
            Some(n) => name = n.to_string(),
            None => return Err(Error::from(ErrorKind::Other)),
        };

        /* Get the relative path */
        let realtive_name = get_relative_path(&name);

        /* Split the name into characters */
        let word: Vec<char> = realtive_name.chars().collect();
        if word[0] == '.' && lstype != ListingType::All {
            continue;
        }

        /* Print the name */
        println!("{}", realtive_name);
    }

    Ok(())
}

fn recursive_listing(pathname: &String, lstype: ListingType) -> Result<(), Error> {
    /* If pathname is a file, return from recursion */
    match Path::new(pathname).is_file() {
        true => return Ok(()),
        false => (),
    };

    let curr_dir: ReadDir;
    match read_dir(pathname) {
        Ok(d) => curr_dir = d,
        Err(e) => return Err(e),
    };

    println!("./{}:", pathname);
    if lstype == ListingType::RecursiveAll {
        println!(".");
        println!("..");
    };

    /* Check for possible entries in directory, or errors */
    for pos_entry in curr_dir {
        let entry: DirEntry;
        match pos_entry {
            Ok(direntry) => entry = direntry,
            Err(e) => return Err(e),
        };

        let name: String;
        match entry.path().to_str() {
            Some(n) => name = n.to_string(),
            None => return Err(Error::from(ErrorKind::Other)),
        };

        println!("{}", name);
        match recursive_listing(&name, lstype.clone()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }

    Ok(())
}

pub fn ls(args: &Vec<String>) -> Result<i32, ()> {
    let lstype = get_listing_type(args);

    let pathname: String;
    match get_pathname(args, lstype.clone()) {
        Ok(name) => pathname = name,
        Err(e) => {
            eprintln!("ls: unexpected error: {}", e);
            return Err(());
        }
    };

    match lstype {
        ListingType::Default | ListingType::All => {
            match default_listing(&pathname, lstype) {
                Ok(_) => return Ok(0),
                Err(e) => {
                    eprintln!("ls: unexpected error: {}", e);
                    return Err(());
                }
            };
        }

        ListingType::Recursive | ListingType::RecursiveAll => {
            match recursive_listing(&pathname, lstype) {
                Ok(_) => return Ok(0),
                Err(e) => {
                    eprintln!("ls: unexpected error: {}", e);
                    return Err(());
                }
            };
        }
    };
}
