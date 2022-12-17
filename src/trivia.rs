use std::time::SystemTime;

use poise::serenity_prelude::colours;
use rand::{Rng, seq::SliceRandom};

use crate::{config::{self, Trivia}, file_sys::{self, MoneyUser, MoneyUsers, Context, CommandOutput}};


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

    let channel = sender.create_dm_channel(ctx).await?;

    channel
        .send_message(ctx, |b| b.content("Sending question..."))
        .await?;

    let channel_msg = &channel
        .messages(ctx, |retriever| retriever.limit(1))
        .await?[0];
    let mut answered = false;

    let qc_question = config::get_config()
    .trivia_question
    .get(rand::prelude::thread_rng().gen_range(0..config::get_config().trivia_question.len()))
    .unwrap()
    .clone();

    let gen = rand::prelude::thread_rng().gen_range(0..2);
    let question = match gen {
        0 => {
            qc_question
        }
        1 => {
            request_question().await
        }
        _ => {
            Trivia::default()
        }
    };

    channel
        .send_message(ctx, |m| {
            m.content("").tts(true).embed(|e| {
                e.title("Write your answer in chat")
                    .description(question.question)
                    .color(colours::roles::BLUE)
            })
        })
        .await?;

    let past_time = std::time::SystemTime::now();
    while !answered {
        std::thread::sleep(std::time::Duration::from_millis(200));

        let cur_msg = &channel
            .messages(ctx, |retriever| retriever.limit(1))
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

            let mut correct = cur_msg_content == question_answer;
            if std::time::SystemTime::now().duration_since(past_time).unwrap().as_secs() > 15 {
                cur_msg
                    .reply(ctx, "Did not reply fast enough, so counting it as wrong. You Googled it, didn't you?")
                    .await?;
                correct = false;
            }

            if correct {
                cur_msg
                    .reply(ctx, "You got it right! Adding $30 to your account!")
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
                        ctx,
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

async fn request_question() -> Trivia {
    use opentdb::api_response::ApiResponse;
    let category = rand::prelude::thread_rng().gen_range(15..30);

    let text = reqwest::get(format!("https://opentdb.com/api.php?amount=1&difficulty=easy&category={}", category)).await.unwrap().text().await.unwrap();
    let rs: ApiResponse = serde_json::from_str(text.as_str()).unwrap();
    let res = &rs.results[0];

    let mut q = rs.results[0].question.clone();
    q = format!("**{}**\nOptions:", q);
    
    let mut possible_answers = vec![res.correct_answer.clone()];
    res.incorrect_answers.iter().for_each(|s| possible_answers.push(s.to_string()));
    possible_answers.shuffle(&mut rand::prelude::thread_rng());
    for s in possible_answers {
        q = format!("{}\n- {}", q, s);
    }

    q = q.replace("&quot;", "\"");
    q = q.replace("&#039;", "\'");

    let mut a = res.correct_answer.clone();
    a = a.replace("&quot;", "\"");
    a = a.replace("&#039;", "\'");

    Trivia {
        question: q,
        answer: a
    }
}