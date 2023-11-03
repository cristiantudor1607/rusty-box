use std::io::{Error, ErrorKind};
use std::fs::rename;

use crate::utils::check_path;

pub fn get_src_and_dest(args: &Vec<String>) -> Result<(String, String), std::io::Error> {
    let n = args.len();
    
    /* If the user type just "mv" */
    if n == 2 {
        eprintln!("mv: missing file operand");
        return Err(Error::from(ErrorKind::InvalidInput));
    };

    /* If the user type "mv ~something~" */
    if n == 3 {
        eprintln!("mv: missing destination file operand after '{}'", args[2]);
        return Err(Error::from(ErrorKind::InvalidInput));
    };

    let src = args[2].clone();
    let dest = args[3].clone();
    return Ok((src, dest));
}

pub fn mv(args: &Vec<String>) -> Result<(), std::io::Error> {
    let src: String;
    let dest: String;

    /* Set the source and destination */
    match get_src_and_dest(args) {
        Ok(ret_tuple) => (src, dest) = ret_tuple,
        Err(e) => return Err(e), 
    };

    /* Check if the source exists */
    match check_path(&src) {
        Ok(ret) => {
            match ret {
                false => {
                    eprint!("mv: cannor stat '{}': No such file or directory", src);
                    return Err(Error::from(ErrorKind::NotFound));
                },
                true => (),
            };
        },

        Err(e) => return Err(e),
    }

    /* TODO: check destination */

    /* Rename */
    match rename(src, dest) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    };

}