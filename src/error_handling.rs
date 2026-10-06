use std::io;

use crate::ERROR;
use crate::WARN;

pub fn throw_error(error: &str, code: i32) {
    eprintln!("{ERROR} {error}");
    std::process::exit(code);
}

pub async fn throw_warn_and_wait_for_confirmation(warning: &str) {
    eprintln!("{WARN} {warning}");

    match io::stdin().read_line(&mut String::new()) {
        Ok(_) => {},
        Err(_) => throw_error("very funny.", -1),
    }
}
