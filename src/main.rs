mod config;
mod commands;
mod id;

use commands::Tempo;
use config::ConfigFile;

fn main() {
    let args = Tempo::parse_wrap();
    let mut config = ConfigFile::get_new();
    
    args.run(&mut config);

    config.save();
}
