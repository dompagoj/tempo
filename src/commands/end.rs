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
        println!(
            "{}",
            format!("Ended ticket {}", first.ticket_name.bright_green()).green()
        );
        config.end_active_entry(0);

        return;
    }

    if args.force {
        dbg!(&config.current);
        dbg!(config.current.len());
        for i in 0..config.current.len() {
            println!(
                "{}",
                format!(
                    "Ending ticket {}",
                    config.current.get(0).unwrap().ticket_name.bright_green()
                )
                .green()
            );
            config.end_active_entry(0);
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

    if let Some(name) = args.ticket_name {
        let found = config
            .current
            .iter()
            .position(|e| e.ticket_name == name.trim());

        match found {
            Some(idx) => {
                config.end_active_entry(idx);
                println!("{} {}", "Ended ticket".green(), name.bright_green());
            }
            None => {
                println!("{} {}", "No tickets with name".red(), name.bright_red());
            }
        }

        return;
    }
}
