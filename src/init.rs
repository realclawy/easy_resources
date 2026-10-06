use crate::error_handling::*;
use crate::config;
use std::env;
use std::path::PathBuf;
use std::fs;

pub async fn config() -> (PathBuf, Vec<String>) {
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
        config::read(config_dir).await
    } else {
        match fs::create_dir(&config_dir) {
            Ok(_) => {},
            Err(err) => {
                    throw_error(
                        format!("config directory is not writtable to. It might be fine to run with admin, but since this is a random program you should probably not do that...\nThis is what the error says: {err}").as_str(), 2
                    ); unreachable!()
                }
        }
        config::read(config_dir).await
    }
}

/* What this returns:
 * the option ran
 * the set in which this is ran
*/
pub async fn fetch_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    let arg_num = args.len();
    let mut args = args.into_iter();
    args.next(); // Consume the path thing

    if arg_num == 3 { // the vector had an additional argument, which is the command call path
        (args.next().unwrap(), args.next().unwrap())
    } else if arg_num < 2 {
        throw_error("not enough arguments provided", 3); unreachable!()
    } else {
        throw_error("too many arguments...", 4); unreachable!()
    }
}
