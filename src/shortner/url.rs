use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn generate_short_code() -> String {
    thread_rng().sample_iter(&Alphanumeric)
    .take(6)
    .map(char::from)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_short_code_length() {
        let short_code = generate_short_code();
        assert_eq!(short_code.len(), 6, "Code generated should be 6 characters long");
    }

    #[test]
    fn test_generate_short_code_alphanumeric() {
        let short_code = generate_short_code();
        assert!(short_code.chars().all(|c| c.is_ascii_alphanumeric()), "Short code should be alphanumeric");
    }
}