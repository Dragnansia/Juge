use crate::{
    commands::{mute::Mute, CommandFunc},
    db::DataBase,
    error,
};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        application::interaction::Interaction,
        gateway::Ready,
        guild::UnavailableGuild,
        id::GuildId,
        prelude::{
            command::{Command, CommandOptionType},
            interaction::InteractionResponseType,
        },
    },
    Error,
};

/// Discord bot
pub struct Juge {
    pub db: DataBase,
}

impl Juge {
    pub fn new() -> Self {
        Self {
            db: DataBase::new(),
        }
    }
}

#[async_trait]
impl EventHandler for Juge {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("[Ready] {}", ready.user.name);

        for g in ready.guilds {
            let res = Self::guild_command_initialisation(&g, &ctx).await;
            if let Err(err) = res {
                println!("[Error - Ready] {}", err);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let res = match command.data.name.as_str() {
                "mute" => Mute::run(&ctx, &command).await,
                _ => Err(error::Error::NoFoundCommand),
            };

            if let Err(err) = res {
                let error_message = match err {
                    error::Error::NoFoundParameter(param) => format!("No found parameter {param}"),
                    error::Error::NoFoundCommand => "No Found command".into(),
                    _ => "Unknown error".into(),
                };

                let _ = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(error_message))
                    })
                    .await;
            }
        }
    }
}

impl Juge {
    pub async fn guild_command_initialisation(
        guild: &UnavailableGuild,
        ctx: &Context,
    ) -> Result<Vec<Command>, Error> {
        GuildId::set_application_commands(&guild.id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("mute")
                    .description("Mute specifique user for specifique duration")
                    .create_option(|option| {
                        option
                            .name("name")
                            .description("User name")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_option(|option| {
                        option
                            .name("duration")
                            .description("duration in seconds")
                            .kind(CommandOptionType::Integer)
                            .required(false)
                    })
            })
        })
        .await
    }
}
