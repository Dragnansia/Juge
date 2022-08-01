use crate::error::Error;
use serenity::{
    async_trait, client::Context,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
};

pub mod mute;

#[async_trait]
pub trait CommandFunc {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), Error>;
}
