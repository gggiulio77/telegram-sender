use actix_web::web;
use surrealdb::{engine::remote::ws::Client, Surreal};
use teloxide::Bot;

// TODO: review this way of managing the App state
pub fn configure(bot: Bot, db: Surreal<Client>) -> web::Data<HandlerConfig> {
    log::info!("Configuring service");
    web::Data::new(HandlerConfig {
        bot: Some(bot),
        db: Some(db),
    })
}

pub struct HandlerConfig {
    pub bot: Option<Bot>,
    pub db: Option<Surreal<Client>>,
}

impl Default for HandlerConfig {
    fn default() -> HandlerConfig {
        HandlerConfig {
            bot: None,
            db: None,
        }
    }
}

// TODO: replace Surreal<Client> with a generic trait
