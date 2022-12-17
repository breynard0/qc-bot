use poise::serenity_prelude::{CacheHttp, GuildChannel, Http, Message};

use crate::file_sys::*;

/// Automatically detects messages with explicit content
#[poise::command(prefix_command, slash_command)]
pub async fn scrub(ctx: Context<'_>) -> CommandOutput {
    ctx.say(
        "Checking the server for explicit messages. This could take several minutes. Sit tight!",
    )
    .await?;
    let shift = 13;
    let blacklist = include_str!("./resources/blacklist.txt").split_whitespace();
    let mut messages = Vec::new();
    for c in ctx.guild().unwrap().channels(&ctx.http()).await? {
        for m in get_messages_in_channel(c.1, &ctx.http()).await {
            if &m.clone().author == ctx.author() {
                for b in blacklist.clone() {
                    let s = caesar_cipher::decrypt(b.to_string(), shift);

                    if m.clone()
                        .content
                        .to_lowercase()
                        .contains(s.to_lowercase().as_str())
                    {
                        messages.push(m.clone());
                    }
                }
            }
        }
    }

    let mut output: Vec<String> = Vec::new();

    for m in messages.clone() {
        let limit = 2000;
        let link = m.link();
        let mut s = match output.len() {
            0 => String::new(),
            _ => output
                .clone()
                .get(output.clone().len() - 1)
                .unwrap()
                .to_string(),
        };

        if format!("{}\n{}", s.clone(), link).len() < limit {
            s = format!("{}\n{}", s.clone(), link);
            output.pop();
            output.push(s);
        } else {
            output.push(link);
        }
    }

    for s in output.clone() {
        ctx.author().dm(&ctx.http(), |b| b.content(s)).await?;
    }

    let len = messages.len();
    if len < 1 {
        ctx.author()
            .dm(&ctx.http(), |b| {
                b.content("Congrats! Your messages are already squeaky clean!")
            })
            .await?;
    }

    Ok(())
}

pub async fn get_messages_in_channel(channel: GuildChannel, http: &Http) -> Vec<Message> {
    let mut messages: Vec<Message> = Vec::new();
    let mut cur_messages: Vec<Message> = Vec::new();
    for m in channel.messages(http, |r| r.limit(100)).await.unwrap() {
        cur_messages.push(m.clone());
    }

    while cur_messages.len() > 0 {
        for m in cur_messages.clone() {
            messages.push(m.clone());
        }
        let mut new_messages: Vec<Message> = Vec::new();
        for m in channel
            .messages(http, |r| {
                r.limit(100).before(
                    cur_messages
                        .clone()
                        .get(cur_messages.clone().len() - 1)
                        .unwrap()
                        .id,
                )
            })
            .await
            .unwrap()
        {
            new_messages.push(m.clone());
        }
        cur_messages = new_messages;
    }

    messages
}