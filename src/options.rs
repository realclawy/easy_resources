use crate::error_handling::throw_error;
use std::{io::stdin, path};
use path::PathBuf;
use systemstat::System;
use std::fs;

pub async fn delete(config_dir: &mut PathBuf, set: String) {
    config_dir.push(set + ".conf");

    if config_dir.exists() {
        fs::remove_file(config_dir).unwrap(); //  TODO: error handle it (I don't feel like doing so rn)
    } else {
        throw_error("That file doesn't exist...", 5).await; unreachable!()
    }
}

pub async fn rename(config_dir: &mut PathBuf, set: String) {
    let mut config_file = config_dir.clone();
    config_file.push(set + ".conf");

    let mut new_name = String::new();
    std::io::stdin().read_line(&mut new_name).expect("dude");

    if config_file.exists() {
        if !path::PathBuf::from(format!("{:?}.conf", &config_dir)).exists() {
            todo!()
        } else {
            throw_error("set that already has that name was found.", 6).await;
        }
    } else {
        throw_error("That file doesn't exist...", 5).await; unreachable!()
    }
}

pub async fn make(config_dir: &mut PathBuf, set: String) {
    let mut config_file_buffer = String::new();


    print!("do you want the hour to be displayed? (Y/n) ");
    let mut have_hour_ans = String::new();
    stdin().read_line(&mut have_hour_ans).expect("DUDE");
    println!();

    if !have_hour_ans.eq_ignore_ascii_case("n") && !have_hour_ans.eq_ignore_ascii_case("no") {
        let mut type_of_date = String::new();
        println!("Would you like the date displayed in american (MM-DD-YYYY) or european (DD-MM-YYYY) format? (a/E)");
        stdin().read_line(&mut type_of_date).expect("DUDE");
        if !type_of_date.eq_ignore_ascii_case("a") && !type_of_date.eq_ignore_ascii_case("american") {
            config_file_buffer.push_str("hour a\n");
        } else {
            config_file_buffer.push_str("hour e\n");
        }
    }
}

pub async fn run(config_dir: &mut PathBuf, set: String, system: System) {
    todo!();
}
