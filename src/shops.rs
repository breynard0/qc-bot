use std::time::SystemTime;

use serenity::framework::standard::macros::{command};
use serenity::framework::standard::{CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::colours;

use crate::{file_sys::{*, self}};

pub fn get_shop(username: &String) -> ShopUser {
    let mut data = de_shops();

    let mut shop_user = ShopUser { items: Vec::new(), user: username.clone() };

    if data.users.iter().position(|s| &s.user == username).is_none() {
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

#[command]
async fn add_item(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single::<String>()?;
    let price = args.single::<i32>()?;
    let emoji = args.single::<String>()?;

    let username = &msg.author.name;
    let mut data = de_shops();

    let mut shop_user = ShopUser { items: Vec::new(), user: username.clone() };

    if data.users.iter().position(|s| &s.user == username).is_none() {
        data.users.push(shop_user.clone());
        data.usernames.push(username.clone());
    }

    let pos = data.users.iter().position(|s| &s.user == username).unwrap();

    if data.usernames.contains(&username) { 
        shop_user = data.users[pos].clone();
    }

    if shop_user.clone().items.contains(&ShopItem{name: name.clone(), price, emoji: emoji.clone()}) {
        msg.reply(ctx, "Add item failed, your shop already contains an item with the same name!").await?;
        return Ok(());
    }
    else {
        shop_user.items.push(ShopItem{name: name.clone(), price, emoji});
    }

    // Publish to data
    if data.usernames.contains(&username) {
        data.users.remove(pos);
        data.usernames.remove(pos);
    }

    data.users.push(shop_user.clone());
    data.usernames.push(username.to_string());

    ser_shops(data);

    msg.reply(ctx, format!("Added item '{}' to your shop!", name)).await?;

    Ok(())
}

#[command]
async fn remove_item(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single::<String>()?;

    let username = &msg.author.name;
    let mut data = de_shops();

    let mut shop_user = ShopUser { items: Vec::new(), user: username.clone() };

    if data.users.iter().position(|s| &s.user == username).is_none() {
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
            shop_user.items.remove(shop_user.items.iter().position(|s| s == &i).unwrap());
            found = true;
        }
    }

    if !found {
        msg.reply(ctx, format!("Your shop does not have item '{}'", name)).await?;
    }
    else {
        msg.reply(ctx, format!("Removed item '{}'", name)).await?;
    }

    // Publish to data
    if data.usernames.contains(&username) {
        data.users.remove(pos);
        data.usernames.remove(pos);
    }

    data.users.push(shop_user.clone());
    data.usernames.push(username.to_string());

    ser_shops(data);

    Ok(())
}

#[command]
async fn items(ctx: &Context, msg: &Message) -> CommandResult {
    let mut _user = ShopUser { items: Vec::new(), user: String::new() };
    
    if msg.mentions.is_empty() {
        _user = get_shop(&msg.author.name.to_string());
    }
    else {
        _user = get_shop(&msg.mentions[0].name.to_string());
    }

    let mut content = "".to_string();

    for i in _user.clone().items {
        content.push_str(format!("{} {}: ${}\n", i.emoji, i.name, i.price).as_str());
    }

    if _user.items.is_empty() {
        msg.channel(ctx).await.unwrap().guild().unwrap().send_message(ctx, |m| {
            m.content("")
            .embed(|e| {
                e.title(format!("{}'s shop", _user.user))
                .field("Items:", "There are no items in this shop", true)
                .colour(colours::roles::BLUE)
            })
        })
        .await?;
    }
    else {
        msg.channel(ctx).await.unwrap().guild().unwrap().send_message(ctx, |m| {
            m.content("")
            .embed(|e| {
                e.title(format!("{}'s shop", _user.user))
                .field("Items:", content, true)
                .colour(colours::roles::BLUE)
            })
        })
        .await?;
    }
    
    
    Ok(())
}

#[command]
async fn buy(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let _name = args.single::<String>()?;
    let item = args.single::<String>()?;

    if msg.mentions.is_empty() {
        msg.reply(ctx, "Please specify a user in the first argument").await?;
        return Ok(())
    }

    let user = msg.mentions[0].clone();
    let author = &msg.author;

    let shop = get_shop(&user.name);
    let mut shop_item = ShopItem { name: String::new(), emoji: String::new(), price: 0 };

    let mut found = false;
    for i in shop.items {
        if i.name == item {
            found = true;
            shop_item = i;
        }
    }

    if !found {
        msg.reply(ctx, "Could not find item specified in second argument. Make sure the item name is spelled correctly (this is case sensitive!) and it is in the right command spot").await?;
        return Ok(())
    }

    if msg.mentions[0].name == msg.author.name {
        msg.reply(ctx, "You can't buy your own thing!").await?;
        return Ok(())
    }

    let dm = author.create_dm_channel(ctx).await?;
    let content = format!("Please verify you would like to buy {} {} for ${} by typing '_QC' below. Type anything else to cancel this", 
    shop_item.emoji, 
    shop_item.name, 
    shop_item.price);
    dm.send_message(ctx, |m| m.content(content)).await?;

    let mut answered = false;
    let last_msg = &dm
    .messages(ctx, |retriever| retriever.limit(1))
    .await?[0];

    while !answered {
        std::thread::sleep(std::time::Duration::from_millis(200));

        let cur_msg = &dm
        .messages(ctx, |retriever| retriever.limit(1))
        .await?[0];

        if cur_msg.content.is_empty() && cur_msg.author.bot {
            continue;
        }

        if cur_msg.content != last_msg.content {
            answered = true;
            
            let dm_msg = &dm.messages(ctx, |retriever| retriever.limit(1)).await?[0];

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
                    msg.reply(ctx, "You can't take money, scammer!").await?;
                    return Ok(());
                }

                for u in data.users.clone() {
                    if u.user == sender.name {
                        sender_data = u;
                    }
                }

                if amount > sender_data.money {
                    dm.send_message(
                        ctx,
                        |m| m.content(format!(
                            "You don't have enough money for this! Missing: ${}",
                            amount - sender_data.money
                        )),
                    )
                    .await?;
                    return Ok(());
                }

                let target = &msg.mentions[0];
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
                dm.send_message(ctx, |m| m.content("Buy order completed! Seller has been notified and should provide your service. If you have been scammed, DM @Siliwolf to help this get sorted out")).await?;

                user.dm(ctx, |m| m.content(format!("{} has bought your shop item {} {} for ${}. Please follow through on whatever service you have promised, or refund their money", author.name, shop_item.emoji, shop_item.name, shop_item.price))).await?;
            }
            else {
                dm.send_message(ctx, |m| m.content("Cancelled buy order")).await?;
            }
        }
    }

    Ok(())
}