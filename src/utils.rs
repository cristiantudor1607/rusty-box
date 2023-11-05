use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::usize;

#[derive(Debug, PartialEq, Eq)]
pub enum PathStatus {
    IsDir,
    IsFile,
    IsNot,
}

pub fn set_path_status(path: &String) -> Result<PathStatus, std::io::Error> {
    let path_struc = Path::new(path);

    match path_struc.try_exists() {
        Ok(ret) => {
            /* Test if the path doesn't exist */
            if ret == false {
                return Ok(PathStatus::IsNot);
            };
        }
        /* Check for unexpected errors */
        Err(e) => return Err(e),
    };

    match path_struc.is_dir() {
        true => return Ok(PathStatus::IsDir),
        false => return Ok(PathStatus::IsFile),
    };
}

pub fn open_file(filename: &String) -> Result<File, std::io::Error> {
    let file = File::open(filename);
    match file {
        Ok(fobj) => return Ok(fobj),
        Err(e) => {
            eprintln!("unexpected error: {}", e);
            return Err(e);
        }
    };
}

pub fn read_file(file: File) -> Lines<BufReader<File>> {
    return BufReader::new(file).lines();
}

pub fn get_params(args: &Vec<String>, range: (usize, usize)) -> Vec<String> {
    let inf = range.0;
    let sup = range.1;

    /* If the upper bound parameter is the MAX size for usize, we want to
    get all the elements of the Vector, starting with lower bound index
    and ending with the last element of the Vector */
    if sup == usize::MAX {
        /* Use clone to avoid making changes to the original Vector, and then call
        drain method to extract the elements from the range */
        let params = args.clone().drain(inf..).collect::<Vec<String>>();
        return params;
    }

    /* If the upper bound is not MAX, we extract from a limited range */
    let params = args.clone().drain(inf..sup).collect();
    return params;
}

pub fn get_string(args: &Vec<String>, index: usize) -> Option<String> {
    if index >= args.len() {
        return None;
    } else {
        return Some(args[index].clone());
    };
}
