use super::*;

pub fn command(config: ConfigRef) {
    println!("--------");
    println!("Status");
    println!("--------");

    println!(
        "{}",
        format!(
            "Currently tracking {} tickets",
            config.current.len().to_string().red()
        )
        .green()
    );

    println!(
        "{}",
        format!(
            "{} Tickets in archive",
            config.entries.len().to_string().red()
        )
        .green()
    );

    println!("--------");
}
