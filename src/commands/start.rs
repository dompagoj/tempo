use crate::id;

use super::*;

#[derive(Args, Debug)]
#[command(about = "Start tracking the specified ticket", author = "Dom")]
pub struct StartCommand {
    pub ticket_name: String,
}

pub fn command(config: ConfigRef, args: StartCommand) {
    let arg_ticket_name = args.ticket_name.trim().to_owned();

    if let Some(_) = config
        .current
        .iter()
        .find(|entry| entry.ticket_name == arg_ticket_name)
    {
        println!(
            "{} {}",
            "Already tracking".bright_green(),
            args.ticket_name.bright_green(),
        );

        return;
    }

    let new = TimeEntry {
        id: id::generate_id(),
        start_time: chrono::Local::now().to_string(),
        ticket_name: arg_ticket_name,
        ..Default::default()
    };
    config.current.push(new);

    println!(
        "{} {}",
        "Started tracking".green(),
        args.ticket_name.bright_green()
    );

    config.save();
}
