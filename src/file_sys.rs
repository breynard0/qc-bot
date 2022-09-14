use poise::serenity_prelude::{ChannelId, User};

use crate::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type CommandOutput = Result<(), Error>;

// User data, which is stored and accessible in all command invocations
pub struct Data {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LotterySave {
    pub users: Vec<User>,
    pub money: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MoneyUser {
    pub user: String,
    pub money: i32,
    pub last_redeem: std::time::SystemTime,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MoneyUsers {
    pub users: Vec<MoneyUser>,
    pub usernames: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShopItem {
    pub name: String,
    pub price: i32,
    pub emoji: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShopUser {
    pub user: String,
    pub items: Vec<ShopItem>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShopUsers {
    pub users: Vec<ShopUser>,
    pub usernames: Vec<String>,
}

pub fn ser_money(data: MoneyUsers) {
    let json: String = serde_json::to_string(&data).expect("Could not parse data");

    std::fs::write(".\\money.json", json).expect("Could not write to money file");
}

pub fn de_money() -> MoneyUsers {
    if std::fs::read(".\\money.json").unwrap().len() != 0 {
        serde_json::from_str(&std::fs::read_to_string(".\\money.json").unwrap().as_str()).unwrap()
    } else {
        MoneyUsers {
            users: [].to_vec(),
            usernames: [].to_vec(),
        }
    }
}

pub fn ser_lottery(data: LotterySave) {
    let json: String = serde_json::to_string(&data).expect("Could not parse data");

    std::fs::write(".\\lottery.json", json).expect("Could not write to money file");
}

pub fn de_lottery() -> LotterySave {
    if std::fs::read(".\\lottery.json").unwrap().len() != 0 {
        serde_json::from_str(&std::fs::read_to_string(".\\lottery.json").unwrap().as_str()).unwrap()
    } else {
        LotterySave {
            users: [].to_vec(),
            money: 0,
        }
    }
}

pub fn ser_shops(data: ShopUsers) {
    let json: String = serde_json::to_string(&data).expect("Could not parse data");

    std::fs::write(".\\shops.json", json).expect("Could not write to money file");
}

pub fn de_shops() -> ShopUsers {
    if std::fs::read(".\\shops.json").unwrap().len() != 0 {
        serde_json::from_str(&std::fs::read_to_string(".\\shops.json").unwrap().as_str()).unwrap()
    } else {
        ShopUsers {
            users: [].to_vec(),
            usernames: [].to_vec(),
        }
    }
}

pub fn ser_memes(data: Vec<String>) {
    let json: String = serde_json::to_string(&data).expect("Could not parse data");

    std::fs::write(".\\memes.json", json).expect("Could not write to memes file");
}

pub fn de_memes() -> Vec<String> {
    if std::fs::read(".\\memes.json").unwrap().len() != 0 {
        serde_json::from_str(&std::fs::read_to_string(".\\memes.json").unwrap().as_str()).unwrap()
    } else {
        Vec::new()
    }
}

pub fn prep_dir() {
    let mut money_exists = false;
    let mut config_exists = false;
    let mut shops_exists = false;
    let mut log_exists = false;
    let mut lottery_exists = false;
    let mut memes_exist = false;

    for file in std::fs::read_dir(".").unwrap() {
        println!(
            "Detected {:#?} in directory",
            file.as_ref().unwrap().file_name()
        );
        match file.unwrap().file_name().to_str().unwrap() {
            "money.json" => {
                money_exists = true;
            }
            "config.toml" => {
                config_exists = true;
            }
            "shops.json" => {
                shops_exists = true;
            }
            "log.txt" => {
                log_exists = true;
            }
            "lottery.json" => {
                lottery_exists = true;
            }
            "memes.json" => {
                memes_exist = true;
            }
            _ => break,
        }
    }

    if !money_exists {
        println!("Money file does not exist, creating it");
        std::fs::File::create(".\\money.json").expect("Could not create money file");
    }

    if !shops_exists {
        println!("Shops file does not exist, creating it");
        std::fs::File::create(".\\shops.json").expect("Could not create shop file");
    }

    if !config_exists {
        println!("Config file does not exist, creating it");
        config::gen_config();
    }

    if !log_exists {
        println!("Log file does not exist, creating it");
        std::fs::File::create(".\\log.txt").expect("Could not create log file");
    }

    if !lottery_exists {
        println!("Lottery file does not exist, creating it");
        std::fs::File::create(".\\lottery.json").expect("Could not create lottery file");
    }

    if !memes_exist {
        println!("Memes file does not exist, creating it");
        std::fs::File::create(".\\memes.json").expect("Could not create memes file");
    }
}

pub async fn log(text: &String, ctx: Context<'_>) {
    let file_text = std::fs::read_to_string(".\\log.txt").unwrap();

    let channel = &ctx
        .guild()
        .unwrap()
        .channels
        .get(&ChannelId {
            0: config::get_config().output_channel_id,
        })
        .unwrap()
        .clone()
        .guild()
        .unwrap();
    channel
        .send_message(&ctx.discord(), |m| m.content(text))
        .await
        .unwrap();

    let write_text = format!("{}\n{}\n", file_text, text);

    std::fs::write(".\\log.txt", write_text).expect("Could not log to file");
}
