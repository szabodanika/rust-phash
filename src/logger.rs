use ansi_term::Colour::Red;

pub fn print_error(message: &str) {
    println!(
        "[ERROR] {}",
        Red.paint(format!("Error:\t{}", message).to_string())
    );
}

pub fn print_debug(message: &str) {
    println!("[DEBUG] {}", message);
}

pub fn print_info(message: &str) {
    println!("{}", message);
}
