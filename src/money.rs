use std::time::SystemTime;

use crate::file_sys::{MoneyUser, MoneyUsers};
use crate::*;
use rand::Rng;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::colours;

#[command]
async fn bal(ctx: &Context, msg: &Message) -> CommandResult {
    let data: MoneyUsers = file_sys::de_money();

    if msg.mentions.is_empty() {
        let mut mu = MoneyUser {
            user: msg.author.name.to_string(),
            money: 100,
            last_redeem: SystemTime::UNIX_EPOCH
        };
        if !data.usernames.contains(&mu.user) {
            msg.reply(ctx, format!("{} has $100", mu.user)).await?;
        }

        for u in data.users.clone() {
            if u.user == msg.author.name {
                mu = u;
                msg.reply(ctx, format!("{} has ${}", mu.user, mu.money))
                    .await?;
            }
        }
    } else {
        let mut mu = MoneyUser {
            user: msg.mentions[0].name.to_string(),
            money: 100,
            last_redeem: SystemTime::UNIX_EPOCH
        };
        if !data.usernames.contains(&mu.user) {
            msg.reply(ctx, format!("{} has $100", mu.user)).await?;
        }

        for u in data.users.clone() {
            if u.user == msg.mentions[0].name {
                mu = u;
                msg.reply(ctx, format!("{} has ${}", mu.user, mu.money))
                    .await?;
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

    if config::get_config()
        .admin_whitelist
        .contains(sender.id.as_u64())
    {
        msg.reply(
            ctx,
            format!("Taxing {} for {}$", msg.mentions[0].name, &amount),
        )
        .await?;

        let mut mu = MoneyUser {
            user: msg.mentions[0].name.to_string(),
            money: 100,
            last_redeem: SystemTime::UNIX_EPOCH
        };
        if !data.usernames.contains(&mu.user) {
            data.usernames.push(mu.user.clone());
            data.users.push(mu.clone());
            file_sys::ser_money(data.clone());
        }

        for u in data.users.clone() {
            if u.user == msg.mentions[0].name {
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
    let mut sender_data: MoneyUser = MoneyUser {
        user: "&mut ".to_string(),
        money: 100,
        last_redeem: SystemTime::UNIX_EPOCH
    };

    if amount < 0 {
        msg.reply(ctx, "You can't take money, scammer!").await?;
        return Ok(());
    }

    for u in data.users.clone() {
        if u.user == sender.name {
            sender_data = u;
        }
    }

    if amount > sender_data.money {
        msg.reply(
            ctx,
            format!(
                "You don't have enough money for this! Missing: ${}",
                amount - sender_data.money
            ),
        )
        .await?;
        return Ok(());
    }

    let target = &msg.mentions[0];
    let mut target_data: MoneyUser = MoneyUser {
        user: "".to_string(),
        money: 100,
        last_redeem: SystemTime::UNIX_EPOCH
    };

    if !data.usernames.contains(&target.name) {
        target_data = MoneyUser {
            money: 100,
            user: target.name.to_string(),
            last_redeem: SystemTime::UNIX_EPOCH
        };
        data.usernames.push(target_data.user.clone());
        data.users.push(target_data.clone());
        file_sys::ser_money(data.clone());
    }

    for u in data.users.clone() {
        if u.user == target.name {
            target_data = u;
        }
    }

    msg.reply(ctx, format!("Paying ${} to {}", amount, target.name))
        .await?;

    target_data.money += amount;
    sender_data.money -= amount;

    {
        let idx1 = data
            .users
            .iter()
            .position(|r| r.user == target_data.user)
            .unwrap();
        let idx2 = data
            .usernames
            .iter()
            .position(|r| r == &target_data.user)
            .unwrap();

        data.users.remove(idx1);
        data.usernames.remove(idx2);

        data.usernames.push(target_data.user.clone());
        data.users.push(target_data.clone());
    }

    {
        let idx1 = data
            .users
            .iter()
            .position(|r| r.user == sender_data.user)
            .unwrap();
        let idx2 = data
            .usernames
            .iter()
            .position(|r| r == &sender_data.user)
            .unwrap();

        data.users.remove(idx1);
        data.usernames.remove(idx2);

        data.usernames.push(sender_data.user.clone());
        data.users.push(sender_data.clone());
    }

    file_sys::ser_money(data);
    Ok(())
}

#[command]
async fn trivia(ctx: &Context, msg: &Message) -> CommandResult {
    let sender = &msg.author;
    let amount = -20;
    let mut data: MoneyUsers = file_sys::de_money();

    let mut mu = MoneyUser {
        user: msg.author.name.to_string(),
        money: 100,
        last_redeem: SystemTime::UNIX_EPOCH
    };
    if !data.usernames.contains(&mu.user) {
        data.usernames.push(mu.user.clone());
        data.users.push(mu.clone());
        file_sys::ser_money(data.clone());
    }

    for u in data.users.clone() {
        if u.user == msg.author.name.to_string() {
            mu = u;
        }
    }

    if mu.money + amount < 0
    {
        msg.reply(ctx, format!("You don't have enough money for this! Missing: ${}", (&mu.money + &amount) * -1)).await?;
        return Ok(())
    }

    mu.money += amount;

    let idx1 = data.users.iter().position(|r| r.user == mu.user).unwrap();
    let idx2 = data.usernames.iter().position(|r| r == &mu.user).unwrap();

    data.users.remove(idx1);
    data.usernames.remove(idx2);

    data.usernames.push(mu.user.clone());
    data.users.push(mu.clone());

    file_sys::ser_money(data);
    data = file_sys::de_money();

    msg.reply(ctx, "Took $20 and sending a trivia question to your DMs now!")
        .await?;

    let channel = sender.create_dm_channel(ctx).await?;

    channel.send_message(ctx, |b|{
        b.content("Sending question...")
    }).await?;

    let channel_msg = &channel.messages(ctx, |retriever|
    {
        retriever.limit(1)
    }).await?[0];
    let mut answered = false;

    let question = config::get_config()
        .trivia_question
        .get(rand::prelude::thread_rng().gen_range(0..config::get_config().trivia_question.len()))
        .unwrap()
        .clone();

    channel
        .send_message(ctx, |m| {
            m.content("")
                .tts(true)
                .embed(|e| {
                    e.title("Write your answer in chat")
                    .description(question.question)
                    .color(colours::roles::BLUE)
                })
        })
        .await?;

    while !answered {
        std::thread::sleep(std::time::Duration::from_millis(200));

        let cur_msg = &channel.messages(ctx, |retriever| {
            retriever.limit(1)
        }).await?[0];

        if cur_msg.content.is_empty() && cur_msg.author.bot
        {
            continue;
        }

        if cur_msg.content != channel_msg.content
        {
            answered = true;

            let mut correct = cur_msg.content.to_lowercase().replace(" ", "") == question.answer.to_lowercase().replace(" ", "");

            //Buffer System
            if !correct
            {
                let greater: usize;
                let lesser: usize;

                if question.answer.len() > cur_msg.content.len()
                {
                    greater = question.answer.len();
                    lesser = cur_msg.content.len();
                }
                else
                {
                    lesser = question.answer.len();
                    greater = cur_msg.content.len();
                }

                if greater - lesser <= 2 {
                    correct = true;
                }
            }
            
            if correct {
                cur_msg.reply(ctx, "You got it right! Adding $30 to your account!").await?;
                msg.reply(ctx, format!("{} got it right!", msg.author.name)).await?;
                mu.money += 30;

                let idx1 = data.users.iter().position(|r| r.user == mu.user).unwrap();
                let idx2 = data.usernames.iter().position(|r| r == &mu.user).unwrap();

                data.users.remove(idx1);
                data.usernames.remove(idx2);

                data.usernames.push(mu.user.clone());
                data.users.push(mu.clone());
            }
            else {
                cur_msg.reply(ctx, format!("Oh no! You didn't get it! Correct answer: {}", question.answer)).await?;
                msg.reply(ctx, format!("{} got it wrong :(", msg.author.name)).await?;
            }
        }
    }

    file_sys::ser_money(data);
    Ok(())
}

struct Time
{
    hours: i32,
    minutes: i8,
    seconds: i8
}

#[command]
async fn redeem(ctx: &Context, msg: &Message) -> CommandResult {
    let amount = 60;
    let mut data: MoneyUsers = file_sys::de_money();
    let cooldown: f64 = 57600.0;

    let mut mu = MoneyUser {
        user: msg.author.name.to_string(),
        money: 100,
        last_redeem: SystemTime::UNIX_EPOCH
    };
    if !data.usernames.contains(&mu.user) {
        data.usernames.push(mu.user.clone());
        data.users.push(mu.clone());
        file_sys::ser_money(data.clone());
    }

    for u in data.users.clone() {
        if u.user == msg.author.name.to_string() {
            mu = u;
        }
    }

    if std::time::SystemTime::now().duration_since(mu.last_redeem).unwrap().as_secs_f64() > cooldown {
        mu.money += amount;
        mu.last_redeem = std::time::SystemTime::now();

        let idx1 = data.users.iter().position(|r| r.user == mu.user).unwrap();
        let idx2 = data.usernames.iter().position(|r| r == &mu.user).unwrap();

        data.users.remove(idx1);
        data.usernames.remove(idx2);

        data.usernames.push(mu.user.clone());
        data.users.push(mu.clone());

        file_sys::ser_money(data);

        msg.reply(ctx, format!("Redeemed ${}!", amount)).await?;
    }
    else
    {
        let secs_until: f64 = (cooldown - std::time::SystemTime::now().duration_since(mu.last_redeem).unwrap().as_secs_f64()).round();

        let duration = Time {
            hours: (secs_until / 3600.0).floor() as i32,
            minutes: ((secs_until % 3600.0) / 60.0).floor() as i8,
            seconds: ((secs_until % 3600.0) % 60.0).floor() as i8
        };

        msg.reply(ctx, format!("You already redeemed your reward! You can redeem again in {} hours, {} minutes and {} seconds", 
            duration.hours, 
            duration.minutes, 
            duration.seconds
        )).await?;
    }

    Ok(())
}