use crate::{config::HandlerConfig, db::User};
use actix_web::{
    error,
    web::{self, Json},
    Error, HttpResponse, Result,
};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use teloxide::requests::Requester;

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub email: String,
    pub message: String,
}

pub async fn handle(
    body: Json<Message>,
    config: web::Data<HandlerConfig>,
) -> Result<HttpResponse, Error> {
    info!("body: {:?}", body);

    let Message { email, message } = body.into_inner();

    let bot = config.bot.as_ref().unwrap();
    let db = config.db.as_ref().unwrap();

    let User { chat_id, .. } = match db.select(("user", email)).await {
        Ok(user) => user,
        Err(error) => {
            return Err(error::ErrorBadRequest(
                json!({ "error": error.to_string() }),
            ));
        }
    };

    bot.send_message(chat_id, message).await.unwrap();

    Ok(HttpResponse::Ok().json(json!({ "action": "message sent" })))
}

// TODO: replace db with a generic repository
// TODO: create Custom Errors
