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

        &_ => {throw_error("WHAT THAT SHOULDN'T HAPPEN", -1); unreachable!()}
    }
}

pub async fn ask() -> String {
    let mut buffer = String::new();

    println!("Choose a colour for your widget from these:");
    println!("{} {}\n{} {}\n{} {}\n{} {}",
        get_painted("black", "black: 1").await,
        get_painted("red", "red: 2").await,
        get_painted("green", "green: 3").await,
        get_painted("orange", "orange: 4").await,
        get_painted("blue", "blue: 5").await,
        get_painted("purple", "purple: 6").await,
        get_painted("aqua", "aqua: 7").await,
        get_painted("white", "white: 8").await
        );
    print!("choose with the colour's number (1-8): ");
    let buffer = loop {
        stdin().read_line(&mut buffer).expect("DUDE");
        println!();

        match buffer.trim().parse::<u32>() {
            Ok(number) => {
                if (1..=8).contains(&number) {
                    break number
                } else {
                    print!("Not a valid number... Press again: ");
                    buffer.clear();
                }
            },
            Err(_) => {
                print!("Not a number... Press again: ");
                buffer.clear();
            }
        }
    }; // now the buffer is parsed... Supposedly, AI did something really strange tbh

    match buffer {
        1 => "black".to_string(),
        2 => "red".to_string(),
        3 => "green".to_string(),
        4 => "orange".to_string(),
        5 => "blue".to_string(),
        6 => "purple".to_string(),
        7 => "aqua".to_string(),
        8 => "white".to_string(),
        _ => panic!("this can't be happening :o")
    }
}
