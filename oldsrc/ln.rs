use std::fs::hard_link;
use std::os::unix::fs::symlink;

use std::io::{Error, ErrorKind};
use crate::utils::LinkType as LinkType;

/* get_link_type function will be called after we make sure that there are
at least 4 strings in the Vector (rustybox, ln, source, link name) */
pub fn get_link_type(args: &Vec<String>) -> LinkType {
    /* -s should be on the 2nd position */
    let possible_opt = &args[2];
    if possible_opt == "-s" || possible_opt == "--symbolic" {
        return LinkType::SoftLink;
    };

    return LinkType::HardLink;
}

/* get_src_and_name function will be called after we make sure that the
parameters of the ln command are good, and after we set the option variable */
pub fn get_src_and_name(args: &Vec<String>, opt: &LinkType) -> (String, String) {
    let src: &String;
    let name: &String;
    
    match opt {
        /* If the link is a HardLink, then the src is at index 2 and name at
        index 3 */
        LinkType::HardLink => {
            src = &args[2];
            name = &args[3];
        },
        /* If the link is a SoftLink, then src is at index 3 and name is at
        index 4 */
        LinkType::SoftLink => {
            src = &args[3];
            name = &args[4];
        },
    };

    return (src.to_string(), name.to_string());
}

pub fn ln(args: &Vec<String>) -> Result<(), std::io::Error> {
    let n = args.len();
    if n == 1 || n == 2 || n == 3 {
        eprintln!("Invalid command");
        return Err(Error::from(ErrorKind::InvalidInput));
    };

    let opt: LinkType = get_link_type(args);
    if n == 4 && opt == LinkType::SoftLink {
        eprintln!("Invalid command");
        return Err(Error::from(ErrorKind::InvalidInput));
    };

    let (src, lname) = get_src_and_name(&args, &opt);

    match opt {
        LinkType::HardLink => {
            match hard_link(src, lname) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            }
        },
        LinkType::SoftLink => {
            /* The compiler suggested to use symlink istead oh soft_link */
            match symlink(src, lname) {
                Ok(_) => return  Ok(()),
                Err(e) => return Err(e),
            }
        },
    };

}