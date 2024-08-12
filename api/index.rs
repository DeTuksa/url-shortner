use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use url_shortener::shortener::UrlShortener;
use actix_web::dev::Server;
use serde::{Deserialize, Serialize};
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Deserialize, Serialize)]
struct ShortenRequest {
    url: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shortener = web::Data::new(UrlShortener::new("url_shortener_db"));

    HttpServer::new(move || {
        App::new()
            .app_data(shortener.clone())
            .route("/shorten", web::post().to(shorten_url))
            .route("/{code}", web::get().to(redirect_url))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

async fn shorten_url(
    shortener: web::Data<UrlShortener>,
    req: web::Json<ShortenRequest>,
) -> impl Responder {
    let short_code = shortener.shorten_url(&req.url);
    HttpResponse::Ok().json(short_code)
}

async fn redirect_url(
    shortener: web::Data<UrlShortener>,
    web::Path(code): web::Path<String>,
) -> impl Responder {
    match shortener.get_url(&code) {
        Some(url) => HttpResponse::Found().append_header(("Location", url)).finish(),
        None => HttpResponse::NotFound().body("Short code not found"),
    }
}
