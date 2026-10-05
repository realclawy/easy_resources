use crate::error_handling::*;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

/* Things this returns:
 * config path (PathBuf)
 * a list of all existing .conf files
*/
pub async fn read(config_path: PathBuf) -> (PathBuf, Vec<String>) {
    let paths = config_path.read_dir().unwrap_or_else(|err| {
        eprintln!("No idea what happened here just read Rust's error:\n{err}");
        std::process::exit(3);
    });

    let mut sets: Vec<String> = Vec::new();
    for path in paths {
        let path = path.unwrap_or_else(|err|{eprintln!("error1177:\n{err}");std::process::exit(57)});
        if path.path().is_file() {
            let path = path.file_name().display().to_string();
            match is_conf_file(path).await {
                Some(name) => sets.push(name),
                None => { continue; }
            }
        } else {
            continue;
        }
    }
    (config_path, sets)
}

// If the termination is .conf it returns the file's name. If it isn't it returns nothing
pub async fn is_conf_file(file: String) -> Option<String> {
    match file.rsplit_once(".") {
        Some((name, terminator)) => {
            if terminator == "conf" {
                if !name.is_empty() {
                    Some(name.to_string())
                } else {
                    throw_warn_and_wait_for_confirmation("you have a stray '.conf' with no name in the configuration. Remove it to get rid of this warning. Press any key to continue.").await;
                    thread::sleep(Duration::new(2, 0)); // wait 2 seconds to call the user's attention.
                    None
                }
            } else {
                None
            }
        }
        None => None
    }
}
