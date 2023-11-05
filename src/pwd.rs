use std::env;

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => {
            match path.to_str() {
                Some(name) => println!("{}", name),
                None => eprintln!(
                    "pwd: unexpected error: String conversion \
                failed"
                ),
            };
        }
        Err(e) => {
            println!("pwd: unexpected error: {}", e);
        }
    };
}
