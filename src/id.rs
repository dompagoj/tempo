use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect()
}
