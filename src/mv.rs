use crate::utils::{get_string, set_path_status, PathStatus};
use std::fs::rename;

pub fn mv(args: &Vec<String>) -> Result<i32, ()> {
    let src: String;
    let target: String;

    /* Get source */
    match get_string(args, 2) {
        Some(path) => src = path,
        None => return Ok(-1),
    };

    /* Get target/destination */
    match get_string(args, 3) {
        Some(path) => target = path,
        None => return Ok(-1),
    };

    /* Check if the source exists */
    match set_path_status(&src) {
        Ok(stat) => {
            match stat {
                /* If if doesn't throw an error */
                PathStatus::IsNot => {
                    eprintln!("mv: cannor stat '{}': No such file or directory", src);
                    return Err(());
                }
                _ => {
                    match rename(src, target) {
                        Ok(_) => return Ok(0),
                        Err(e) => {
                            eprintln!("mv: unexpected error: {}", e);
                            return Err(());
                        }
                    };
                }
            };
        }
        Err(e) => {
            eprintln!("mv: unexpected error: {}", e);
            return Err(());
        }
    };
}
