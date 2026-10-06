use crate::{clear, color, error_handling::throw_error};
use std::{io::{Write, stdin}, path};
use path::PathBuf;
use std::fs;
use fs::File;

pub async fn delete(config_dir: &mut PathBuf, set: String) {
    config_dir.push(set + ".conf");

    if config_dir.exists() {
        fs::remove_file(config_dir).unwrap(); //  TODO: error handle it (I don't feel like doing so rn)
    } else {
        throw_error("That file doesn't exist...", 5); unreachable!()
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
            throw_error("set that already has that name was found.", 6);
        }
    } else {
        throw_error("That file doesn't exist...", 5); unreachable!()
    }
}

pub async fn make(mut config_dir: PathBuf, set: String) {
    let mut config_file_buffer = String::new();


    //  NOTE: DATE + HOUR
    print!("do you want the hour to be displayed? (Y/n) ");
    let mut have_hour_ans = String::new();
    stdin().read_line(&mut have_hour_ans).expect("DUDE");
    println!();

    if !have_hour_ans.eq_ignore_ascii_case("n") && !have_hour_ans.eq_ignore_ascii_case("no") {
        let mut type_of_date = String::new();
        print!("Would you like the date displayed in american (MM-DD-YYYY) or european (DD-MM-YYYY) format? (a/E)");
        stdin().read_line(&mut type_of_date).expect("DUDE");
        println!();
        if !type_of_date.eq_ignore_ascii_case("a") && !type_of_date.eq_ignore_ascii_case("american") {
            let date_color = color::ask().await;
            config_file_buffer.push_str(("hour a ".to_string() + date_color.as_str() + "\n").as_str()); // hour a <color>
        } else {
            let date_color = color::ask().await;
            config_file_buffer.push_str(("hour e ".to_string() + date_color.as_str() + "\n").as_str()); // hour e <color>
        }
    }
    drop(have_hour_ans);

    clear();

    //  NOTE: PROCESSES
    for i in 1..=4 {
        loop {
            let mut process_to_show = String::new();
            println!("What process would you like to show in slot number {i}?:");
            println!("cu: CPU usage");
            println!("ct: CPU temperature");
            println!("mu: memory usage");
            println!("up: uptime");
            println!("bt: battery (only if you actually have a battery)");
            println!("sd: sound level");
            println!("no: nothing");
            print!("answer with the 2-letter word: ");
            stdin().read_line(&mut process_to_show).expect("DUDE");
            let process_to_show = process_to_show.to_ascii_lowercase();

            match process_to_show.as_str() {
                "cu"|"ct"|"mu"|"up"|"bt"|"sd" => {
                    let process_n_color = color::ask().await;
                    config_file_buffer.push_str((process_to_show.to_string() + process_n_color.as_str() + "\n").as_str());
                    clear();
                    break;
                },
                "no" => {
                    config_file_buffer.push_str((process_to_show.to_string() + "\n").as_str());
                    clear();
                    break;
                },
                _ => {
                    clear();
                    println!("We couldn't get the process you meant... Type again please\n");
                },
            };
        }
    }
    config_dir.push(set + ".conf");
    let mut err = false;
    let mut config_file = File::create(config_dir).unwrap_or_else(|_| {
        throw_error("couldn't create config file... This is likely a perms error", 10); unreachable!()
    });
    config_file.write_all(config_file_buffer.as_bytes()).unwrap_or_else(|_| {
        throw_error("couldn't create config file... This is likely a perms error", 10); unreachable!()
    });
}
/*
pub async fn run(config_dir: &mut PathBuf, set: String, system: System) {
    todo!();
} */
