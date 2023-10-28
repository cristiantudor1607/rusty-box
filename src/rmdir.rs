use std::io::{Error, ErrorKind};

use crate::utils;


pub fn rmdir(dirs: Vec<String>) -> Result<(), std::io::Error> {
    /* Similar ro mkdir, if the user type "rmdir", then it should display 
    an warning and exit with failure */
    if dirs.is_empty() {
        println!("rmdir: missing operand");
        return Err(Error::from(ErrorKind::InvalidInput));
    };

    let mut ret: Result<(), std::io::Error> = Ok(());

    for dir in dirs {
        match utils::check_path(&dir) {
            Ok(result) => {

            },

            Err(e) => {
                println!("rmdir: unexpected error: {}", e);
                ret = Err(e);
            }
        }
    }
}