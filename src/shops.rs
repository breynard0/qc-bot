use serenity::framework::standard::macros::{command};
use serenity::framework::standard::{CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::colours;

use crate::file_sys::*;

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
async fn add_item(_ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    shop_user.items.push(ShopItem { name: name, price: price, emoji: emoji });

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
async fn remove_item(_ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    for i in shop_user.items.clone() {
        if i.name == name {
            shop_user.items.remove(shop_user.items.iter().position(|s| s == &i).unwrap());
        }
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

    for i in _user.items {
        content.push_str(format!("{} {}: ${}\n", i.emoji, i.name, i.price).as_str());
    }

    msg.channel(ctx).await.unwrap().guild().unwrap().send_message(ctx, |m| {
        m.content("")
        .embed(|e| {
            e.title(format!("{}'s shop", _user.user))
            .field("Items:", content, true)
            .colour(colours::roles::BLUE)
        })
    })
    .await?;
    
    Ok(())
}