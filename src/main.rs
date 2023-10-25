use std::env;
/* Suggestions: 1. Write a function for every command
                2. Start with the pwd command
                3. Continue with the other commands that do not have parameters
*/

fn pwd() {
    
    match env::current_dir() {
        Ok(path) => {
            let name = path.to_str().unwrap();
            println!("{}", name);
        },
        Err(e) => {
            println!("Ups... Unexpected error: {}", e);
        }
    };

}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "pwd" => pwd(),
        _ => todo!("Urmeste"),
    };
    // TODO 1: Read the command line arguments

    // TODO 2: If the first argument is pwd, call pwd()
}
