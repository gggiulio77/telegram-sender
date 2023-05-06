use anyhow::Result;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};
use teloxide::types::ChatId;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub chat_id: ChatId,
    pub telegram_first_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn init() -> Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>(std::env::var("DB_URL")?).await?;

    db.signin(Root {
        username: &std::env::var("DB_USER")?,
        password: &std::env::var("DB_PASSWORD")?,
    })
    .await?;

    db.use_ns("test").use_db("test").await?;

    Ok(db)
}

// TODO: create a repository
