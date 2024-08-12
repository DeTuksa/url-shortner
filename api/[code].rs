use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;
use url_shortner::UrlShortner;
use vercel_runtime::{
    http::bad_request, run, Body, Error,
    Request, Response, StatusCode,
};

#[derive(Deserialize, Serialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let shortener = UrlShortner::new("db");
    let parsed_url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();
    let code_key = hash_query.get("code");
    
    match code_key {
        None => {
            return bad_request(
                APIError {
                    message: "Query string is invalid",
                    code: "query_string_invalid"
                }
            );
        }
        Some(code) => match shortener.get_url(&code) {
            Some(url) => Ok(
                Response::builder()
            .status(StatusCode::OK)
            .header("Location", url)
            .body(Body::Empty)?
            ),
            None => {
                Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(Body::Text(format!("No matching url found")))?)
            }
        }
    }
}