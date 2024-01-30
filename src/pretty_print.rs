use colored::Colorize;

pub fn print_row(idx: usize, item: &str) {
    println!("  {}. --> {}", idx + 1, item.bright_green());
}
