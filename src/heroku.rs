pub mod heroku {

  use reqwest;
  use reqwest::header::{Headers, ContentType, Authorization};
  use std::collections::HashMap;

  pub struct PlatformAPI {
    client: reqwest::Client,
    token: String
  }

  impl PlatformAPI {

    pub fn new(token: String) -> PlatformAPI {
      let client = reqwest::Client::new();
      PlatformAPI {
        client: client,
        token: token,
      }
    }

    pub fn config_vars(&mut self, app_name: String) -> String {

      let url = format!("https://api.heroku.com/apps/{}/config-vars", app_name);

      let mut response = self.client.get(&url)
        .headers(self.construct_headers())
        .send()
        .expect("Failed to retrieve config vars.");

      println!("body = {:?}", response.text());
      response.text().unwrap()
    }

    pub fn set_config_vars(&mut self, app_name: String, configs: HashMap<&str, &str>) {

      let url = format!("https://api.heroku.com/apps/{}/config-vars", app_name);

      let mut response = self.client.patch(&url)
        .headers(self.construct_headers())
        .json(&configs)
        .send()
        .expect("Failed to retrieve config vars.");

      println!("body = {:?}", response.text());
    }

    fn construct_headers(&mut self) -> Headers {
      let mut headers = Headers::new();
      headers.set_raw("Accept", "application/vnd.heroku+json; version=3");
      headers.set(ContentType::json());
      headers.set(Authorization(format!("Bearer {}", self.token)));
      headers
    }
  }
}
