use serenity::{
    async_trait, client::Context,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
};

use crate::error::Error;

pub mod mute;

#[async_trait]
pub trait CommandFunc {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), Error>;
}
