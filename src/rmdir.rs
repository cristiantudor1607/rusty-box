use crate::utils::get_params;
use crate::utils::set_path_status;
use crate::utils::PathStatus;
use std::fs;

pub fn rmdir(args: &Vec<String>) -> Result<i32, ()> {
    /* If the user types rustybox rmdir */
    if args.len() == 2 {
        return Ok(-1);
    };

    /* Get the directories */
    let dirs = get_params(args, (2, usize::MAX));

    let mut error = false;

    for dir in dirs {
        match set_path_status(&dir) {
            Ok(stat) => {
                match stat {
                    /* If the path that we want to delete doesn't exist: */
                    PathStatus::IsNot => {
                        eprintln!(
                            "rmdir: failed to remove '{}': No such file or \
						directory",
                            dir
                        );
                        error = true;
                    }
                    /* If the path that we want to delete is a file: */
                    PathStatus::IsFile => {
                        eprintln!(
                            "rmdir: failed to remove '{}': Not a \
						directory",
                            dir
                        );
                        error = true;
                    }
                    /* If the path that we want to delete is a directory: */
                    PathStatus::IsDir => {
                        match fs::remove_dir(&dir) {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("rmdir: unexpected error: {}", e);
                                error = true;
                            }
                        };
                    }
                }
            }

            Err(e) => {
                eprintln!("rmdir: unexpected error: {}", e);
                error = true;
            }
        }
    }

    if error {
        return Err(());
    };

    Ok(0)
}
