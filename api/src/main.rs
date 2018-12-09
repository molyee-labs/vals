#![allow(dead_code)]

extern crate core;
extern crate postgres;
extern crate serde;
extern crate toml;
#[macro_use]
extern crate serde_derive;

mod config;
mod db;
mod service;

use config::Config;
use std::env;

fn main() {
    let config = load_config();
    print!("config={:?}", config);
}

fn config_path() -> String {
    let config_prefix = "--config=";
    let len = config_prefix.len();
    match env::args().find(|ref i| &i[0..len] == config_prefix) {
        Some(arg) => arg[len..].to_string(),
        _ => "config.default.toml".to_string(),
    }
}

fn load_config() -> Config {
    let path = config_path();
    config::load(&path)
}
