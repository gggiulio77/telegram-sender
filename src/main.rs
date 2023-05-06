mod config;
mod db;
mod handler;

use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use teloxide::Bot;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port: u16 = match std::env::var("PORT") {
        Ok(v) => v.parse().unwrap(),
        Err(_) => 8080,
    };

    let bot = Bot::from_env();
    let db = db::init().await.unwrap();

    // Create the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(config::configure(bot.clone(), db.clone()))
            .route("/", web::post().to(handler::handle))
            .route("/", web::get().to(HttpResponse::Ok))
            .route(
                "/health/{_:(readiness|liveness)}",
                web::get().to(HttpResponse::Ok),
            )
    })
    .bind(("127.0.0.1", port))?
    .workers(1)
    .run()
    .await
}
