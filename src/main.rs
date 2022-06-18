use qc_bot::*;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::money::BAL_COMMAND;
use crate::money::PAY_COMMAND;
use crate::money::TAX_COMMAND;
use crate::money::TRIVIA_COMMAND;

#[group]
#[commands(start_game, tax, bal, pay, trivia)]
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
    msg.reply(ctx, format!("Starting game with mentionned players"))
        .await?;

    Ok(())
}
