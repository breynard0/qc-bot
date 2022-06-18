use qc_bot::*;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::colours;

use crate::money::BAL_COMMAND;
use crate::money::PAY_COMMAND;
use crate::money::TAX_COMMAND;
use crate::money::TRIVIA_COMMAND;
use crate::money::REDEEM_COMMAND;

#[group]
#[commands(start_game, tax, bal, pay, trivia, redeem, help)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    //DEBUG
    std::env::set_current_dir(".\\test").ok();

    //Init
    file_sys::prep_dir();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(config::get_config().prefix))
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = config::get_config().token;
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn start_game(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, format!("Starting game with mentioned players"))
        .await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel(ctx).await.unwrap().guild().unwrap().send_message(ctx, |m| {
        m.content("")
        .embed(|e| {
            e.title("Help Menu (Note: Prefix in this is --, but it may be different)")
            .field("**General Commands**", "help: *Shows this embed. Usage: --help*\n
            ", false)

            .field("**Economy Commands**", "tax: *Add or remove money from a user. Admin command. Usage: --tax <amount> <@User>*\n
            bal: *Check how much money specified user has. Leave user field blank for yourself. Usage: --bal <@User>*\n
            pay: *Pay money to another user. Usage: --pay <amount> <@User>*\n
            trivia: *Get a trivia question. Costs $20. Usage: --trivia*\n
            redeem: *Redeem your daily reward. Usage: --redeem*\n
            ", false)
            .colour(colours::roles::BLUE)
        })
    })
    .await?;

    Ok(())
}