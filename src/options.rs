use crate::{
    screen,
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


pub async fn print_help() {
println!(
"usage: easy_resources COMMAND [COMMAND_OPTIONS]

Make sets to define what you want to display on the terminal, and then run them!

commands:
    help: displays this message
    list: lists all your sets
    make <set>: makes a new set with the provided name
    run <set>: runs a set
    delete <set>: deletes a set
    rename <set>: asks for a new name for the specified set

Additional information:
    Set location: ~/.config/easy_resources/<set>.conf
    Report issues at: https://github.com/realclawy/easy_resources/issues
"
    );
}

pub async fn list(config_dir: PathBuf) {
    let files = config::read(config_dir).await;

    if !files.is_empty() {
        println!("Available sets:");
        let mut iter_files = files.iter();
        print!("{}", iter_files.next().unwrap());
        for file in iter_files {
            print!(", {file}")
        }
    } else {
        print!("You have no sets made. Create one with easy_resources make <set_name>");
    }
    println!()
}

pub async fn delete(mut config_dir: PathBuf, set: String) {
    config_dir.push(set + ".conf");

    if config_dir.exists() {
        if let Err(e) = fs::remove_file(&config_dir) {
            throw_error(format!("Failed to remove file: {}", e).as_str(), 23);
        }
    } else {
        throw_error("That file doesn't exist...", 5);
    }
}

pub async fn rename(config_dir: PathBuf, set: String) {
    let mut config_file = config_dir.clone();
    config_file.push(set + ".conf");

    let mut new_name_str = String::new();
    std::io::stdin().read_line(&mut new_name_str).expect("dude");
    let new_name_str = new_name_str.trim().to_string();
    let mut new_name = config_dir.clone();
    new_name.push(new_name_str + ".conf");

    if config_file.exists() {
        if !new_name.exists() {
            if let Err(e) = fs::rename(&config_file, &new_name) {
                throw_error(format!("Couldn't rename it. This is likely a perms issue: {}", e).as_str(), 22);
            }
        } else {
            throw_error("set that already has that name was found.", 6);
        }
    } else {
        throw_error("That set doesn't exist...", 5);
    }
}

pub async fn make(mut config_dir: PathBuf, set: String) {
    let mut config_file_buffer = String::new();


    //  NOTE: DATE + HOUR
    print!("do you want the hour to be displayed? (Y/n) ");
    let mut have_hour_ans = String::new();
    screen::flush();
    stdin().read_line(&mut have_hour_ans).expect("DUDE");
    let have_hour_ans = have_hour_ans.trim();
    println!();

    if !have_hour_ans.eq_ignore_ascii_case("n") && !have_hour_ans.eq_ignore_ascii_case("no") {
        let mut type_of_date = String::new();
        print!("Would you like the date displayed in american (MM-DD-YYYY) or european (DD-MM-YYYY) format? (a/E) ");
        screen::flush();
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

    screen::clear();

    //  NOTE: PROCESSES
    for i in 1..=4 {
        loop {
            println!("What process would you like to show in slot number {i}?:");
            println!("cu: CPU usage");
            println!("ct: CPU temperature");
            println!("gu: GPU usage");
            println!("mu: memory usage");
            println!("up: uptime");
            println!("bt: battery (only if you actually have a battery)");
            println!("sd: sound level (pipewire only)");
            println!("no: nothing");
            print!("answer with the 2-letter word: ");
            screen::flush();
            let mut process_to_show = String::new();
            stdin().read_line(&mut process_to_show).expect("DUDE");
            let process_to_show = process_to_show.trim().to_ascii_lowercase();

            match process_to_show.as_str() {
                "cu"|"ct"|"mu"|"up"|"bt"|"sd" => {
                    let process_n_color = color::ask().await;
                    config_file_buffer.push_str((process_to_show.to_string() + " " + process_n_color.as_str() + "\n").as_str());
                    screen::clear();
                    break;
                },
                "no" => {
                    config_file_buffer.push_str((process_to_show.to_string() + "\n").as_str());
                    screen::clear();
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
    let mut config_file = match File::create(&config_dir) {
        Ok(file) => file,
        Err(e) => {
            throw_error(format!("couldn't create config file... This is likely a perms error: {}", e).as_str(), 10);
            return;
        }
    };
    if let Err(e) = config_file.write_all(config_file_buffer.as_bytes()) {
        throw_error(format!("couldn't write to config file... This is likely a perms error: {}", e).as_str(), 10);
    }
}

pub async fn run(mut config_dir: PathBuf, set: String) {
    //  NOTE: Open/read file
    let system = System::new();
    let mut date = Date { display: false, american: false, color: "black".to_string() };
    let mut widgets: Vec<Widget> = Vec::new();

    let accessible_files = config::read(config_dir.clone()).await;

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
        throw_error("The date in this set is... Weird... Check it out please", 15);
    }
    if !date_raw[1].eq("no") {
        if date_raw.len() < 3 {
            throw_error("The date in this set is... Weird... Check it out please", 15);
        }
        date.display = true;
        if date_raw[1].eq("a") {
            date.american = true;
        }
        date.color = date_raw[2].to_string();
    }

    // widgets
    for n in 0..=3 { // 4 loops. I'd do 0..4 but I'm scared it won't work lol
        let widget_raw: Vec<&str>;
        // Get a line with actual config in case people add spaces for readability or smth
        loop {
            let buffer_line = buffer.next().unwrap();
            if !buffer_line.is_empty() {
                widget_raw = buffer_line.split(" ").collect();
                break;
            }
        }

        // get the widget
        if widget_raw[0].eq("no") {
            widgets.push(Widget {widget_type: "no".to_string(), color: "black".to_string()}); // black is a fallback so it doesn't crash for whatever reason
        } else if widget_raw.len() < 2 {
            let error_message = format!("Something's weird with widget number {}...", n + 1);
            throw_error(error_message.as_str(), 15);
        } else {
            widgets.push(Widget {widget_type: widget_raw[0].to_string(), color: widget_raw[1].to_string()});
        }
    }


    // Pre-display
    let (mut time_display, mut widget_1, mut widget_2, mut widget_3, mut widget_4) = tokio::join!(
        fetch::time(date.display, date.american, &date.color),
        fetch::get(&system, &widgets[0], None),
        fetch::get(&system, &widgets[1], None),
        fetch::get(&system, &widgets[2], None),
        fetch::get(&system, &widgets[3], None)
    );

    // Initialize terminal
    let screen_init = screen::init();
    if let Err(e) = screen_init {
        throw_error(format!("Failed to initialize terminal: {}", e).as_str(), 20); unreachable!()
    }
    let mut terminal = screen_init.unwrap();

    // Main loop
    loop {
        // Render the screen
        if let Err(e) = screen::render(
            &mut terminal,
            &time_display,
            &date.color,
            &widget_1,
            &widget_2,
            &widget_3,
            &widget_4,
        ) {
            throw_error(format!("Failed to render screen: {}", e).as_str(), 21);
        }

        // Check for quit key
        if screen::check_quit(std::time::Duration::from_millis(1000)) {
            break;
        }

        // update date and widgets
        (time_display, widget_1, widget_2, widget_3, widget_4) = tokio::join!(
            fetch::time(date.display, date.american, &date.color),
            fetch::get(&system, &widgets[0], widget_1.next),
            fetch::get(&system, &widgets[1], widget_2.next),
            fetch::get(&system, &widgets[2], widget_3.next),
            fetch::get(&system, &widgets[3], widget_4.next)
        );
    }

    // Cleanup terminal
    if let Err(e) = screen::cleanup() {
        throw_error(format!("Failed to cleanup terminal: {}", e).as_str(), 22);
    }
}
