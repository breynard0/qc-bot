use poise::PrefixFrameworkOptions;

use crate::file_sys::{CommandOutput, Context};
use qc_bot::file_sys::Data;
use qc_bot::*;

use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::colours;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    //DEBUG
    std::env::set_current_dir(".\\data").ok();

    //Init
    file_sys::prep_dir();

    let token = config::get_config().token;
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::all();

    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            // Register commands
            commands: vec![
                register(),
                help(),
                lizard_government(),
                money::bal(),
                money::tax(),
                money::trivia(),
                money::pay(),
                money::redeem(),
                money::leaderboard(),
                shops::add_item(),
                shops::remove_item(),
                shops::items(),
                shops::buy(),
                shops::reset_shops_channel(),
                lottery::lottery(),
                memes::upload_meme(),
                memes::get_meme(),
            ],

            prefix_options: PrefixFrameworkOptions {
                prefix: Some(config::get_config().prefix),
                mention_as_prefix: true,
                execute_untracked_edits: true,
                execute_self_messages: false,
                ignore_bots: true,
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .token(token)
        .intents(intents)
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> CommandOutput {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// lizard government
#[poise::command(prefix_command, slash_command)]
async fn lizard_government(ctx: Context<'_>) -> CommandOutput {
    let mut i: usize = 0;
    while i < 10 {
        ctx.say("lizard government").await?;
        i += 1;
    }

    Ok(())
}

/// Shows a help menu
#[poise::command(prefix_command, slash_command)]
async fn help(ctx: Context<'_>) -> CommandOutput {
    ctx.send(|m| {
        m.content("**Please use slash commands**")
        .embed(|e| {
            e.title("Help Menu (Note: Prefix in this is --, but it may be different)")
            .field("**General Commands**", "**help:** *Shows this embed. Usage: --help*\n
            **lizard_government:** *lizard government. Usage: --lizard_government*\n
            ", 
             false)

            .field("**Base Economy Commands**", "**tax:** *Add or remove money from a user. Admin command. Usage: --tax <amount> <@User>*\n
            **bal:** *Check how much money specified user has. Leave user field blank for yourself. Usage: --bal <@User>*\n
            **pay:** *Pay money to another user. Usage: --pay <amount> <@User>*\n
            **trivia:** *Get a trivia question. Costs $20. Usage: --trivia*\n
            **redeem:** *Redeem your daily reward. Usage: --redeem*\n
            **leaderboard:** *Show the top five richest users. Usage: --leaderboard*\n
            **lottery:** *Enter the lottery. Costs $100. Usage: --lottery*\n
            ", false)

            .field("**Economy Shop Commands**", "**add_item:** *Add an item to your shop. Only supports Discord built-in emojis. Usage: --add_item <name> <price> <emoji>*\n
            **remove_item:** *Removes an item from your shop. Usage: --remove_item <name>*\n
            **items:** *Check what items a shop has. Leave user field blank for yourself. Usage: --items <@User>*
            **reset_shops_channel:** *Reset items in the shops channel. Usage: --reset_shops_channel*
            **buy:** *Buy an item from another user. Usage: --buy <@User> <item_name>*", false)

            .field("**Meme Commands**", "**get_meme:** *Print a meme to chat. Usage: --get_meme*\n
            **upload_meme:** *Upload a meme. Usage: --upload_meme <link>*\n
            **items:** *Check what items a shop has. Leave user field blank for yourself. Usage: --items <@User>*
            **reset_shops_channel:** *Reset items in the shops channel. Usage: --reset_shops_channel*
            **buy:** *Buy an item from another user. Usage: --buy <@User> <item_name>*", false)

            .colour(colours::roles::BLUE)
        })
    })
    .await?;

    Ok(())
}
