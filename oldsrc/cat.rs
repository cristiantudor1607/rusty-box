use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::io;


pub fn open_file(filename: &String) -> Option<File> {
    let file = File::open(filename);
    let f = match file {
        Ok(file) => Some(file),
        Err(_) => None,
    };

    f
}

pub fn read_file(file: Option<File>) -> Result<Lines<BufReader<File>>, ()>{
    match file {
        /* For this part, the approach is similar with the one used in error
        propagation exercise in the 2nd lab */
        Some(file) => {
            let content = BufReader::new(file).lines();
            return Ok(content);
        },
        None => {
            return Err(());
        },
    };
}

pub fn print_content(buffer: Result<Lines<BufReader<File>>, ()>) -> bool {
    match buffer {
        /* If the buffer was created and loaded succesfully, print it's
        content */
        Ok(buffer) => {
            for line in buffer {
                match line {
                    Ok(text) => println!("{}", text),
                    Err(_) => return false,
                }
            }

            return true;
        },

        /* If there have been an issuse, do nothing and let the caller of the
        function know that there was a problem */
        Err(()) => {
            return false;
        }
    }
}

pub fn cat(files: Vec<String>) -> bool {
    /* If you type just "cat" in terminal, it enters in an infinte loop and 
    it can be stopped only by interrupting the process it creates; While in the
    loop, you can type strings and it will imediately display the string on the
    next line*/
    if files.is_empty() {
        loop {
            /* https://fitech101.aalto.fi/programming-languages/rust/8-interaction-input-and-os/#:~:text=To%20read%20user%20input%20in,written%20on%20the%20command%20line. */
            let mut fake_input = String::new();
            match io::stdin().read_line(&mut fake_input) {
                Ok(_) => {
                    print!("{}", fake_input);
                },

                Err(e) => {
                    println!("Ups... Unexpected error: {}", e);
                    return false;
                }
            }
        };
    };

    /* If one of the file doesn't exist, the code will be set to false, and 
    the function will return false at the end, after printing the contents of
    the existing files */
    let mut code: bool = true;
    
    for filename in files {
        /* Open the file */
        let file_struc: Option<File> = open_file(&filename);
        
        /* Read it's content in a buffer */
        let buf = read_file(file_struc);
        
        /* Print content or the cat error message */
        match print_content(buf) {
            false => {
                println!("cat: {}: No such file or directory", filename);
                code = false;
            },

            true => (),
        };
    };

    code

}