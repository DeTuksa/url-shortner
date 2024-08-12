use sled::Db;
use crate::shortner::url::generate_short_code;

pub struct UrlShortner {
    db: Db
}

impl UrlShortner {
    pub fn new(db_path: &str) -> Self {
        Self {
            db: sled::open(db_path).expect("Failed to open database")
        }
    }

    pub fn shorten_url(&self, url: &str) -> String {
        let short_code = generate_short_code();
        self.db.insert(&short_code, url.as_bytes()).expect("Failed to store URL");
        short_code
    }

    pub fn get_url(&self, short_code: &str) -> Option<String> {
        self.db.get(short_code)
        .expect("Failed to fetch URL")
        .map(|url_bytes| String::from_utf8(url_bytes.to_vec()).expect("Invalid UTF-8"))
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    fn setup_test_db() -> UrlShortner {
        let temp_dir = TempDir::new().expect("Failed to create a temporary directory");
        let temp_db_path = temp_dir.path().to_str().unwrap();
        UrlShortner::new(temp_db_path)
    }

    #[test]
    fn test_shorten_url() {
        let shortner = setup_test_db();
        let original_url = "https://test.com/testing";

        let short_code = shortner.shorten_url(&original_url);
        assert_eq!(short_code.len(), 6, "Short code should be 6 characters long");

        let retrieved_url = shortner.get_url(&short_code).expect("URL should return");
        assert_eq!(retrieved_url, original_url, "Retrieved URL should match the original URL");
    }

    #[test]
    fn test_invalid_code() {
        let shortner = setup_test_db();

        let invalid_code = "invalid-code";
        let retrieved_url = shortner.get_url(invalid_code);
        assert!(retrieved_url.is_none(), "Invalid code should return None");
    }
}