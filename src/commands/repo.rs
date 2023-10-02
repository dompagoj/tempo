use super::*;
use std::path::PathBuf;

#[derive(Args, Debug)]
#[command(about = "Manage tracket git repos")]
pub struct RepoCommand {
    #[command(subcommand)]
    pub action: RepoCommandAction,
}

#[derive(Subcommand, Debug)]
pub enum RepoCommandAction {
    Add { path: PathBuf },
    Rm,
    Ls,
}
#[derive(Args, Debug)]
struct AddArgs {
    path: PathBuf,
}

pub fn command(config: Cfg, args: RepoCommandAction) -> anyhow::Result<()> {
    config.repos.initialize();

    match args {
        RepoCommandAction::Add { path } => {
            let full_path = match std::fs::canonicalize(path) {
                Ok(val) => val,
                Err(_) => {
                    println!("{}", "Invalid path".bright_red());
                    return Ok(());
                }
            };
            let found = config.repos.inner().list.iter().find(|path| path == &&full_path);

            if found.is_some() {
                bail_ok!("{} Already added", full_path.to_str().unwrap().green());
            }

            config.repos.inner_mut().list.push(full_path.clone());
            bail_ok!("Added {}", full_path.to_str().unwrap().bright_green());
        }
        RepoCommandAction::Ls => {
            println!("Existing repos: ");
            for (idx, path) in config.repos.inner().list.iter().enumerate() {
                println!("  {}. --> {}", idx + 1, path.to_str().unwrap().bright_green());
            }
        }
        RepoCommandAction::Rm => {
            let list = &mut config.repos.inner_mut().list;

            if list.is_empty() {
                bail_ok!("Not currently tracking any repos...");
            }

            let options = list.iter().map(|path| path.to_str().unwrap()).collect::<Vec<_>>();

            let res = match inquire::MultiSelect::new("Delete repo(s)", options).prompt() {
                Ok(val) => val,
                Err(_) => return Ok(()),
            };

            if res.len() == list.len() {
                list.clear();
                bail_ok!("{}", "Removed all tracked repos".red());
            }

            config.repos.inner_mut().list = list
                .iter()
                .filter(|item| !(*res).contains(&item.to_str().unwrap()))
                .cloned()
                .collect::<Vec<_>>();
        }
    }

    Ok(())
}
