use std::fs::DirBuilder;
use crate::utils::get_params;
use crate::utils::set_path_status;
use crate::utils::PathStatus;

pub fn create_newdir(path: &String) -> Result<(), std::io::Error> {
    /* Create the builder */
    let mut builder = DirBuilder::new();

    /* Set recursive field to true */
    builder.recursive(true);

    /* Create the new directory */
    match builder.create(path) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    };
}

pub fn mkdir(args: &Vec<String>) -> Result<i32, ()> {
    /* If the user types just "mkdir", then it is an invalid command */
    if args.len() == 2 {
        return Ok(-1);
    };

    /* Get the directories from the args list */
    let dirs = get_params(args, (2, usize::MAX));

    let mut error: bool = false;

    for dir in dirs {
        match set_path_status(&dir) {
            Ok(stat) => {
                match stat {
                    /* If the path already exists, print an error and do nothing */
                    PathStatus::IsDir => {
                        eprintln!("mkdir: {}: Directory exists", dir);
                        error = true;
                    },
                    PathStatus::IsFile => {
                        eprintln!("mkdir: {}: File exists", dir);
                        error = true;
                    },
                    /* If the path doesn't exist we can create it */
                    PathStatus::IsNot => {
                        match create_newdir(&dir) {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("mkdir: unexpected error: {}", e);
                                error = true;
                            }
                        };
                    }
                };
            }
            Err(e) => {
                eprintln!("mkdir: unexpected error: {}", e);
                error = true;
            }
        };
    }

    /* Throw the error */
    if error {
        return Err(());
    };

    Ok(0)
}
