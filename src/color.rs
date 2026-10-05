use std::io::stdin;

use crate::error_handling::throw_error;

// Each code represents a colour (british and american English mixup :/)
const BLACK: &str = "\x1b[30m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const ORANGE: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const PURPLE: &str = "\x1b[35m";
const AQUA: &str = "\x1b[36m"; // Yes, I AM calling light blue "aqua" because of that one game...
const WHITE: &str = "\x1b[37m";
const END: &str = "\x1b[0m";

pub async fn get_painted(col: &str, string: &str) -> String {
    match col {
        "black" => format!("{BLACK}{string}{END}"),
        "red" => format!("{RED}{string}{END}"),
        "green" => format!("{GREEN}{string}{END}"),
        "orange" => format!("{ORANGE}{string}{END}"),
        "blue" => format!("{BLUE}{string}{END}"),
        "purple" => format!("{PURPLE}{string}{END}"),
        "aqua" => format!("{AQUA}{string}{END}"),
        "white" => format!("{WHITE}{string}{END}"),

        &_ => {throw_error("WHAT THAT SHOULDN'T HAPPEN", -1).await; unreachable!()}
    }
}

pub async fn ask() -> String {
    let mut buffer = String::new();
    println!("Choose a colour for your widget from these:");
    stdin().read_line(&mut buffer).expect("DUDE");

    "a".to_string()
}
