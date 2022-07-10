use super::CommandFunc;
use crate::error::Error;
use serenity::{
    async_trait,
    client::Context,
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
};

/// Mute user with duration in second
pub struct Mute;

const DEFAULT_DURATION: i64 = 300;

#[async_trait]
impl CommandFunc for Mute {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), Error> {
        let mut options = command.data.options.iter();
        let user = options
            .find(|e| e.name == "user")
            .ok_or(Error::NoFoundParameter("name"))?;
        let duration = match options.find(|o| o.name == "duration") {
            Some(d) => d
                .value
                .clone()
                .unwrap_or_default()
                .as_i64()
                .unwrap_or(DEFAULT_DURATION),
            None => DEFAULT_DURATION,
        };

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content(format!("{user:?} mute for {duration}"))
                    })
            })
            .await?;

        Ok(())
    }
}
