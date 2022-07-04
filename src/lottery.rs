use std::time::SystemTime;

use rand::prelude::*;

use crate::*;
use crate::{file_sys::{CommandOutput, Context, de_lottery}, file_sys::*};

/// Spent $100 to enter the lottery! Winner gets
#[poise::command(slash_command, prefix_command)]
pub async fn lottery(
    ctx: Context<'_>
) -> CommandOutput {
    let mut data = de_lottery();

    let sender = ctx.author();
    let amount = -100;

    if data.users.contains(sender)
    {
        ctx.send(|m| m.content(format!("You've already entered!"))).await?;
        return Ok(())
    }

    {
        let mut money_data = de_money();
        let user = ctx.author();

        let mut mu = MoneyUser {
            user: user.name.clone(),
            money: 100,
            last_redeem: SystemTime::UNIX_EPOCH,
        };
        if !money_data.usernames.contains(&mu.user) {
            money_data.usernames.push(mu.user.clone());
            money_data.users.push(mu.clone());
            file_sys::ser_money(money_data.clone());
        }

        for u in money_data.users.clone() {
            if u.user == user.name {
                mu = u;
            }
        }

        if mu.money + amount >= 0 {
            mu.money += amount;

            let idx1 = money_data.users.iter().position(|r| r.user == mu.user).unwrap();
            let idx2 = money_data.usernames.iter().position(|r| r == &mu.user).unwrap();

            money_data.users.remove(idx1);
            money_data.usernames.remove(idx2);

            money_data.usernames.push(mu.user.clone());
            money_data.users.push(mu.clone());

            // Lottery
            data.money += 100;
            data.users.push(sender.clone());

            sender.dm(ctx.discord(), |m| m.content("You have entered the lottery! Draw will happen when 5 people have entered.")).await?;

            if data.users.len() >= 5 {
                let idx: usize = thread_rng().gen_range(0..=4);
                let winner = &data.users[idx];

                for u in data.users.clone() {
                    if u.id.as_u64() == winner.id.as_u64() {
                        u.dm(ctx.discord(), |m| m.content("You have won the lottery! Sending money to you...")).await?;

                        let mut mu = MoneyUser {
                            user: u.name.clone(),
                            money: 100,
                            last_redeem: SystemTime::UNIX_EPOCH,
                        };

                        for a in money_data.users.clone() {
                            if a.user == u.name {
                                mu = a;
                            }
                        }

                        mu.money += data.money;

                        let idx1 = money_data.users.iter().position(|r| r.user == mu.user).unwrap();
                        let idx2 = money_data.usernames.iter().position(|r| r == &mu.user).unwrap();

                        money_data.users.remove(idx1);
                        money_data.usernames.remove(idx2);

                        money_data.usernames.push(mu.user.clone());
                        money_data.users.push(mu.clone());
                        log(&format!("{} won the lottery", u.name), ctx).await;
                    }
                    else {
                        u.dm(ctx.discord(), |m| m.content("The lottery was drawn and you didn't win. Better luck next time!")).await?;
                    }
                }

                data.money = 0;
                data.users = Vec::new();
            }
                
        }
        else {
            ctx.send(|m| m.content(format!("You don't have enough money to do this (missing: ${})!", mu.money - amount))).await?;
            return Ok(())
        }

        ser_money(money_data);
        ser_lottery(data);
    }

    Ok(())
}