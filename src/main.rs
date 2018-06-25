extern crate clap;
extern crate dotenv;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;

use clap::{App, Arg, SubCommand};

use dotenv::dotenv;
use std::env;

mod heroku;
use heroku::heroku as platform_api;

mod config;
use config::config as cfg;

fn main() {
    let matches = App::new("heroku-env")
        .version("0.0.6")
        .author("Jérémie Veillet <jeremie.veillet@gmail.com>")
        .about("CLI to Update or create environment variables on Heroku written in Rust.")
        .subcommand(
            SubCommand::with_name("push")                        // The name we call argument with
                                .about("Push local config vars to heroku")           // The message displayed in "myapp -h"
                                .arg(
                                    Arg::with_name("config")
                                        .short("c")
                                        .long("config")
                                        .value_name("FILE")
                                        .help("Sets a user defined config file in YAML format")
                                        .takes_value(true),
                                ),
        )
        .get_matches();

    dotenv().expect("Couldn't find a .env file. Please create a .env file first.");

    match matches.subcommand() {
        ("push", Some(push_matches)) => {
            if let Some(config_matches) = push_matches.value_of("config") {
                push(config_matches.to_string());
            } else {
                push(default_file_path().to_string());
            }
        }
        ("", None) => println!(
            "No subcommand was used. For a list of subcommand, please try heroku-env --help"
        ), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}

/// Push heroku config vars to the pipelines
///
/// # Arguments
///
/// * `config_file_path` - The config file's path in the file system.
///
fn push(config_file_path: String) {
    let heroku_config = cfg::Config::from_path(config_file_path);
    update_config_vars(heroku_config);
}

/// Defaut file path of the config file
///
/// # Result
/// * `String` - The config file in the home directory of the user.
///
/// # Examples
///
/// - `default_file_path()` -> `/home/john/.heroku-env/config.yml`
///
fn default_file_path() -> String {
    let home_dir = env::home_dir().unwrap();
    format!("{}/.heroku-env/config.yml", home_dir.display())
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
/// # Arguments
///
/// * `config` - Config Struct containing the settings structure.
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
