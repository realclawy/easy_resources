use crate::error_handling::*;
use std::env;
use std::path::PathBuf;
use std::fs;

pub async fn config() -> PathBuf {
    let mut config_dir: PathBuf;

    match dirs::config_dir() {
        Some(config_path) => config_dir = config_path,
        None => {
            throw_error("config directory is not reachable. This is probably either the library's fault or your computer's.", 1);
            unreachable!();
        }
    }

    config_dir.push("easy_resources");

    if config_dir.exists() { 
        config_dir
    } else {
        match fs::create_dir(&config_dir) {
            Ok(_) => {},
            Err(err) => {
                    throw_error(
                        format!("config directory is not writtable to. It might be fine to run with admin, but since this is a random program you should probably not do that...\nThis is what the error says: {err}").as_str(), 2
                    ); unreachable!()
                }
        }
        config_dir
    }
}

/* What this returns:
 * the option ran
 * the set in which this is ran
*/
pub async fn fetch_args() -> (String, Option<String>) {
    let args: Vec<String> = env::args().collect();
    let arg_num = args.len() - 1;
    let mut args = args.into_iter();
    args.next(); // Consume the path thing

    if arg_num == 1 {
        let arg = args.next().unwrap().to_lowercase();
        match arg.as_str() {
            "help" | "h" | "-h" | "--help" | "list" | "ls" | "l" => { (arg, None) },
            &_ => { throw_error("Not enough or incorrect arguments provided", 3); unreachable!() },
        }
    } else if arg_num == 2 { // the vector had an additional argument, which is the command call path
        (args.next().unwrap(), Some(args.next().unwrap()))
    } else if arg_num < 1 {
        throw_error("Not enough arguments provided", 3); unreachable!()
    } else {
        throw_error("Too many arguments...", 4); unreachable!()
    }
}
