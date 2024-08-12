use crate::shortner::url::generate_short_code;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct UrlMapping {
    short_code: String,
    url: String,
}
pub struct UrlShortner {
    collection: Collection<UrlMapping>,
}

impl UrlShortner {
    pub async fn new(db_name: &str, collection_name: &str) -> Self {
        let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
        let client = Client::with_uri_str(&mongo_uri).await.expect("Failed to initialize MongoDB client");
        let database = client.database(db_name);
        let collection = database.collection::<UrlMapping>(collection_name);

        Self {
            collection
        }
    }

    pub async fn shorten_url(&self, url: &str) -> String {
        let short_code = generate_short_code();
        let new_mapping = UrlMapping {
            short_code: short_code.clone(),
            url: url.to_string(),
        };
        self.collection.insert_one(new_mapping).await.expect("Failed to store URL");
        short_code
    }

    pub async fn get_url(&self, short_code: &str) -> Option<String> {
        match self.collection.find_one(doc! {"short_code": short_code}).await {
            Ok(Some(url_mapping)) => Some(url_mapping.url),
            Ok(None) => None,
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use std::env;
    use dotenv::dotenv;

    async fn setup_test_db() -> UrlShortner {
        dotenv().ok();
        let db_name = env::var("DATABASE").expect("DATABASE must be set");
        let collection = env::var("COLLECTION").expect("COLLECTION must be set");
        UrlShortner::new(&db_name, &collection).await
    }

    #[tokio::test]
    async fn test_shorten_url() {
        let shortner = setup_test_db().await;
        let original_url = "https://test.com/testing";

        let short_code = shortner.shorten_url(&original_url).await;
        assert_eq!(short_code.len(), 6, "Short code should be 6 characters long");

        let retrieved_url = shortner.get_url(&short_code).await.expect("URL should return");
        assert_eq!(retrieved_url, original_url, "Retrieved URL should match the original URL");
    }

    #[tokio::test]
    async fn test_invalid_code() {
        let shortner = setup_test_db().await;

        let invalid_code = "invalid-code";
        let retrieved_url = shortner.get_url(invalid_code).await;
        assert!(retrieved_url.is_none(), "Invalid code should return None");
    }
}