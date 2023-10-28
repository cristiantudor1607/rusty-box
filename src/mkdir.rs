use std::io::{Error, ErrorKind};
use std::fs::DirBuilder;
use crate::utils;

pub fn create_dir(path: &String) -> Result<(), std::io::Error> {
    /* Create the builder */
    let mut builder = DirBuilder::new();

    /* Set recursive field to true */
    builder.recursive(true);

	/* Create the new directory */
    match builder.create (path) {
        Ok(_) => return Ok(()),
		Err(e) => return Err(e),
    };
}

pub fn mkdir(dirs: Vec<String>) -> Result<(), std::io::Error> {
	
	/* If the user typed just "mkdir", print a message and throw an error */
	if dirs.is_empty() {
        println!("mkdir: missing operand");
		let custom =	Error::from(ErrorKind::InvalidInput);
		return Err(custom);
    };

	/* If all the strings in the dirs Vector are valid paths, the
	ret value will remain Ok(()). If there is at least one path
	that already exists, it will be changed to an error */
    let mut ret: Result<(), std::io::Error> = Ok(());

	for dir in dirs {
		match utils::check_path(&dir) {
			Ok(result) => {
				match result {
					/* If the path already exists, just print an error and set 
					the ret variable */
					true => {
						println!("mkdir: cannot create directory '{}': File exists", dir);
						ret = Err(Error::from(ErrorKind::AlreadyExists));
					},
					/* If the path doesn't exists, create the directory and
					check for other errors */
					false => {
						match create_dir(&dir) {
							Ok(_) => (),

							Err(e) => {
								println!("mkdir: unexpected error: {}", e);
								ret = Err(e);
							},
						};
					},
				};
			},
			
			Err(e) => {
				println!("mkdir: unexpected error: {}", e);
				ret = Err(e);
			},
		}
	};

	ret
}