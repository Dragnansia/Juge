#[macro_use]
extern crate dotenv_codegen;

mod commands;
mod db;
mod error;
mod juge;
mod setting;

use db::DataBase;
use dotenv::dotenv;
use error::Error;
use juge::Juge;
use serenity::{prelude::GatewayIntents, Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().unwrap();
    DataBase::init();

    let token = dotenv!("DISCORD_TOKEN");
    let intents = GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Juge::new())
        .await?;
    client.start().await?;

    Ok(())
}
