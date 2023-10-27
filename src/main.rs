use std::env;
use std::process::ExitCode;
use std::usize;
use std::fs::File;

/* Suggestions: 1. Write a function for every command
                2. Start with the pwd command
                3. Continue with the other commands that do not have parameters
*/

// All the methods I use are taken from doc.rust-lang.org

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

fn extract_params_inrange(args: &Vec<String>, inf: usize, sup: usize) -> Vec<String> {
    
    /* If the upper bound parameter is the MAX size for usize, we want to
    extract all the elements of the vector, starting with lower bound position
    and finishing with the last element of the vector */ 
    if sup == usize::MAX {
        let params = args.clone().drain(inf..).collect::<Vec<String>>();
        return params;
    }
    
    /* If the upper bound is not MAX, we extract from an actual range */
    let params = args.clone().drain(inf..sup).collect();
    return params;
}

fn open_file(filename: &String) -> Option<File> {
    let file = File::open(filename);
    let f = match file {
        Ok(file) => Some(file),
        Err(_) => None,
    };

    f
}

fn read_file(file: Option<File>) -> Result<Lines<BufReader<File>>, bool>{
    match file {
        Some(file) => {

        },

    }

}



fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "pwd" => {
            pwd();
            return ExitCode::SUCCESS;
        },
        "cat" => {
            let params = extract_params_inrange(&args, 2, usize::MAX);
            println!("{:?}", args);
            println!("{:?}", params);
            return ExitCode::SUCCESS;
        },
        _ => {
            println!("Invalid command");
            return ExitCode::FAILURE;
        },
    };

}
