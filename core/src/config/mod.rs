use ::result::*;
use toml;
use std::env;
use std::io;

#[derive(Deserialize, Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    threads: u32,
}

#[derive(Deserialize, Debug)]
struct DbConfig {
    host: String,
    port: u16,
    user: String,
    pass: String,
}

#[derive(Deserialize, Debug)]
struct OptionsConfig {

}

#[derive(Deserialize, Debug)]
pub struct Config {
    server: ServerConfig,
    db: DbConfig,
    options: OptionsConfig,
}

impl Config {
    pub fn threads_limit(&self) -> u32 {
        self.server.threads
    }

    pub fn bind(&self) -> String {
        format!("{}:{}", &self.server.host, &self.server.port)
    }

    pub fn dsn(&self) -> String {
        let db = &self.db;
        format!("{}:{}@{}:{}/?prefer_socket=false", db.user, db.pass, db.host, db.port)
    }
}

fn config_path() -> Result<String> {
    let args = env::args().collect();
    let config_prefix = "--config=";
    let len = config_prefix.len();
    let arg = args.into_iter().find(|&i| i[0..len] == config_prefix)?;
    arg[len..]
}

pub fn load() -> Config {
    let file_path = config_path();
    let mut handle = File::open(&file_path)?;
    let mut content = String::new();
    handle.read_to_string(content)?;
    toml::from_str(&content)?
}
