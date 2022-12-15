use super::*;

pub fn command(config: ConfigRef) {
    let user_data = config.user_data.initialize();
    let tempo_key = user_data.get_tempo_api_key();
    
    println!("{} : {}", "Tempo api key".green(), tempo_key.red());
}
