use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use url_shortner::UrlShortner;
use vercel_runtime::{
    http::bad_request, process_request, process_response, run_service, service_fn, Body, Error,
    Request, RequestPayloadExt, Response, ServiceBuilder, StatusCode,
};

#[derive(Deserialize, Serialize)]
struct ShortenReq {
    url: String,
}

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = ServiceBuilder::new()
        .map_request(process_request)
        .map_response(process_response)
        .service(service_fn(handler));

    run_service(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let db_name = env::var("DATABASE").expect("DATABASE must be set");
    let collection = env::var("COLLECTION").expect("COLLECTION must be set");
    let shortener = UrlShortner::new(&db_name, &collection).await;
    let payload = req.payload::<ShortenReq>();
    match payload {
        Err(..) => bad_request(APIError {
            message: "Invalid payload",
            code: "invalid_payload",
        }),
        Ok(None) => bad_request(APIError {
            message: "No payload",
            code: "no_payload",
        }),
        Ok(Some(body)) => {
            let code = shortener.shorten_url(&body.url).await;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(
                    json!({
                        "message": format!("Url shortened"),
                        "code": code
                    })
                    .to_string()
                    .into(),
                )?)
        }
    }
}
