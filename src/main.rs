use url_shortner::UrlShortner;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ShortenReq {
    url: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shortener = web::Data::new(UrlShortner::new());
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
    let url = shortner.shorten_url(&req.url);
    HttpResponse::Ok().json(url)
}

async fn redirect_url(
    shortener: web::Data<UrlShortner>,
    code: web::Path<String>,
) -> impl Responder {
    match shortener.get_url(&code) {
        Some(url) => HttpResponse::Found().append_header(("Location", url)).finish(),
        None => HttpResponse::NotFound().body("Short code not found"),
    }
}