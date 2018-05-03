extern crate clap;
extern crate reqwest;
extern crate dotenv;


use clap::{App, Arg};

use std::collections::HashMap;
use dotenv::dotenv;
use std::env;

mod heroku;
use heroku::heroku as platform_api;

fn main() {
  let matches = App::new("Heroku-Env")
        .version("0.0.1")
        .author("Jérémie Veillet <jeremie.veillet@gmail.com>")
        .about("Update environment variables on Heroku pipelines.")
        .arg(Arg::with_name("pipeline")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .short("p")
                 .help("Choose the pipelines to update."))
        .get_matches();

  let pipeline = matches.value_of("pipeline").unwrap();
  println!("Updating pipeline: {}", pipeline);


  dotenv().expect("Couldn't find a .env file. Please create a .env file first.");

  let heroku_api_token = env::var("API_TOKEN").expect("API_TOKEN env variable not found in the .env");
  let heroku_app = env::var("CONFIG_APP").expect("CONFIG_APP env variable not found in the .env");

  let mut client = platform_api::PlatformAPI::new(heroku_api_token.to_string());
  let mut map = HashMap::new();
  map.insert("TEST_VAR", "hello test var 3");

  client.set_config_vars(heroku_app.to_string(), map);
}
