use std::fs::File;
use std::io::Read;
use toml;
use serde_derive::*;

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
struct OptionsConfig {}

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
        format!(
            "{}:{}@{}:{}/?prefer_socket=false",
            db.user, db.pass, db.host, db.port
        )
    }
}

pub fn load(path: &str) -> Config {
    let mut content = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut content).unwrap();
    toml::from_str(&content).unwrap()
}
