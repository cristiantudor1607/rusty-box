use std::fs::{set_permissions, File, Metadata};
use std::os::unix::fs::PermissionsExt;

use crate::utils::get_string as get_filename;

#[derive(Debug, PartialEq, Eq)]
pub enum UserType {
    ForOwner,
    ForGroup,
    ForOthers,
    ForAll,
}

pub enum ChmodType {
    Add,
    Del,
}

fn octal_to_u32(octal: u32) -> u32 {
    return (octal % 10) * 1 + ((octal / 10) % 10) * 8 + ((octal / 100) % 10) * 64;
}

fn set_users(args: &Vec<String>) -> Option<Vec<UserType>> {
    /* Split the 3rd argument to chars */
    let w: Vec<_> = args[2].chars().collect();

    /* Create a vector to add users in it */
    let mut ret: Vec<UserType> = Vec::new();

    for entity in w.clone() {
        /* The user type field can be empty, but after +/-, there always
        should be something, so stop here */
        if entity == '+' || entity == '-' {
            break;
        }

        match entity {
            'u' => ret.push(UserType::ForOwner),
            'g' => ret.push(UserType::ForGroup),
            'o' => ret.push(UserType::ForOthers),
            'a' => ret.push(UserType::ForAll),
            _ => (),
        };
    }

    /* If the return vector is empty, then w should contain only r, w, and x */
    if ret.is_empty() {
        if !w.contains(&'r') && !w.contains(&'w') && !w.contains(&'x') {
            return None;
        }
    }

    return Some(ret);
}

/* The function won't consider Numeric Format */
fn add_or_delete_perm(args: &Vec<String>) -> ChmodType {
    /* Split the 3rd argument */
    let w: Vec<_> = args[2].chars().collect();

    if w.contains(&'+') {
        return ChmodType::Add;
    } else {
        return ChmodType::Del;
    }
}

fn get_current_perms(filename: &String) -> Result<u32, std::io::Error> {
    /* Get the metadata */
    let metadata: Metadata;
    match get_metadata(filename) {
        Ok(m) => metadata = m,
        Err(e) => return Err(e),
    };

    /* Get current permissions */
    let perms = metadata.permissions().mode();

    return Ok(perms);
}

fn get_wanted_perms(args: &Vec<String>) -> u32 {
    let mut diff: u32 = 0o0;

    /* Split the argument that contains the permissions*/
    let c: Vec<_> = args[2].chars().collect();

    let read = 'r';
    let write = 'w';
    let execute = 'x';

    if c.contains(&read) {
        diff += 0o4;
    };

    if c.contains(&write) {
        diff += 0o2;
    };

    if c.contains(&execute) {
        diff += 0o1;
    };

    return diff;
}

fn get_updated_perms(old_perms: &u32, diff: &u32, op: &ChmodType) -> u32 {
    match op {
        ChmodType::Add => {
            if old_perms + diff > 0o7 {
                return 0o7;
            } else {
                return old_perms + diff;
            };
        }
        ChmodType::Del => {
            if old_perms <= diff {
                return 0o0;
            } else {
                return old_perms - diff;
            };
        }
    }
}

fn get_new_perms(old_perms: u32, diff: u32, users: Vec<UserType>, op: ChmodType) -> u32 {
    let mut total: u32 = old_perms;
    for user in users {
        let mut new_perms = 0o0;
        /* If the perms are changed for owner or all */
        if user == UserType::ForOwner || user == UserType::ForAll {
            /* Calculate and add new perms */
            let owner = (total / (8 * 8)) % 8;
            let new = get_updated_perms(&owner, &diff, &op);
            new_perms += new * 64;
        } else {
            /* If they aren't changed, add the old perms */
            new_perms += ((total / (8 * 8)) % 8) * 64;
        };
        /* If the perms are changed for group or all */
        if user == UserType::ForGroup || user == UserType::ForAll {
            /* Calculate and add new perms */
            let group = (total / 8) % 8;
            let new = get_updated_perms(&group, &diff, &op);
            new_perms += new * 8;
        } else {
            /* If they aren't changed, add the old perms */
            new_perms += ((total / 8) % 8) * 8;
        };
        /* If the perms are changed for others or all */
        if user == UserType::ForOthers || user == UserType::ForAll {
            /* Calculate and add new perms */
            let others = total % 8;
            let new = get_updated_perms(&others, &diff, &op);
            new_perms += new;
        } else {
            /* If they aren't changed, add the old perms */
            new_perms += total % 8;
        };

        /* The new perms become the current perms for the next iteration */
        total = new_perms;
    }

    return total;
}

pub fn get_metadata(filename: &String) -> Result<Metadata, std::io::Error> {
    /* Open the file */
    let file: File;
    match File::open(filename) {
        Ok(f) => file = f,
        Err(e) => return Err(e),
    };

    /* Load metadata */
    let metadata: Metadata;
    match file.metadata() {
        Ok(m) => metadata = m,
        Err(e) => return Err(e),
    };

    Ok(metadata)
}

/* Check link 3 from README */
fn set_perm(filename: &String, perms: u32) -> Result<(), std::io::Error> {
    /* Get the metadata of the file */
    let metadata: Metadata;
    match get_metadata(filename) {
        Ok(m) => metadata = m,
        Err(e) => return Err(e),
    };

    /* Load permissions */
    let mut new_perms = metadata.permissions();
    new_perms.set_mode(perms);

    match set_permissions(filename, new_perms) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    };
}

pub fn chmod(args: &Vec<String>) -> Result<i32, ()> {
    /* Check the user input */
    if args.len() != 4 {
        return Ok(-1);
    };

    /* Get the filename */
    let filename = get_filename(args, 3).unwrap();

    /* Get the types of users to apply permissions */
    let users: Vec<UserType>;
    let mut input_perms: u32 = 0;
    match set_users(args) {
        Some(u) => users = u,
        /* If it returns None, check if the input can be turned into an u32 */
        None => {
            match args[2].parse() {
                Ok(p) => {
                    input_perms = p;
                    /* Initialize users with an empty vector */
                    users = Vec::new();
                }
                /* If the input can't be converted to a number, then it is an
                invalid command */
                Err(_) => return Ok(-1),
            };
        }
    };

    /* If users is empty, then the provided format is a numeric format */
    if users.is_empty() {
        let new_perms = octal_to_u32(input_perms);
        /* Set the permissions */
        match set_perm(&filename, new_perms) {
            Ok(_) => return Ok(0),
            Err(e) => {
                eprintln!("chmod: unexpected error: {}", e);
                return Err(());
            }
        };
    };

    let old_perms: u32;
    match get_current_perms(&filename) {
        Ok(p) => old_perms = p,
        Err(e) => {
            eprintln!("chmod: unexpected error: {}", e);
            return Err(());
        }
    };
    /* Get the wanted permissions as a number */
    let diff = get_wanted_perms(args);

    /* Get the type of modification: remove or add permissions */
    let modif_type = add_or_delete_perm(args);

    /* Get the new permissions */
    let new_perms = get_new_perms(old_perms, diff, users, modif_type);

    match set_perm(&filename, new_perms) {
        Ok(_) => return Ok(0),
        Err(e) => {
            println!("chmod : unexpected error: {}", e);
            return Err(());
        }
    };
}
