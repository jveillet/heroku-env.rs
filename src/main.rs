extern crate clap;
extern crate dotenv;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;

use clap::App;

use dotenv::dotenv;
use std::env;

mod heroku;
use heroku::heroku as platform_api;

mod config;
use config::config as cfg;

fn main() {
    let _matches = App::new("heroku-env")
        .version("0.0.3")
        .author("Jérémie Veillet <jeremie.veillet@gmail.com>")
        .about("Update environment variables on Heroku pipelines.")
        .get_matches();

    dotenv().expect("Couldn't find a .env file. Please create a .env file first.");

    let heroku_config = cfg::Config::new(path());
    update_config_vars(heroku_config);
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

/// Get the configuration file path
/// # Result
/// Sting
/// The local configuration path in development or in the home directory
///
fn path() -> String {
    if is_development() {
        println!("APP_ENV is development.");
        String::from("config/config.yml")
    } else {
        let home_dir = env::home_dir().unwrap();
        format!("{}/.heroku-env/config.yml", home_dir.display())
    }
}

/// Check if the environment is in development mode.
///
/// # Result
/// bool True if the environment is in development mode.
///
fn is_development() -> bool {
    let app_env = env::var("APP_ENV").ok();
    let app_env = app_env
        .as_ref()
        .map(String::as_str)
        .unwrap_or("development");

    app_env == "development"
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
                "Skipping update for => {:?}, no settings were found.",
                app.name
            );
        } else {
            client.set_config_vars(app.name.to_string(), app.settings);
        }
    }
}
