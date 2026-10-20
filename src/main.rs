// lib
mod init; // starting processes
mod config; // configuration handling
mod error_handling;
use error_handling::throw_error;
mod options; // Options when the command is ran
mod fetch; // getting the command arguments
mod structs;
mod color; // Colour logic
mod screen; // Screen handling logic

// I'm creating these like this because I already have too many libraries
// (and I don't want yet another one)
const ERROR: &str = "\x1b[31mError:\x1b[0m"; // "Error:" with red letters
const WARN: &str = "\x1b[33mWarning:\x1b[0m"; // "Warning:" with orange letters

#[tokio::main]
async fn main() {
    let config_dir = init::config().await;
    let (option, set) = init::fetch_args().await;

    match option.as_str() {
        "run" | "r" => options::run(config_dir, set).await,
        "make" | "m" => options::make(config_dir, set).await,
        "delete" | "d" => options::delete(config_dir, set).await,
        "rename" => options::rename(config_dir, set).await,
        &_ => throw_error("option does not exist", 5)
    };
}
