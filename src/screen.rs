use std::io::Write;

use crate::structs::WidgetDisplay;

pub fn clear() {
    println!("\x1B[2J\x1B[H"); // This clears the screen and puts the cursor at the start
}

pub fn flush() { // flushing the print buffer loads it into the terminal or smth
    std::io::stdout().flush().unwrap();
}

pub async fn display(
    date: &String,
    widget_1: &WidgetDisplay,
    widget_2: &WidgetDisplay,
    widget_3: &WidgetDisplay,
    widget_4: &WidgetDisplay, ) {

    // Provisional display for testing; not entering final release
    clear();
    println!("{date}");
    println!("\n");
    println!("{}", widget_1.current);
    println!("{}", widget_2.current);
    println!("{}", widget_3.current);
    println!("{}", widget_4.current);
}
