mod data;
mod commands;
mod id;

use commands::Tempo;

fn main() {
    let args = Tempo::parse_wrap();
    let mut config = data::ConfigFile::get_new();
    
    args.run(&mut config);

    config.save();
}
