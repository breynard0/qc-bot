#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub admin_whitelist: Vec<u64>
}

impl Config {
    fn new() -> Config
    {
        Config { token: String::new(), prefix: "--".to_string(), admin_whitelist: Vec::new() }
    }
}

pub fn get_config() -> Config {
    let config: Config = toml::from_str(&std::fs::read_to_string(".\\config.toml").unwrap()).unwrap();
    config
}

pub fn gen_config() {
    std::fs::File::create(".\\config.toml").expect("Could not create money file");

    let config = Config::new();
    std::fs::write(".\\config.toml", toml::to_string(&config).unwrap()).unwrap();
}