use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

pub fn print_row(idx: usize, item: &str) {
    println!("  {}. --> {}", idx + 1, item.bright_green());
}

pub fn get_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {bar} {pos}/{len}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    pb.set_style(spinner_style);
    pb.set_position(0);

    return pb;
}
