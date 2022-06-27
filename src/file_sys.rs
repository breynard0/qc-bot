use crate::*;

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
    pub emoji: String
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

pub fn prep_dir() {
    let mut money_exists = false;
    let mut config_exists = false;
    let mut shops_exists = false;

    for file in std::fs::read_dir(".").unwrap() {
        println!("Detected {:#?} in directory", file.as_ref().unwrap().file_name());
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
            _ => break,
        }
    }

    if !money_exists {
        println!("Money file does not exist, creating it");
        std::fs::File::create(".\\money.json").expect("Could not create money file");
    }

    if !shops_exists {
        println!("Shops file does not exist, creating it");
        std::fs::File::create(".\\shops.json").expect("Could not create money file");
    }

    if !config_exists {
        println!("Config file does not exist, creating it");
        config::gen_config();
    }
}
