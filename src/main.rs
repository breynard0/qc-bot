// Branch using Poise for commands

use poise::PrefixFrameworkOptions;
use qc_bot::*;

use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::colours;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
struct Data {}

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
            commands: vec![register(), help(), age()],

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
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or(ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|m| {
        m.content("")
        .embed(|e| {
            e.title("Help Menu (Note: Prefix in this is --, but it may be different)")
            .field("**General Commands**", "**help:** *Shows this embed. Usage: --help*\n
            ", false)

            .field("**Base Economy Commands**", "**tax:** *Add or remove money from a user. Admin command. Usage: --tax <amount> <@User>*\n
            **bal:** *Check how much money specified user has. Leave user field blank for yourself. Usage: --bal <@User>*\n
            **pay:** *Pay money to another user. Usage: --pay <amount> <@User>*\n
            **trivia:** *Get a trivia question. Costs $20. Usage: --trivia*\n
            **redeem:** *Redeem your daily reward. Usage: --redeem*\n
            **leaderboard:** *Show the top five richest users. Usage: --leaderboard*\n
            ", false)

            .field("**Economy Shop Commands**", "**add_item:** *Add an item to your shop. Only supports Discord built-in emojis. Usage: --add_item <name> <price> <emoji>*\n
            **remove_item:** *Removes an item from your shop. Usage: --remove_item <name>*\n
            **items:** *Check what items a shop has. Leave user field blank for yourself. Usage: --items <@User>*
            **buy:** *Buy an item from another user. Usage: --buy <@User> <item_name>*", false)
            .colour(colours::roles::BLUE)
        })
    })
    .await?;

    Ok(())
}
