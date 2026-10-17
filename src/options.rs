use crate::{
    clear, // Function for clearing the screen.  TODO: make a screen.rs lib file
    color, // colored print
    config, // get things about the config file.  NOTE: mayybe reduce "run" size by putting slices here
    error_handling::throw_error,
    fetch, // get arguments given
    structs::*
};
use std::{io::{Write, stdin}, path};
use path::PathBuf;
use systemstat::{Platform, System};
use std::fs;
use fs::File;

pub async fn delete(mut config_dir: PathBuf, set: String) {
    config_dir.push(set + ".conf");

    if config_dir.exists() {
        fs::remove_file(config_dir).unwrap(); //  TODO: error handle it (I don't feel like doing so rn)
    } else {
        throw_error("That file doesn't exist...", 5); unreachable!()
    }
}

pub async fn rename(config_dir: PathBuf, set: String) {
    let mut config_file = config_dir.clone();
    config_file.push(set + ".conf");

    let mut new_name = String::new();
    std::io::stdin().read_line(&mut new_name).expect("dude");
    let new_name = path::PathBuf::from(format!("{:?}.conf", config_dir));

    if config_file.exists() {
        if !new_name.exists() {
            fs::rename(config_file, new_name).unwrap_or_else(|_| {throw_error("Couldn't rename it. This is likely a perms issue...", 22);});
        } else {
            throw_error("set that already has that name was found.", 6); unreachable!()
        }
    } else {
        throw_error("That set doesn't exist...", 5); unreachable!()
    }
}

pub async fn make(mut config_dir: PathBuf, set: String) {
    let mut config_file_buffer = String::new();


    //  NOTE: DATE + HOUR
    print!("do you want the hour to be displayed? (Y/n) ");
    let mut have_hour_ans = String::new();
    std::io::stdout().flush().unwrap();
    stdin().read_line(&mut have_hour_ans).expect("DUDE");
    let have_hour_ans = have_hour_ans.trim();
    println!();

    if !have_hour_ans.eq_ignore_ascii_case("n") && !have_hour_ans.eq_ignore_ascii_case("no") {
        let mut type_of_date = String::new();
        print!("Would you like the date displayed in american (MM-DD-YYYY) or european (DD-MM-YYYY) format? (a/E) ");
        std::io::stdout().flush().unwrap(); // If you don't flush it will not work ig?
        stdin().read_line(&mut type_of_date).expect("DUDE");
        let type_of_date = type_of_date.trim();
        println!();
        if !type_of_date.eq_ignore_ascii_case("a") && !type_of_date.eq_ignore_ascii_case("american") {
            let date_color = color::ask().await;
            config_file_buffer.push_str(("hour e ".to_string() + date_color.as_str() + "\n").as_str()); // hour a <color>
        } else {
            let date_color = color::ask().await;
            config_file_buffer.push_str(("hour a ".to_string() + date_color.as_str() + "\n").as_str()); // hour e <color>
        }
    } else {
        config_file_buffer.push_str("hour no\n");
    }
    let _ = have_hour_ans;

    clear();

    //  NOTE: PROCESSES
    for i in 1..=4 {
        loop {
            println!("What process would you like to show in slot number {i}?:");
            println!("cu: CPU usage");
            println!("ct: CPU temperature");
            println!("gu: GPU usage (W.I.P)");
            println!("mu: memory usage");
            println!("up: uptime");
            println!("bt: battery (only if you actually have a battery)");
            println!("sd: sound level");
            println!("no: nothing");
            print!("answer with the 2-letter word: ");
            std::io::stdout().flush().unwrap();
            let mut process_to_show = String::new();
            stdin().read_line(&mut process_to_show).expect("DUDE");
            let process_to_show = process_to_show.trim().to_ascii_lowercase();

            match process_to_show.as_str() {
                "cu"|"ct"|"mu"|"up"|"bt"|"sd" => {
                    let process_n_color = color::ask().await;
                    config_file_buffer.push_str((process_to_show.to_string() + " " + process_n_color.as_str() + "\n").as_str());
                    clear();
                    break;
                },
                "no" => {
                    config_file_buffer.push_str((process_to_show.to_string() + "\n").as_str());
                    clear();
                    break;
                },
                _ => {
                    //clear();
                    println!("We couldn't get the process you meant... Type again please\n");
                },
            };
        }
    }

    //  NOTE: create file
    config_dir.push(set + ".conf");
    let mut config_file = File::create(config_dir).unwrap_or_else(|_| {
        throw_error("couldn't create config file... This is likely a perms error", 10); unreachable!()
    });
    config_file.write_all(config_file_buffer.as_bytes()).unwrap_or_else(|_| {
        throw_error("couldn't create config file... This is likely a perms error", 10); unreachable!()
    });
}

pub async fn run(mut config_dir: PathBuf, set: String) {
    //  NOTE: Open/read file
    let system = System::new();
    let mut date = Date { display: false, american: false, color: "black".to_string() };
    let mut widgets: Vec<Widget> = Vec::new();

    let accessible_files = config::read(&config_dir).await;
    dbg!("{&accessible_files}");

    let mut set_found = false;
    for file in accessible_files {
        if file.eq(&set) {
            set_found = true;
            break;
        }
    } // this for loop already drops accessible_files

    if !set_found {
        throw_error("you don't have that set. Make sure you spelt it correctly (both here and when you made it)", 12);
    }
    let _ = set_found; // weird way to say drop (I think?)

    config_dir.push(set + ".conf");

    let mut buffer = String::new();
    match fs::read(config_dir) {
        Ok(read_buffer) => buffer = String::from_utf8_lossy(&read_buffer).to_string(),
        Err(_) => throw_error("Could not read file, likely due to a lack of permissions", 13)
    };

    // Check for enough lines
    if buffer.lines().count() < 5 {
        throw_error("not enough arguments in the config file.", 14);
    }

    let mut buffer = buffer.lines(); // Get the lines; there should be 5: date display and 4 widgets

    // date display
    let date_raw: Vec<&str> = buffer.next().unwrap()
        .split(" ").collect();

    if date_raw.len() < 2 {
        throw_error("The date in this set is... Weird... Check it out please", 15); unreachable!()
    }
    if !date_raw[1].eq("no") {
        if date_raw.len() < 3 {
            throw_error("The date in this set is... Weird... Check it out please", 15); unreachable!()
        }
        date.display = true;
        if date_raw[1].eq("a") {
            date.american = true;
        }
        date.color = date_raw[2].to_string();
    }

    // widgets
    for n in 0..=3 { // 4 loops. I'd do 0..4 but I'm scared it won't work lol
        // Get a line with actual config in case people add spaces for readability or smth
        let mut widget_raw: Vec<&str> = Vec::new();
        loop {
            let buffer_line = buffer.next().unwrap();
            if !widget_raw.is_empty() {
                widget_raw = buffer_line.split(" ").collect();
                break;
            }
        }

        // get the widget
        if widget_raw[0].eq("no") {
            widgets.push(Widget {widget_type: "no".to_string(), color: "black".to_string()}); // black is a fallback so it doesn't crash for whatever reason
        }
        if widget_raw.len() < 2 {
            let error_message = format!("Something's weird with widget number {n}...");
            throw_error(error_message.as_str(), 15);
        }
        widgets.push(Widget {widget_type: widget_raw[0].to_string(), color: widget_raw[1].to_string()});
    }


    // Pre-display
    let (time_display, widget_1, widget_2, widget_3, widget_4) = tokio::join!(
        fetch::time(date.display, date.american),
        fetch::get(&system, &widgets[0], None),
        fetch::get(&system, &widgets[1], None),
        fetch::get(&system, &widgets[2], None),
        fetch::get(&system, &widgets[3], None)
    );
    //  TODO: display
    todo!()
}
