use std::fmt::Display;

use super::*;

#[derive(Args, Debug)]
#[command(about = "Remove some part of local storage")]
pub struct RmCommand {
    #[command(subcommand)]
    action: Option<RmCommandAction>,
}

#[derive(Subcommand, Debug)]
pub enum RmCommandAction {
    Current,
    Archived,
    #[arg()]
    All(AllArgs),
    #[clap(skip)]
    Unknown,
}

#[derive(Args, Debug)]
pub struct AllArgs {
    #[arg(short, long)]
    force: bool,
}

impl Display for RmCommandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let RmCommandAction::All(_) = self {
            write!(f, "All")
        } else {
            write!(f, "{:?}", self)
        }
    }
}

pub fn command(config: ConfigRef, args: RmCommand) {
    let mut action = args.action.unwrap_or(RmCommandAction::Unknown);

    if let RmCommandAction::Unknown = action {
        use RmCommandAction::*;
        let options = vec![Current, Archived, All(AllArgs { force: false })];
        action = inquire::Select::new("What to delete", options)
            .prompt()
            .unwrap_or(RmCommandAction::Unknown);
    }

    match action {
        RmCommandAction::Current => {
            print_deleting_message(config.current.iter(), config.current.len());
            config.current.clear();
        }
        RmCommandAction::Archived => {
            print_deleting_message(config.entries.iter(), config.entries.len());
            config.entries.clear();
        }
        RmCommandAction::All(args) => {
            let result = if !args.force {
                inquire::Confirm::new("Delete all data?").prompt().ok()
            } else {
                Some(true)
            };

            if let Some(true) = result {
                println!("{}", "Deleted all data!".bright_red());
                if let Err(err) = config.delete_file() {
                    eprintln!("Failed to delete file {:?}", err);
                }
            } else {
                println!("Aborting...");
            }
        }
        RmCommandAction::Unknown => {}
    }
}

fn print_deleting_message<'a, T>(iter: T, len: usize)
where
    T: Iterator<Item = &'a TimeEntry>,
{
    if len == 0 {
        println!("Nothing to delete!");
        return;
    }

    for entry in iter {
        println!(
            "{} {}",
            "Removing ticket".green(),
            entry.ticket_name.bright_green()
        );
    }
}
