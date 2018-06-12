extern crate clap;
extern crate dotenv;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;

use clap::{App, Arg};

use dotenv::dotenv;
use std::env;

mod heroku;
use heroku::heroku as platform_api;

mod config;
use config::config as cfg;

fn main() {
    let matches = App::new("heroku-env")
        .version("0.0.5")
        .author("Jérémie Veillet <jeremie.veillet@gmail.com>")
        .about("CLI to Update or create environment variables on Heroku written in Rust.")
        .arg(
            Arg::with_name("run")
                .short("r")
                .long("run")
                .help("Create or update config vars on Heroku"),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a user defined config file in YAML format")
                .takes_value(true)
                .requires("run"),
        )
        .get_matches();

    dotenv().expect("Couldn't find a .env file. Please create a .env file first.");

    if matches.is_present("run") {
        let file_path;
        if let Some(path) = matches.value_of("config") {
            file_path = path.to_string();
        } else {
            let home_dir = env::home_dir().unwrap();
            file_path = format!("{}/.heroku-env/config.yml", home_dir.display())
        }
        let heroku_config = cfg::Config::from_path(file_path);
        update_config_vars(heroku_config);
    }
}

/// Intialize an Heroku Platform API Client
///
/// # Result
/// Platform_api::PlatformAPI
///
fn heroku_client() -> platform_api::PlatformAPI {
    let heroku_api_token =
        env::var("HK_API_TOKEN").expect("HK_API_TOKEN env variable not found in the .env");

    platform_api::PlatformAPI::new(heroku_api_token.to_string())
}

/// Lauch the update of config vars for every app in the config file.
///
/// config: Config Struct
///
fn update_config_vars(config: cfg::Config) {
    let mut client = heroku_client();

    for app in config.apps {
        println!("Updating app {}", app.name);
        if app.settings.is_empty() {
            println!(
                "Skipping update for => {}, no settings were found.",
                app.name
            );
        } else {
            client.set_config_vars(app.name.to_string(), app.settings);
        }
    }
}
