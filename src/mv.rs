use std::fs::rename;
use crate::utils::{get_string, set_path_status, PathStatus};


pub fn mv(args: &Vec<String>) -> Result<i32, ()> {
    
    let src: String;
    let target: String;

    match get_string(args, 2) {
        Some(path) => src = path,
        None => return Ok(-1),
    };

    match get_string(args, 3) {
        Some(path) => target = path,
        None => return Ok(-1),
    };

    /* Check if the source exists */
    match set_path_status(&src) {
        Ok(stat) => {
            match stat {
                PathStatus::IsNot => {
                    eprintln!("mv: cannor stat '{}': No such file or directory", src);
                    return Err(());
                },
                _ => {
                    // TODO: verifica daca trebuie sa concatenezi ceva la destinatie
                    match rename(src, target) {
                        Ok(_) => return Ok(0),
                        Err(e) => {
                            eprintln!("mv: unexpected error: {}", e);
                            return Err(());
                        },
                    };
                },
            };
        },
        Err(e) => {
            eprintln!("mv: unexpected error: {}", e);
            return Err(());
        },
    };

}
