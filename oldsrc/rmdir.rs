use std::io::{Error, ErrorKind};
use std::fs;

use crate::utils;

pub fn rmdir(dirs: Vec<String>) -> Result<(), std::io::Error> {
	/* Similar ro mkdir, if the user type "rmdir", then it should display 
	an warning and exit with failure */
	if dirs.is_empty() {
		println!("rmdir: missing operand");
		return Err(Error::from(ErrorKind::InvalidInput));
	};

	/* This is the return value that is gonna be changed if something
	bad happens */
	let mut ret_value: Result<(), std::io::Error> = Ok(());

	for dir in dirs {
		match utils::check_dir(&dir) {
			Ok(ret) => {
				match ret {
					/* Print the message for a path that doesn't exist */
					utils::PathStatus::IsNot => {
						println!("rmdir: failed to remove '{}': No such file or \
						directory", dir);
						ret_value = Err(Error::from(ErrorKind::NotFound));
					},
					
					/* Print the message for a path that points to a file */
					utils::PathStatus::IsFile => {
						println!("rmdir: failed to remove '{}': Not a \
						directory", dir);
						/* It should have been ErrorKind::NotADirectory, but
						is unstable */
						ret_value = Err(Error::from(ErrorKind::Other));
					}

					/* Do the actual removal */
					utils::PathStatus::IsDir => {
						match fs::remove_dir(&dir) {
							Ok(_) => (),
							Err(e) => {
								println!("rmdir: failed to remove '{}': \
								Directory not empty", dir);
								ret_value = Err(e);
							},
						};
					},
				};	
			},
			
			Err(e) => {
				println!("rmdir: unexpected error: {}", e);
				ret_value = Err(e);
			}
		}
	}

	ret_value
}