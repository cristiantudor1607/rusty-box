use std::fs::hard_link;
use std::os::unix::fs::symlink;

use crate::utils::get_string;

#[derive(Debug, PartialEq, Eq)]
pub enum LinkType {
    SoftLink,
    HardLink,
}

fn lnget_link_type(args: &Vec<String>) -> Option<LinkType> {
    /* If there is an option, it should be at index 2 */
    let n = args.len();
    let pos_option = &args[2];
    if pos_option == "-s" || pos_option == "--symbolic" {
        if n == 5 {
            return Some(LinkType::SoftLink);
        } else {
            return None;
        };
    };

    /* If the length is 5 and the SoftLink option wasn't returned, then
    the option provided is an invalid one */
    if n == 5 {
        return None;
    }

    /* If it reaches this point, it should be a hard link */
    return Some(LinkType::HardLink);
}

pub fn ln(args: &Vec<String>) -> Result<i32, ()> {
    let n = args.len();

    if n < 4 || n > 5 {
        return Ok(-1);
    };

    let link: LinkType;
    match lnget_link_type(args) {
        Some(t) => link = t,
        None => return Ok(-1),
    };

    /* Get the source for the link and the name of the link */
    let src: String;
    let name: String;
    match link {
        LinkType::HardLink => {
            match get_string(args, 2) {
                Some(s) => src = s,
                /* It won't return None, because the args were already verified,
                but I have to return something, because the compiler will later
                think I use src and name uninitialized */
                None => return Ok(1),
            };

            match get_string(args, 3) {
                Some(s) => name = s,
                /* Same as above */
                None => return Ok(1),
            };
        }

        LinkType::SoftLink => {
            match get_string(args, 3) {
                Some(s) => src = s,
                None => return Ok(1),
            };

            match get_string(args, 4) {
                Some(s) => name = s,
                None => return Ok(1),
            };
        }
    }

    /* Now, let's create the link */
    match link {
        /* For hard links, I'll use the hard_link function */
        LinkType::HardLink => {
            match hard_link(src, name) {
                Ok(_) => return Ok(0),
                Err(e) => {
                    eprintln!("ln: unexpected error: {}", e);
                    return Err(());
                }
            };
        }
        LinkType::SoftLink => {
            /* The compiler suggested to use symlink instead of soft_link */
            match symlink(src, name) {
                Ok(_) => return Ok(0),
                Err(e) => {
                    eprintln!("ln: unexpected error: {}", e);
                    return Err(());
                }
            };
        }
    };
}
