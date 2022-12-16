use poise::serenity_prelude::{Color, CacheHttp};

use crate::file_sys::*;

/// Print out rules messages
#[poise::command(prefix_command, slash_command)]
pub async fn rules(ctx: Context<'_>) -> CommandOutput {
    if !crate::config::get_config()
        .admin_whitelist
        .contains(ctx.author().id.as_u64())
    {
        ctx.say("Insufficient permissions!").await?;
        return Ok(());
    }

    ctx.channel_id().send_message(ctx.http(), |x| x.content("https://cdn.discordapp.com/attachments/841058266452983829/1049070321115861032/qc_rules.png")).await?;

    ctx.channel_id().send_message(ctx.http(),|x| x.embed(|x| x.color(Color::GOLD).title("1. **No Outsiders**").field("​", "You **MUST** go to the school to be in this server. If you have gone to QC in the past, you are allowed to join and will be given the Elder Coyote role.", false))).await?;
    ctx.channel_id().send_message(ctx.http(),|x| x.embed(|x| x.color(Color::RED).title("2. **No Disrespect**").field("​", "We do not tolerate disrespect in any kind here. If you have something rude to say, please keep it to yourself. Also, please do not disrespect someone's interests or hobbies.", false))).await?;
    ctx.channel_id().send_message(ctx.http(),|x| x.embed(|x| x.color(Color::TEAL).title("3. **Swearing**").field("​", "Swearing *is* allowed. But keep it to a minimum. Racial/homophobic/transphobic slurs are not tolerated here. If you say one of these you will be immediately banned or muted.", false))).await?;
    ctx.channel_id().send_message(ctx.http(),|x| x.embed(|x| x.color(Color::DARK_BLUE).title("4. **No NSFW or Illegal Content**").field("​", "Inappropriate images, scam/hack links, or anything like that are not allowed in this server. You'll most likely be immediately banned or muted.", false))).await?;
    ctx.channel_id().send_message(ctx.http(),|x| x.embed(|x| x.color(Color::ORANGE).title("5. **No Spamming**").field("​", "Please keep the amount of messages at a time to a minimum or it will result in a warning or mute.", false))).await?;
    ctx.channel_id().send_message(ctx.http(),|x| x.embed(|x| x.color(Color::KERBAL).title("**Final Notes**").field("​", "Do not abuse any loopholes in these rules. If you need *anything*, feel free to DM any admin or moderator.", false))).await?;

    Ok(())
}
