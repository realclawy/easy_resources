use systemstat::{Platform, System};
mod init;
mod config;
mod error_handling;
use error_handling::throw_error;
mod options;
mod fetch;
mod structs;
mod color;

// I'm creating these like this because I already have too many libraries
// (and I don't want yet another one)
const ERROR: &str = "\x1b[31mError:\x1b[0m"; // "Error:" with red letters
const WARN: &str = "\x1b[33mWarning:\x1b[0m"; // "Warning:" with orange letters
pub fn clear() {
    println!("\x1B[2J\x1B[H"); // This clears the screen and puts the curson at the start
}

#[tokio::main]
async fn main() {
    let mut config_dir = init::config().await;
    let (option, set) = init::fetch_args().await;

    match option.as_str() {
        "run" | "r" => options::run(config_dir, set).await,
        "make" | "m" => options::make(config_dir, set).await,
        "delete" | "d" => options::delete(config_dir, set).await,
        "rename" => options::rename(config_dir, set).await,
        &_ => throw_error("option does not exist", 5)
    };
}
