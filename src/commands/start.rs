use crate::id;

use super::*;

#[derive(Args, Debug)]
#[command(about = "Start tracking the specified ticket", author = "Dom")]
pub struct StartCommand {
    pub ticket_name: String,
}

pub fn command(config: ConfigRef, args: StartCommand) {
    let arg_ticket_name = args.ticket_name.trim().to_owned();

    if let Some(entry) = config
        .current
        .iter()
        .find(|entry| entry.ticket_name == arg_ticket_name)
    {
        println!(
            "{} {}",
            "Already tracking".bright_green(),
            entry.ticket_name.bright_green(),
        );

        return;
    }

    if let Some((idx, entry)) = config
        .entries
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.ticket_name == arg_ticket_name)
    {
        println!(
            "{} {}",
            "You already tracked and ended this ticket".yellow(),
            args.ticket_name.bright_yellow(),
        );

        let result = inquire::Confirm::new(
            format!(
                "You already tracked and finished ticket {}, \n Do you want to resume it?",
                entry.ticket_name
            )
            .as_str(),
        )
        .prompt()
        .unwrap_or_default();

        if !result {
            return;
        }
        
        config.resume_entry(idx);
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
