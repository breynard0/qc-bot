use serenity::async_trait;
use serenity::framework::standard::Args;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use werewolf_bot::*;
use werewolf_bot::file_sys::{MoneyUsers, MoneyUser};

#[group]
#[commands(start_game, tax, bal, pay)]
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
    msg.reply(ctx, format!("Starting game with mentionned players")).await?;

    Ok(())
}

#[command]
async fn bal(ctx: &Context, msg: &Message) -> CommandResult
{
    let data: MoneyUsers = file_sys::de_money();

    if msg.mentions.is_empty()
    {
        let mut mu = MoneyUser{user: msg.author.name.to_string(), money: 100};
        if !data.usernames.contains(&mu.user)
        {
            msg.reply(ctx, format!("{} has $100", mu.user)).await?;
        }

        for u in data.users.clone() {
            if u.user == msg.author.name
            {
                mu = u;
                msg.reply(ctx, format!("{} has ${}", mu.user, mu.money)).await?;
            }
        }
    }
    else
    {
        let mut mu = MoneyUser{user: msg.mentions[0].name.to_string(), money: 100};
        if !data.usernames.contains(&mu.user)
        {
            msg.reply(ctx, format!("{} has $100", mu.user)).await?;
        }

        for u in data.users.clone() {
            if u.user == msg.mentions[0].name
            {
                mu = u;
                msg.reply(ctx, format!("{} has ${}", mu.user, mu.money)).await?;
            }
        }
    }

    Ok(())
}

#[command]
async fn tax(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let sender = &msg.author;
    let amount = args.single::<i32>().unwrap();
    let mut data: MoneyUsers = file_sys::de_money();

    if config::get_config().admin_whitelist.contains(sender.id.as_u64())
    {
        msg.reply(ctx, format!("Taxing {} for {}$", msg.mentions[0].name, &amount)).await?;

        let mut mu = MoneyUser{user: msg.mentions[0].name.to_string(), money: 100};
        if !data.usernames.contains(&mu.user)
        {
            data.usernames.push(mu.user.clone());
            data.users.push(mu.clone());
            file_sys::ser_money(data.clone());
        }

        for u in data.users.clone() {
            if u.user == msg.mentions[0].name
            {
                mu = u;
            }
        }

        mu.money += amount;

        let idx1 = data.users.iter().position(|r| r.user == mu.user).unwrap();
        let idx2 = data.usernames.iter().position(|r| r == &mu.user).unwrap();

        data.users.remove(idx1);
        data.usernames.remove(idx2);

        data.usernames.push(mu.user.clone());
        data.users.push(mu.clone());
    }

    file_sys::ser_money(data);
    Ok(())
}

#[command]
async fn pay(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let sender = msg.clone().author;
    let amount = args.single::<i32>().unwrap();
    let mut data: MoneyUsers = file_sys::de_money();
    let mut sender_data: MoneyUser = MoneyUser { user: "".to_string(), money: 100 };

    if amount < 0
    {
        msg.reply(ctx, "You can't take money, scammer!").await?;
        return Ok(())
    }

    for u in data.users.clone() {
        if u.user == sender.name
        {
            sender_data = u;
        }
    }

    if amount > sender_data.money
    {
        msg.reply(ctx, format!("You don't have enough money for this! Missing: ${}", amount - sender_data.money)).await?;
        return Ok(())
    }

    let target = &msg.mentions[0];
    let mut target_data: MoneyUser = MoneyUser { user: "".to_string(), money: 100 };

    if !data.usernames.contains(&target.name)
    {
        target_data = MoneyUser{money: 100, user: target.name.to_string()};
        data.usernames.push(target_data.user.clone());
        data.users.push(target_data.clone());
        file_sys::ser_money(data.clone());
    }
    
    for u in data.users.clone() {
        if u.user == target.name
        {
            target_data = u;
        }
    }

    msg.reply(ctx, format!("Paying ${} to {}", amount, target.name)).await?;

    target_data.money += amount;
    sender_data.money -= amount;

    {
        let idx1 = data.users.iter().position(|r| r.user == target_data.user).unwrap();
        let idx2 = data.usernames.iter().position(|r| r == &target_data.user).unwrap();

        data.users.remove(idx1);
        data.usernames.remove(idx2);

        data.usernames.push(target_data.user.clone());
        data.users.push(target_data.clone());
    }

    {
        let idx1 = data.users.iter().position(|r| r.user == sender_data.user).unwrap();
        let idx2 = data.usernames.iter().position(|r| r == &sender_data.user).unwrap();

        data.users.remove(idx1);
        data.usernames.remove(idx2);

        data.usernames.push(sender_data.user.clone());
        data.users.push(sender_data.clone());
    }

    file_sys::ser_money(data);
    Ok(())
}