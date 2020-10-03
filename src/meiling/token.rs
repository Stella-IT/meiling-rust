use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn generate_random_token() -> String {
    const TOKEN_LEN: usize = 128;

    let token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(TOKEN_LEN)
        .collect();
    assert_eq!(token.len(), TOKEN_LEN);

    token
}
