use crate::{Context, Error, PrefixContext};

/// Evaluates Go code
#[poise::command(discard_spare_arguments, slash_command)]
pub async fn go(ctx: Context<'_>) -> Result<(), Error> {
    poise::say_reply(ctx, "No".into()).await?;
    Ok(())
}

/// Links to the bot GitHub repo
#[poise::command(discard_spare_arguments, slash_command)]
pub async fn source(ctx: Context<'_>) -> Result<(), Error> {
    poise::say_reply(ctx, r"https://github.com/kangalioo/rustbot".into()).await?;
    Ok(())
}

/// Show this menu
#[poise::command(track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let reply = if let Some(command) = command {
        if let Some(command) = ctx
            .framework()
            .options()
            .prefix_options
            .commands
            .iter()
            .find(|cmd| cmd.name == command)
        {
            match command.options.multiline_help {
                Some(f) => f(),
                None => command
                    .options
                    .inline_help
                    .unwrap_or("No help available")
                    .to_owned(),
            }
        } else {
            format!("No such command `{}`", command)
        }
    } else {
        let is_also_a_slash_command = |command_name| {
            let slash_commands = &ctx.framework().options().slash_options.commands;
            slash_commands.iter().any(|c| c.name == command_name)
        };

        let mut categories = indexmap::IndexMap::new();
        for cmd in &ctx.framework().options().prefix_options.commands {
            categories
                .entry(cmd.options.category)
                .or_insert(Vec::new())
                .push(cmd);
        }

        let mut menu = String::from("```\n");
        for (category_name, commands) in categories {
            menu += category_name.unwrap_or("Commands");
            menu += ":\n";
            for command in commands {
                if command.options.hide_in_help {
                    continue;
                }

                let prefix = if is_also_a_slash_command(command.name) {
                    '/'
                } else {
                    '?'
                };

                menu += &format!(
                    "  {}{:<12}{}\n",
                    prefix,
                    command.name,
                    command.options.inline_help.unwrap_or("")
                );
            }
        }
        menu += "\nType ?help command for more info on a command.";
        menu += "\nYou can edit your message to the bot and the bot will edit its response.";
        menu += "\n```";

        menu
    };

    poise::say_reply(ctx, reply).await?;

    Ok(())
}

pub async fn is_owner(ctx: crate::PrefixContext<'_>) -> Result<bool, Error> {
    Ok(ctx.msg.author.id.0 == 472029906943868929)
}

/// Register slash commands in this guild or globally
///
/// Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(check = "is_owner", hide_in_help)]
pub async fn register(ctx: PrefixContext<'_>, #[flag] global: bool) -> Result<(), Error> {
    let guild_id = ctx.msg.guild_id.ok_or("Must be called in guild")?;
    let commands = &ctx.framework.options().slash_options.commands;
    poise::say_prefix_reply(ctx, format!("Registering {} commands...", commands.len())).await?;
    for cmd in commands {
        if global {
            cmd.create_global(&ctx.discord.http).await?;
        } else {
            cmd.create_in_guild(&ctx.discord.http, guild_id).await?;
        }
    }
    poise::say_prefix_reply(ctx, "Done!".to_owned()).await?;
    Ok(())
}