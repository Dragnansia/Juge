use crate::{error::Error, juge::Juge};
use serenity::{
    async_trait, client::Context,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
};

pub mod mute;

#[async_trait]
pub trait CommandFunc {
    async fn run(
        juge: &Juge,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Error>;
}
