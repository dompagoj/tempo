// use std::fmt::Display;
//
// use super::*;
//
// #[derive(Args, Debug)]
// #[command(about = "Remove some part of local storage")]
// pub struct RmCommand {
//     #[command(subcommand)]
//     action: Option<RmCommandAction>,
// }
//
// #[derive(Subcommand, Debug)]
// pub enum RmCommandAction {
//     Current,
//     Archived,
//     #[arg()]
//     All(AllArgs),
//     #[clap(skip)]
//     Unknown,
// }
//
// #[derive(Args, Debug)]
// pub struct AllArgs {
//     #[arg(short, long)]
//     force: bool,
//
//     #[arg(long)]
//     delete_file: bool,
// }
//
// impl Display for RmCommandAction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if let RmCommandAction::All(_) = self {
//             write!(f, "All")
//         } else {
//             write!(f, "{:?}", self)
//         }
//     }
// }
