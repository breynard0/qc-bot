use crate::*;
use std::time::SystemTime;

use crate::file_sys::{
    de_shops, ser_shops, CommandOutput, Context, MoneyUser, MoneyUsers, ShopItem, ShopUser, log,
};

use futures::{Stream, StreamExt};
use serenity::model::prelude::*;
use serenity::utils::colours;

pub fn get_shop(username: &String) -> ShopUser {
    let mut data = de_shops();

    let mut shop_user = ShopUser {
        items: Vec::new(),
        user: username.clone(),
    };

    if data
        .users
        .iter()
        .position(|s| &s.user == username)
        .is_none()
    {
        data.users.push(shop_user.clone());
        data.usernames.push(username.clone());
    }

    let pos = data.users.iter().position(|s| &s.user == username).unwrap();

    if data.usernames.contains(&username) {
        shop_user = data.users[pos].clone();
    }

    // Publish to data
    if data.usernames.contains(&username) {
        data.users.remove(pos);
        data.usernames.remove(pos);
    }

    data.users.push(shop_user.clone());
    data.usernames.push(username.to_string());

    ser_shops(data);
    shop_user
}

pub async fn reset_shops(ctx: Context<'_>) {
    let channel = &ctx
        .guild()
        .unwrap()
        .channels
        .get(&ChannelId {
            0: config::get_config().shops_channel_id,
        })
        .unwrap()
        .clone()
        .guild()
        .unwrap();

    let del_msgs = channel.messages(ctx.discord(), |r| r.limit(100)).await.unwrap();
    for msg in del_msgs {
        msg.delete(ctx.discord()).await.expect("Message does not exist");
    }

    let data = de_shops();

    for su in data.users {
        let mut content = "".to_string();

            for i in su.clone().items {
                content.push_str(format!("{} {}: ${}\n", i.emoji, i.name, i.price).as_str());
            }

            if su.items.is_empty() {
                channel.send_message(ctx.discord(), |m| {
                    m.content("").embed(|e| {
                        e.title(format!("{}'s shop", su.user))
                            .field("Items:", "There are no items in this shop", true)
                            .colour(colours::roles::BLUE)
                    })
                })
                .await.unwrap();
            } else {
                channel.send_message(ctx.discord(), |m| {
                    m.content("").embed(|e| {
                        e.title(format!("{}'s shop", su.user))
                            .field("Items:", content, true)
                            .colour(colours::roles::BLUE)
                    })
                })
                .await.unwrap();
        }
    }
}

/// Add an item to your shop
#[poise::command(slash_command)]
pub async fn add_item(
    ctx: Context<'_>,
    #[description = "Name of item"] name: String,
    #[description = "Price of item"] price: i32,
    #[description = "Emoji for item"] emoji: String,
) -> CommandOutput {
    let username = &ctx.author().name;
    let mut data = de_shops();

    let mut shop_user = ShopUser {
        items: Vec::new(),
        user: username.clone(),
    };

    if data.users.iter().position(|s| &s.user == username).is_none() {
        data.users.push(shop_user.clone());
        data.usernames.push(username.clone());
    }

    let pos = data.users.iter().position(|s| &s.user == username).unwrap();

    if data.usernames.contains(&username) {
        shop_user = data.users[pos].clone();
    }

    if shop_user.clone().items.contains(&ShopItem {
        name: name.clone(),
        price,
        emoji: emoji.clone(),
    }) {
        ctx.say("Add item failed, your shop already contains an item with the same name!")
            .await?;
        return Ok(());
    } else {
        shop_user.items.push(ShopItem {
            name: name.clone(),
            price,
            emoji,
        });
    }

    // Publish to data
    if data.usernames.contains(&username) {
        data.users.remove(pos);
        data.usernames.remove(pos);
    }

    data.users.push(shop_user.clone());
    data.usernames.push(username.to_string());

    ser_shops(data);

    ctx.say(format!("Added item '{}' to your shop!", name))
        .await?;

    reset_shops(ctx).await;

    Ok(())
}

/// Remove an item from your shop
#[poise::command(slash_command)]
pub async fn remove_item(
    ctx: Context<'_>,
    #[description = "Name of item to remove"]
    #[autocomplete = "autocomplete_item"]
    name: String,
) -> CommandOutput {
    let username = &ctx.author().name;
    let mut data = de_shops();

    let mut shop_user = ShopUser {
        items: Vec::new(),
        user: username.clone(),
    };

    if data
        .users
        .iter()
        .position(|s| &s.user == username)
        .is_none()
    {
        data.users.push(shop_user.clone());
        data.usernames.push(username.clone());
    }

    let pos = data.users.iter().position(|s| &s.user == username).unwrap();

    if data.usernames.contains(&username) {
        shop_user = data.users[pos].clone();
    }

    let mut found = false;
    for i in shop_user.items.clone() {
        if i.name == name {
            shop_user
                .items
                .remove(shop_user.items.iter().position(|s| s == &i).unwrap());
            found = true;
        }
    }

    if !found {
        ctx.say(format!("Your shop does not have item '{}'", name))
            .await?;
    } else {
        ctx.say(format!("Removed item '{}'", name)).await?;
    }

    // Publish to data
    if data.usernames.contains(&username) {
        data.users.remove(pos);
        data.usernames.remove(pos);
    }

    data.users.push(shop_user.clone());
    data.usernames.push(username.to_string());

    ser_shops(data);

    reset_shops(ctx).await;

    Ok(())
}

/// See what items a shops has
#[poise::command(slash_command)]
pub async fn items(ctx: Context<'_>, #[description = "User to get items"] user: User) -> CommandOutput {
    let mut _user = ShopUser {
        items: Vec::new(),
        user: String::new(),
    };

    _user = get_shop(&user.name.to_string());

    let mut content = "".to_string();

    for i in _user.clone().items {
        content.push_str(format!("{} {}: ${}\n", i.emoji, i.name, i.price).as_str());
    }

    if _user.items.is_empty() {
        ctx.send(|m| {
            m.content("").embed(|e| {
                e.title(format!("{}'s shop", _user.user))
                    .field("Items:", "There are no items in this shop", true)
                    .colour(colours::roles::BLUE)
            })
        })
        .await?;
    } else {
        ctx.send(|m| {
            m.content("").embed(|e| {
                e.title(format!("{}'s shop", _user.user))
                    .field("Items:", content, true)
                    .colour(colours::roles::BLUE)
            })
        })
        .await?;
    }

    Ok(())
}

// Item buy autocomplete
async fn autocomplete_item(_ctx: Context<'_>, partial: String) -> impl Stream<Item = String> {

    let data = de_shops();

    let mut items: Vec<&ShopItem> = Vec::new();
    for su in data.users.iter() {
        for i in &su.items {
            items.push(i);
        }
    }

    let mut dummy: Vec<String> = Vec::new();
    for i in items.clone() {
        dummy.push(format!("{} {}, ${}", i.emoji, i.name, i.price));
    }

    let mut content: Vec<String> = Vec::new();
    for i in items.clone() {
        content.push(i.name.clone());
    }

    futures::stream::iter(dummy.clone())
        .filter(move |name| futures::future::ready(name.starts_with(&partial)))
        .map(move |name| content[dummy.iter().position(|s| s == &name).unwrap()].clone())
}

/// Buy an item from someone
#[poise::command(slash_command)]
pub async fn buy(ctx: Context<'_>,
    #[description = "User to buy items from"]
    user: User,
    #[description = "Item to buy"]
    #[autocomplete = "autocomplete_item"]
    item: String
) -> CommandOutput {
    let _name = user.clone().name;

    let user = &user;
    let author = ctx.author();

    let shop = get_shop(&user.name);
    let mut shop_item = ShopItem {
        name: String::new(),
        emoji: String::new(),
        price: 0,
    };

    let mut found = false;
    for i in shop.items {
        if i.name == item {
            found = true;
            shop_item = i;
        }
    }

    if !found {
        ctx.say("Could not find item specified in second argument. Make sure the item name is spelled correctly (this is case sensitive!) and it is in the right command spot").await?;
        return Ok(());
    }

    if user.name == ctx.author().name {
        ctx.say("You can't buy your own thing!").await?;
        return Ok(());
    }

    let dm = author.create_dm_channel(ctx.discord()).await?;
    let content = format!("Please verify you would like to buy {} {} for ${} by typing '_QC' below. Type anything else to cancel this", 
    shop_item.emoji, 
    shop_item.name, 
    shop_item.price);
    dm.send_message(ctx.discord(), |m| m.content(content)).await?;

    let mut answered = false;
    let last_msg = &dm.messages(ctx.discord(), |retriever| retriever.limit(1)).await?[0];

    while !answered {
        std::thread::sleep(std::time::Duration::from_millis(200));

        let cur_msg = &dm.messages(ctx.discord(), |retriever| retriever.limit(1)).await?[0];

        if cur_msg.content.is_empty() && cur_msg.author.bot {
            continue;
        }

        if cur_msg.content != last_msg.content {
            answered = true;

            let dm_msg = &dm.messages(ctx.discord(), |retriever| retriever.limit(1)).await?[0];

            if dm_msg.content == "_QC" {
                //Money
                let sender = author;
                let amount = shop_item.price;
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
                    dm.send_message(ctx.discord(), |m| {
                        m.content(format!(
                            "You don't have enough money for this! Missing: ${}",
                            amount - sender_data.money
                        ))
                    })
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

                // Notification
                dm.send_message(ctx.discord(), |m| m.content("Buy order completed! Seller has been notified and should provide your service. If you have been scammed, DM @Siliwolf to help this get sorted out")).await?;

                user.dm(ctx.discord(), |m| m.content(format!("{} has bought your shop item {} {} for ${}. Please follow through on whatever service you have promised, or refund their money", author.name, shop_item.emoji, shop_item.name, shop_item.price))).await?;

                println!(
                    "{} bought {} from {} for ${}",
                    ctx.author().name, shop_item.name, user.name, shop_item.price
                );

                log(
                    &format!(
                        "{} bought {} from {} for ${} at {}",
                        ctx.author().name,
                        shop_item.name,
                        user.name,
                        shop_item.price,
                        chrono::Local::now().format("%Y-%m-%d][%H:%M:%S")
                    ),
                    ctx
                )
                .await;
            } else {
                dm.send_message(ctx.discord(), |m| m.content("Cancelled buy order"))
                    .await?;
            }
        }
    }

    Ok(())
}

/// Reset shops in the shops channel
#[poise::command(slash_command)]
pub async fn reset_shops_channel(ctx: Context<'_>,) -> CommandOutput {
    reset_shops(ctx).await;

    Ok(())
}
