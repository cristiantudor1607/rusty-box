use std::usize;
use std::path::Path;

pub fn extract_params_inrange(args: &Vec<String>, inf: usize, sup: usize) -> Vec<String> {
    
    /* If the upper bound parameter is the MAX size for usize, we want to
    extract all the elements of the Vector, starting with lower bound position
    and finishing with the last element of the Vector */ 
    if sup == usize::MAX {
        /* Use clone to avoid making changes to the original Vector, and then call
        drain method to extract the wanted range */
        let params = args.clone().drain(inf..).collect::<Vec<String>>();
        return params;
    }
    
    /* If the upper bound is not MAX, we extract from an actual range */
    let params = args.clone().drain(inf..sup).collect();
    return params;
}

/**
 * check_path function returns a boolean value which describe if the
 * path exists or not (true or false), or drops and error is try_exists
 * fail
 */
pub fn check_path(path: &String) -> Result<bool, std::io::Error> {
    let my_path = Path::new(path);

    match my_path.try_exists() {
        Ok(ret) => {
            return Ok(ret);
        },
        Err(e) => {
            return Err(e);
        },
    };
}