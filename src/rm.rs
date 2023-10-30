use crate::utils::RmOption as RmOption;

pub fn set_options(args: &Vec<String>) -> Option<RmOption> {
    let opt = args[2].as_str();

    if opt == "-r" || opt == "-R" || opt == "-recursive" {
        return Some(RmOption::Recursive);
    };

    if opt == "-d" || opt == "-dir" {
        return Some(RmOption::Dir);
    };

    return None;
}