use crate::*;
use std::time::SystemTime;

use crate::file_sys::{CommandOutput, Context, MoneyUser, MoneyUsers};
use rand::Rng;

use serenity::model::prelude::*;
use serenity::utils::colours;

/// Check balance of a user or yourself
#[poise::command(slash_command, prefix_command)]
pub async fn bal(
    ctx: Context<'_>,
    #[description = "User whose balance you want to see."] user: User,
) -> CommandOutput {
    let data: MoneyUsers = file_sys::de_money();

    let mut mu = MoneyUser {
        user: user.name.to_string(),
        money: 100,
        last_redeem: SystemTime::UNIX_EPOCH,
    };
    if !data.usernames.contains(&mu.user) {
        ctx.say(format!("{} has $100", mu.user)).await?;
    }

    for u in data.users.clone() {
        if u.user == user.name {
            mu = u;
            ctx.say(format!("{} has ${}", mu.user, mu.money)).await?;
        }
    }

    Ok(())
}

/// Add or remove money from somone. Admin command
#[poise::command(slash_command)]
pub async fn tax(
    ctx: Context<'_>,
    #[description = "User to tax"] user: User,
    #[description = "Amount to add"] amount: i32,
) -> CommandOutput {
    let sender = ctx.author();
    let mut data: MoneyUsers = file_sys::de_money();

    if config::get_config()
        .admin_whitelist
        .contains(sender.id.as_u64())
    {
        ctx.say(format!("Taxing {} for {}$", user.name, &amount))
            .await?;

        let mut mu = MoneyUser {
            user: user.name.clone(),
            money: 100,
            last_redeem: SystemTime::UNIX_EPOCH,
        };
        if !data.usernames.contains(&mu.user) {
            data.usernames.push(mu.user.clone());
            data.users.push(mu.clone());
            file_sys::ser_money(data.clone());
        }

        for u in data.users.clone() {
            if u.user == user.name {
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

    file_sys::log(
        &format!(
            "{} taxed {} for ${} at {}",
            ctx.author().name,
            user.name,
            amount,
            chrono::Local::now().format("%Y-%m-%d][%H:%M:%S")
        ),
        ctx,
    )
    .await;

    Ok(())
}

/// Pay a certain amount of money to another user
#[poise::command(slash_command)]
pub async fn pay(
    ctx: Context<'_>,
    #[description = "User to pay"] user: User,
    #[description = "Amount to pay"] amount: i32,
) -> CommandOutput {
    let sender = ctx.author();
    let mut data: MoneyUsers = file_sys::de_money();
    let mut sender_data: MoneyUser = MoneyUser {
        user: "".to_string(),
        money: 100,
        last_redeem: SystemTime::UNIX_EPOCH,
    };

    if amount < 0 {
        ctx.say("You can't take money, scammer!").await?;
        return Ok(());
    }

    for u in data.users.clone() {
        if u.user == sender.name {
            sender_data = u;
        }
    }

    if amount > sender_data.money {
        ctx.say(format!(
            "You don't have enough money for this! Missing: ${}",
            amount - sender_data.money
        ))
        .await?;
        return Ok(());
    }

    let target = user;
    let mut target_data: MoneyUser = MoneyUser {
        user: "".to_string(),
        money: 100,
        last_redeem: SystemTime::UNIX_EPOCH,
    };

    if !data.usernames.contains(&target.name) {
        target_data = MoneyUser {
            money: 100,
            user: target.name.to_string(),
            last_redeem: SystemTime::UNIX_EPOCH,
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

    if target.name == sender.name {
        ctx.say("You can't send money to yourself").await?;
        return Ok(());
    }

    ctx.say(format!("Paying ${} to {}", amount, target.name))
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

/// Pay $20 for a trivia question, get $30 back if you get it right
#[poise::command(prefix_command, slash_command)]
pub async fn trivia(ctx: Context<'_>) -> CommandOutput {
    let sender = ctx.author();
    let amount = -20;
    let mut data: MoneyUsers = file_sys::de_money();

    let mut mu = MoneyUser {
        user: ctx.author().name.to_string(),
        money: 100,
        last_redeem: SystemTime::UNIX_EPOCH,
    };
    if !data.usernames.contains(&mu.user) {
        data.usernames.push(mu.user.clone());
        data.users.push(mu.clone());
        file_sys::ser_money(data.clone());
    }

    for u in data.users.clone() {
        if u.user == ctx.author().name.to_string() {
            mu = u;
        }
    }

    if mu.money + amount < 0 {
        ctx.say(format!(
            "You don't have enough money for this! Missing: ${}",
            (&mu.money + &amount) * -1
        ))
        .await?;
        return Ok(());
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

    ctx.say("Took $20 and sending a trivia question to your DMs now!")
        .await?;

    let channel = sender.create_dm_channel(ctx.discord()).await?;

    channel
        .send_message(ctx.discord(), |b| b.content("Sending question..."))
        .await?;

    let channel_msg = &channel
        .messages(ctx.discord(), |retriever| retriever.limit(1))
        .await?[0];
    let mut answered = false;

    let question = config::get_config()
        .trivia_question
        .get(rand::prelude::thread_rng().gen_range(0..config::get_config().trivia_question.len()))
        .unwrap()
        .clone();

    channel
        .send_message(ctx.discord(), |m| {
            m.content("").tts(true).embed(|e| {
                e.title("Write your answer in chat")
                    .description(question.question)
                    .color(colours::roles::BLUE)
            })
        })
        .await?;

    while !answered {
        std::thread::sleep(std::time::Duration::from_millis(200));

        let cur_msg = &channel
            .messages(ctx.discord(), |retriever| retriever.limit(1))
            .await?[0];

        if cur_msg.content.is_empty() && cur_msg.author.bot {
            continue;
        }

        if cur_msg.content != channel_msg.content {
            answered = true;

            let cur_msg_content = cur_msg
                .content
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
                .to_lowercase();
            let question_answer = question
                .answer
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
                .to_lowercase();

            let correct = cur_msg_content == question_answer;

            if correct {
                cur_msg
                    .reply(
                        ctx.discord(),
                        "You got it right! Adding $30 to your account!",
                    )
                    .await?;
                ctx.say(format!("{} got it right!", ctx.author().name))
                    .await?;
                mu.money += 30;

                let idx1 = data.users.iter().position(|r| r.user == mu.user).unwrap();
                let idx2 = data.usernames.iter().position(|r| r == &mu.user).unwrap();

                data.users.remove(idx1);
                data.usernames.remove(idx2);

                data.usernames.push(mu.user.clone());
                data.users.push(mu.clone());
            } else {
                cur_msg
                    .reply(
                        ctx.discord(),
                        format!(
                            "Oh no! You didn't get it! Correct answer: {}",
                            question.answer
                        ),
                    )
                    .await?;
                ctx.say(format!("{} got it wrong :(", ctx.author().name))
                    .await?;
            }
        }
    }

    file_sys::ser_money(data);
    Ok(())
}

struct Time {
    hours: i32,
    minutes: i8,
    seconds: i8,
}

/// Redeem your daily reward
#[poise::command(prefix_command, slash_command)]
pub async fn redeem(ctx: Context<'_>) -> CommandOutput {
    let mut data: MoneyUsers = file_sys::de_money();
    let cooldown: f64 = 57600.0;

    let mut mu = MoneyUser {
        user: ctx.author().name.to_string(),
        money: 100,
        last_redeem: SystemTime::UNIX_EPOCH,
    };
    if !data.usernames.contains(&mu.user) {
        data.usernames.push(mu.user.clone());
        data.users.push(mu.clone());
        file_sys::ser_money(data.clone());
    }

    for u in data.users.clone() {
        if u.user == ctx.author().name.to_string() {
            mu = u;
        }
    }

    let mut amount = 60;

    if mu.money < 10000 {
        amount = 90;
    }

    if mu.money < 5000 {
        amount = 100;
    }

    if mu.money < 1000 {
        amount = 140;
    }

    if std::time::SystemTime::now()
        .duration_since(mu.last_redeem)
        .unwrap()
        .as_secs_f64()
        > cooldown
    {
        mu.money += amount;
        mu.last_redeem = std::time::SystemTime::now();

        let idx1 = data.users.iter().position(|r| r.user == mu.user).unwrap();
        let idx2 = data.usernames.iter().position(|r| r == &mu.user).unwrap();

        data.users.remove(idx1);
        data.usernames.remove(idx2);

        data.usernames.push(mu.user.clone());
        data.users.push(mu.clone());

        file_sys::ser_money(data);

        ctx.say(format!("Redeemed ${}!", amount)).await?;
    } else {
        let secs_until: f64 = (cooldown
            - std::time::SystemTime::now()
                .duration_since(mu.last_redeem)
                .unwrap()
                .as_secs_f64())
        .round();

        let duration = Time {
            hours: (secs_until / 3600.0).floor() as i32,
            minutes: ((secs_until % 3600.0) / 60.0).floor() as i8,
            seconds: ((secs_until % 3600.0) % 60.0).floor() as i8,
        };

        ctx.say(format!("You already redeemed your reward! You can redeem again in {} hours, {} minutes and {} seconds", 
            duration.hours, 
            duration.minutes, 
            duration.seconds
        )).await?;
    }

    Ok(())
}

/// Shows top five richest users
#[poise::command(prefix_command, slash_command)]
pub async fn leaderboard(ctx: Context<'_>) -> CommandOutput {
    let data = file_sys::de_money();

    let mut first = MoneyUser {
        user: "Blank".to_string(),
        money: 0,
        last_redeem: std::time::UNIX_EPOCH,
    };
    let mut second = MoneyUser {
        user: "Blank".to_string(),
        money: 0,
        last_redeem: std::time::UNIX_EPOCH,
    };
    let mut third = MoneyUser {
        user: "Blank".to_string(),
        money: 0,
        last_redeem: std::time::UNIX_EPOCH,
    };
    let mut forth = MoneyUser {
        user: "Blank".to_string(),
        money: 0,
        last_redeem: std::time::UNIX_EPOCH,
    };
    let mut fifth = MoneyUser {
        user: "Blank".to_string(),
        money: 0,
        last_redeem: std::time::UNIX_EPOCH,
    };

    for u in data.users {
        if u.money > first.money {
            fifth = forth;
            forth = third;
            third = second;
            second = first;
            first = u.clone();
            continue;
        } else if u.money > second.money {
            fifth = forth;
            forth = third;
            third = second;
            second = u.clone();
            continue;
        } else if u.money > third.money {
            fifth = forth;
            forth = third;
            third = u.clone();
            continue;
        } else if u.money > forth.money {
            fifth = forth;
            forth = u.clone();
            continue;
        } else if u.money > fifth.money {
            fifth = u.clone();
            continue;
        }
    }

    let content = format!(
        "1) {}, {}\n2) {}, {}\n3) {}, {}\n4) {}, {}\n5) {}, {}",
        first.user,
        first.money,
        second.user,
        second.money,
        third.user,
        third.money,
        forth.user,
        forth.money,
        fifth.user,
        fifth.money
    );

    ctx.send(|m| {
        m.content("").embed(|e| {
            e.title("Leaderboard")
                .field("Top five:", content, false)
                .colour(colours::roles::ORANGE)
        })
    })
    .await?;

    Ok(())
}
