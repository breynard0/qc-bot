use rand::prelude::*;

use crate::file_sys::{self, log, CommandOutput, Context};

/// Upload a meme
#[poise::command(slash_command)]
pub async fn upload_meme(
    ctx: Context<'_>,
    #[description = "Discord link to meme"] link: String,
) -> CommandOutput {
    log(
        &format!(
            "Let Siliwolf know if this meme should be removed\n{}",
            link.clone()
        ),
        ctx,
    )
    .await;

    let mut data = file_sys::de_memes();
    data.push(link);
    file_sys::ser_memes(data);

    Ok(())
}

/// Get a meme in the cha
#[poise::command(slash_command, prefix_command)]
pub async fn get_meme(ctx: Context<'_>) -> CommandOutput {
    let idx = thread_rng().gen_range(0..file_sys::de_memes().len());
    ctx.say(file_sys::de_memes()[idx].clone()).await.unwrap();

    Ok(())
}
