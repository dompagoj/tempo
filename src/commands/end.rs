use super::*;

#[derive(Args, Debug)]
#[command(about = "Stop tracking the current ticket")]
pub struct EndCommand {
    ticket_name: Option<String>,

    #[arg(short = 'f', long)]
    force: bool,
}

pub fn command(config: ConfigRef, args: EndCommand) {
    if config.current.len() == 0 {
        println!("{}", "Not tracking any tickets".bright_red());
        return;
    }

    if config.current.len() == 1 {
        let first = config.current.first().unwrap();
        print_end_msg(first);
        config.end_active_entry(0);

        return;
    }

    if args.force {
        config.end_active_entries(Some(print_end_msg));

        return;
    }

    if let Some(name) = args.ticket_name {
        let found = config
            .current
            .iter()
            .position(|e| e.ticket_name == name.trim());

        match found {
            Some(idx) => {
                print_end_msg(config.current.get(idx).unwrap());
                config.end_active_entry(idx);
            }
            None => {
                println!("{} {}", "No tickets with name".red(), name.bright_red());
            }
        }

        return;
    }

    let result = inquire::MultiSelect::new(
        "Which to delete",
        config
            .current
            .iter()
            .map(|entry| entry.ticket_name.clone())
            .collect(),
    )
    .prompt()
    .unwrap_or_default();

    for entry_to_delete in result {
        println!(
            "{}",
            format!("Ended ticket {}", entry_to_delete.bright_green()).green()
        );
        let idx = config
            .current
            .iter()
            .position(|entry| entry.ticket_name == entry_to_delete)
            .unwrap();

        config.end_active_entry(idx);
    }
}

fn print_end_msg(entry: &TimeEntry) {
    println!("{} {}", "Ended ticket".green(), entry.ticket_name.bright_green());
}
