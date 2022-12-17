#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub output_channel_id: u64,
    pub shops_channel_id: u64,
    pub admin_whitelist: Vec<u64>,
    pub trivia_question: Vec<Trivia>,
}

impl Config {
    fn new() -> Config {
        Config {
            token: String::new(),
            prefix: "--".to_string(),
            output_channel_id: 0,
            shops_channel_id: 0,
            admin_whitelist: Vec::new(),
            trivia_question: [].to_vec(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, PartialEq, PartialOrd)]
pub struct Trivia {
    pub question: String,
    pub answer: String,
}

pub fn get_config() -> Config {
    let config: Config =
        toml::from_str(&std::fs::read_to_string("./config.toml").unwrap()).unwrap();
    config
}

pub fn gen_config() {
    std::fs::File::create("./config.toml").expect("Could not create money file");

    let config = Config::new();
    std::fs::write("./config.toml", toml::to_string(&config).unwrap()).unwrap();
}
