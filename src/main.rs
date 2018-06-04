extern crate clap;
extern crate dotenv;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;

use clap::App;

use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

mod heroku;
use heroku::heroku as platform_api;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    version: String,
    apps: Vec<Apps>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Apps {
    name: String,
    #[serde(default)]
    settings: HashMap<String, String>,
}

fn main() {
    let _matches = App::new("heroku-env")
        .version("0.0.2")
        .author("Jérémie Veillet <jeremie.veillet@gmail.com>")
        .about("Update environment variables on Heroku pipelines.")
        .get_matches();

    dotenv().expect("Couldn't find a .env file. Please create a .env file first.");

    let yaml_file = read_config_file();
    println!("YAML file content => {}", yaml_file);

    let heroku_api_token =
        env::var("HK_API_TOKEN").expect("HK_API_TOKEN env variable not found in the .env");
    let mut client = platform_api::PlatformAPI::new(heroku_api_token.to_string());
    let heroku_config: Config = serde_yaml::from_str(&yaml_file)
        .expect("The configuration file is not in the expected format.");

    for app in heroku_config.apps {
        println!("Updating app {}", app.name);
        println!("With settings => {:?}", app.settings);
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

/// Read content from the configuration file
/// By defaul the file should be store in the user's home
/// in a .heroku-env/config.yaml file.
///
/// # Result
/// String containing the file content.
///
use std::fs::File;
use std::io::Read;
fn read_config_file() -> String {
    let complete_path = if development() {
        println!("APP_ENV is development.");
        String::from("config/config.yml")
    } else {
        let home_dir = env::home_dir().unwrap();
        format!("{}/.heroku-env/config.yml", home_dir.display())
    };
    let mut data = String::new();
    let mut f = File::open(complete_path)
        .expect("Unable to open config.yml file. Please create the file in ~/.heroku-env/");
    f.read_to_string(&mut data)
        .expect("Unable to read the config.yml file.");
    data
}

/// Check if the environment is in development mode.
///
/// # Result
/// bool True if the environment is in development mode.
///
fn development() -> bool {
    let app_env = env::var("APP_ENV").ok();
    let app_env = app_env
        .as_ref()
        .map(String::as_str)
        .unwrap_or("development");

    let mut result = false;
    if app_env == "development" {
        result = true;
    }
    result
}
