use url_shortner::UrlShortner;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

#[derive(Deserialize, Serialize)]
struct ShortenReq {
    url: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_name = env::var("DATABASE").expect("DATABASE must be set");
    let collection = env::var("COLLECTION").expect("COLLECTION must be set");
    let shortener = web::Data::new(UrlShortner::new(&db_name, &collection).await);
    HttpServer::new(move || {
        App::new()
        .app_data(shortener.clone())
        .route("/shorten", web::post().to(shorten_url))
        .route("/{code}", web::get().to(redirect_url))
    }).bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn shorten_url(
    shortner: web::Data<UrlShortner>,
    req: web::Json<ShortenReq>
) -> impl Responder {
    let url = shortner.shorten_url(&req.url).await;
    HttpResponse::Ok().json(url)
}

async fn redirect_url(
    shortener: web::Data<UrlShortner>,
    code: web::Path<String>,
) -> impl Responder {
    match shortener.get_url(&code).await {
        Some(url) => HttpResponse::Found().append_header(("Location", url)).finish(),
        None => HttpResponse::NotFound().body("Short code not found"),
    }
}