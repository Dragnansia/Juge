use super::CommandFunc;
use crate::error::Error;
use serenity::{
    async_trait,
    client::Context,
    model::{
        id::RoleId,
        prelude::interaction::{
            application_command::{ApplicationCommandInteraction, ResolvedTarget},
            InteractionResponseType,
        },
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

        // Check if clone member is good with data saved
        if let Some(member) = command.member.clone() {
            // Verif if user is not already mute and if time is add to the current mute duration
            // or reset to the new time if it's higher current remind time

            let target = command.data.target().unwrap();
            match target {
                ResolvedTarget::User(user, _) => {
                    println!("User name: {}", user.name);
                }
                _ => {}
            }

            // member.add_role(&ctx.http, RoleId(0)).await?;
            println!("Member name: {}", member.display_name());

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
        } else {
            Err(Error::NoFoundUser)
        }
    }
}
