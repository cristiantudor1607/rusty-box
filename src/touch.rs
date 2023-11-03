use std::path::Path;
use std::fs::File;
use std::fs;
use crate::utils::get_string as get_filename;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TouchDateType {
    Modify,
    Access,
    Create,
    NoCreate,
}

fn which_date(args: &Vec<String>) -> Option<TouchDateType> {
    let mut date = TouchDateType::Create;
    
    /* Check for options */
    match args[2].as_str() {
        "-a" => date = TouchDateType::Access,
        "-m" => date = TouchDateType::Modify,
        "-c" | "--no-creat" => date = TouchDateType::NoCreate,
        _ => (),
    };

    /* If it finds an option, but the command is like 
    "rustybox touch -c", then it is an invalid command */
    if date != TouchDateType::Create && args.len() == 3 {
        return None;
    }

    return Some(date);
}

fn touch_unexisting(name: &String, o: TouchDateType) -> std::io::Result<bool> {
    /* Switch to the case where file doesn't exist */
    match Path::new(name).try_exists() {
        Ok(status) => {
            /* If the NoCreate flag isn't set, create the file */
            if status == false && o != TouchDateType::NoCreate {
                match File::create(name) {
                    Ok(_) => return Ok(true),
                    Err(e) => {
                        eprintln!("touch: unexpected error: {}", e);
                        return Err(e);
                    },
                };
            } else if status == false && o == TouchDateType::NoCreate {
                return Ok(true);
            };
        },
        /* Throw the error */
        Err(e) => {
            eprintln!("touch: unexpected error: {}", e);
            return Err(e);
        },
    };

    Ok(false)
}

fn update_acces_time(filename: &String) -> Result<(), std::io::Error>{
    /* Read the file to modify acces time */
    match fs::read(filename) {
        Ok(_) => return Ok(()),
        Err(e) => {
            eprintln!("touch: unexpected error: {}", e);
            return Err(e);
        },
    };
}

pub fn touch(args: &Vec<String>) -> Result<i32, ()> {
    /* Check if the command is invalid */
    let n = args.len();
    if n < 3 {
        return Ok(-1);
    };

    /* Set the options */
    let opt: TouchDateType;
    match which_date(args) {
        Some(date) => opt = date,
        None => return Ok(-1),
    };

    /* Get the filename */
    let filename: String;
    let index: usize;

    match opt {
        TouchDateType::Create => index = 2,
        _ => index = 3,
    };
    match get_filename(args, index) {
        Some(name) => filename = name,
        None => return  Err(()),
    };

    /* If the file doesn't exists, create it, unless the NoCreate flag is set */
    match touch_unexisting(&filename, opt.clone()) {
        Ok(ret) => {
            if ret == true {
                return Ok(0);
            };
        },
        Err(_) => return Err(()),
    };
    
    match opt {
        TouchDateType::Access => {
            match update_acces_time(&filename) {
                Ok(_) => return Ok(0),
                Err(_) => return Err(()),
            };
        },
        _ => (),
    };

    Ok(0)
}