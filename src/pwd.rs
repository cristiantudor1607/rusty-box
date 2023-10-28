use std::env;

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => {
            // TODO: Get rid of the unwrap thing
            let name = path.to_str().unwrap();
            println!("{}", name);
        },
        Err(e) => {
            println!("Ups... Unexpected error: {}", e);
        }
    };

}