//! heroku-env-rs is a command line utility to intercat with heroku config vars written in Rust
//!
//! Project Repository: (https://gitlab.com/jveillet/heroku-env-rs)[gitlab.com/jveillet/heroku-env-rs]
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
//! USAGE:
//! heroku-env-rs 0.1.8
//! Jérémie Veillet <jeremie.veillet@gmail.com>
//! CLI to interact with config vars on Heroku written in Rust.
//!
//! USAGE:
//!    hke [SUBCOMMAND]
//!
//! FLAGS:
//!    -h, --help       Prints help information
//!    -V, --version    Prints version information
//!
//! SUBCOMMANDS:
//!    help    Prints this message or the help of the given subcommand(s)
//!    pull    Pull heroku config vars down to the local environment
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
use heroku as platform_api;

mod config;
use config as cfg;

use std::collections::HashMap;

fn main() {
    let matches = App::new("heroku-env-rs")
        .version("0.1.8")
        .author("Jérémie Veillet <jeremie.veillet@gmail.com>")
        .about("CLI to interact with config vars on Heroku written in Rust.")
        .subcommand(
            SubCommand::with_name("push")
                .about("Push local config vars to heroku")
                .arg(
                    Arg::with_name("app")
                        .short("a")
                        .long("app")
                        .value_name("NAME")
                        .help("App to run command against")
                        .required_unless("config")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("Sets a user defined config file in YAML format")
                        .conflicts_with("app")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("vars")
                        .value_name("KEY=VALUE")
                        .help("Key-Value pairs of config vars ")
                        .required_unless("config")
                        .takes_value(true)
                        .multiple(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("pull")
                .about("Pull heroku config vars down to the local environment")
                .arg(
                    Arg::with_name("app")
                        .short("a")
                        .long("app")
                        .value_name("NAME")
                        .help("App to run command against")
                        .multiple(true)
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("FILE")
                        .help("Save the output to a config file in YAML format")
                        .takes_value(true),
                ),
        )
        .get_matches();

    dotenv().expect("Couldn't find a .env file. Please create a .env file first.");

    match matches.subcommand() {
        ("push", Some(push_matches)) => {
            if push_matches.is_present("app") {
                if let Some(a) = push_matches.value_of("app") {
                    let app_name = a.to_string();
                    let settings = config_vars_from_args(push_matches);
                    push_single_app(&app_name, settings);
                }
            }
            if let Some(_c) = push_matches.value_of("config") {
                if let Some(config_matches) = push_matches.value_of("config") {
                    push(config_matches.to_string());
                }
            }
        }
        ("pull", Some(pull_matches)) => {
            if pull_matches.is_present("app") {
                if let Some(apps) = pull_matches.values_of("app") {
                    let mut path = String::new();
                    if pull_matches.is_present("output") {
                        if let Some(output) = pull_matches.value_of("output") {
                            path = output.to_string();
                        }
                    }
                    pull(apps, &path)
                }
            }
        }
        ("", None) => {
            println!("No subcommand was used. For a list of subcommands, please try hke --help")
        } // If no subcommand was used it'll match the tuple ("", None)
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

/// Push heroku config vars for a single app
///
/// # Arguments
///
/// * `app_name` - The app to update.
/// * `settings` - a HashMap containing list of config vars (key-value pairs).
///
fn push_single_app(app_name: &str, settings: HashMap<String, String>) {
    match cfg::Config::from_app(&app_name, settings) {
        Ok(heroku_config) => update_config_vars(heroku_config),
        Err(err) => println!("Error: {}", err),
    }
}

/// Pull config vars down to the local machine for one or more apps
///
/// # Arguments
///
/// * `apps` - An iterator to get arguments from command line
/// * `path` - A String containing the path to the file to write config vars into. Leave blank
/// to not create a file
///
fn pull(apps: clap::Values, path: &str) {
    let mut client = heroku_client();

    let mut config: cfg::Config = cfg::Config::new();
    for app in apps {
        match client.get_config_vars(app.to_string()) {
            Ok(config_vars) => {
                let mut heroku_app = cfg::App::new();
                println!("{}", app.to_string());
                heroku_app.name = app.to_string();
                for arg in config_vars {
                    println!("{}", arg);
                    let tuple: Vec<&str> = arg.split('=').collect();
                    heroku_app
                        .settings
                        .insert(tuple[0].to_string(), tuple[1].to_string());
                }
                println!("-------------------------");
                config.apps.push(heroku_app);
            }
            Err(platform_error) => {
                println!(
                    "PlatformError: {}, {}",
                    platform_error.id, platform_error.message
                );
            }
        }
    }
    if !path.to_string().is_empty() {
        match config.save(path) {
            Ok(s) => println!("{}", s),
            Err(err) => println!("Error: {}", err),
        }
    }
}

/// Construct a Map of config vars (key-value pairs) from the command line arguments
///
/// # Arguments
///
/// * `push_matches` - List of command line arguments matchers (see clap documentation)
///
/// # Result
///
/// * `HaspMap<String, String>` - Map of config vars (key-value pairs)
///
fn config_vars_from_args(push_matches: &clap::ArgMatches) -> HashMap<String, String> {
    let mut settings: HashMap<String, String> = HashMap::new();
    if let Some(vars) = push_matches.values_of("vars") {
        for var in vars {
            let key_value: Vec<&str> = var.split('=').collect();
            settings.insert(key_value[0].to_string(), key_value[1].to_string());
        }
    }
    settings
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
                "Skipping update for app {}, no settings were found.",
                app.name
            );
        } else {
            match client.set_config_vars(app.name.to_string(), app.settings) {
                Ok(config_vars) => {
                    println!("{}", app.name);
                    for arg in config_vars {
                        println!("{}", arg);
                    }
                    println!("-------------------------");
                }
                Err(platform_error) => {
                    println!(
                        "PlatformError: {}, {}",
                        platform_error.id, platform_error.message
                    );
                    break;
                }
            }
        }
    }
}
