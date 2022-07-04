use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        application::interaction::Interaction, gateway::Ready, guild::UnavailableGuild,
        id::GuildId, prelude::command::Command,
    },
    Error,
};

/// Discord bot
pub struct Juge;

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

    async fn interaction_create(&self, _: Context, _: Interaction) {}
}

impl Juge {
    pub async fn guild_command_initialisation(
        guild: &UnavailableGuild,
        ctx: &Context,
    ) -> Result<Vec<Command>, Error> {
        GuildId::set_application_commands(&guild.id, &ctx.http, |commands| commands).await
    }
}
