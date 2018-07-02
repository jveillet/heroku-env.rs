//! heroku-env is a command line utility to intercat with heroku config vars
//!
//! Project Repository: (https://gitlab.com/jveillet/heroku-env)[gitlab.com/jveillet/heroku-env]
//!
//! # Licence
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! # Usage
//! ```
//! heroku-env 0.0.6
//! Jérémie Veillet <jeremie.veillet@gmail.com>
//! CLI to Update or create environment variables on Heroku written in Rust.
//!
//! USAGE:
//!    heroku-env [SUBCOMMAND]

//! FLAGS:
//!    -h, --help       Prints help information
//!    -V, --version    Prints version information
//!
//! SUBCOMMANDS:
//!    help    Prints this message or the help of the given subcommand(s)
//!    push    Push local config vars to heroku
//! ```
//!
extern crate clap;
extern crate dotenv;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;

extern crate serde_json;

use clap::{App, Arg, SubCommand};

use dotenv::dotenv;
use std::env;

mod heroku;
use heroku::heroku as platform_api;

mod config;
use config::config as cfg;

fn main() {
    let matches = App::new("heroku-env")
        .version("0.0.7")
        .author("Jérémie Veillet <jeremie.veillet@gmail.com>")
        .about("CLI to Update or create environment variables on Heroku written in Rust.")
        .subcommand(
            SubCommand::with_name("push")
                .about("Push local config vars to heroku")
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
    match cfg::Config::from_path(config_file_path) {
        Ok(heroku_config) => update_config_vars(heroku_config),
        Err(err) => println!("Error: {}", err),
    }
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
        if app.settings.is_empty() {
            println!(
                "Skipping update for => {}, no settings were found.",
                app.name
            );
        } else {
            println!(".:: Updating heroku app {}", app.name);
            client.set_config_vars(app.name.to_string(), app.settings);
            println!("Done ::.");
        }
    }
}
