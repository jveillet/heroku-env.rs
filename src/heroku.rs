//! Main interface to interact with the heroku API
//!
//! This library is only allowing push of config vars to this day.
//!
pub mod heroku {

    use reqwest;
    use reqwest::header::{Authorization, ContentType, Headers};
    use serde_json;
    use serde_json::Value;
    use std::collections::HashMap;

    pub struct PlatformAPI {
        /// The reqwest client
        client: reqwest::Client,
        /// The token to use with the heroku API
        token: String,
    }

    impl PlatformAPI {
        /// Constructor of the Platform API
        ///
        /// # Arguments
        /// * `token` - A String containing the heroku API token
        ///
        /// # Result
        /// * `PlatformAPI` - A PlatformAPI structure
        ///
        /// # Example
        ///
        /// ```rust
        /// let mut client = heroku::heroku::PlatformAPI::new("1234");
        /// ```
        ///
        pub fn new(token: String) -> PlatformAPI {
            let client = reqwest::Client::new();
            PlatformAPI {
                client: client,
                token: token,
            }
        }

        /// Set config vars on heroku
        ///
        /// # Arguments
        ///
        /// * `app_name` - A string containing the app to push config vars
        /// * `configs` - A HashMap containing key-value pairs
        ///
        /// # Example
        ///
        /// ```rust
        /// let mut client = heroku::heroku::PlatformAPI::new("1234");
        /// let settings = HashMap::new();
        /// settings.push("MY_VAR", "my value");
        /// let name = String::from("my-app");
        /// client.set_config_vars(name, settings);
        /// ```
        pub fn set_config_vars(&mut self, app_name: String, configs: HashMap<String, String>) {
            let url = format!("https://api.heroku.com/apps/{}/config-vars", app_name);

            let mut response = self.client
                .patch(&url)
                .headers(self.construct_headers())
                .json(&configs)
                .send()
                .expect("Error: failed to set the app config vars.");

            // Read the body response from the API call in raw text
            let result = response.text().unwrap();

            // Parse the string response to JSON
            let serde_value: Value = serde_json::from_str(&result).unwrap();

            // Use this JSOn as object to iterate on it
            let config_vars = serde_value.as_object().unwrap();
            for key in config_vars.keys() {
                // Print the key-value pairs as a return
                println!(
                    "{:?}={:?}",
                    key,
                    config_vars.get(key).unwrap().as_str().unwrap()
                );
            }
        }

        /// Construct the necessary headers for HTTP request to heroku platform API
        ///
        /// # Result
        ///
        /// * `Headers` - A Headers struct containing HTTP headers (see reqwest documentation)
        ///
        fn construct_headers(&mut self) -> Headers {
            let mut headers = Headers::new();
            headers.set_raw("Accept", "application/vnd.heroku+json; version=3");
            headers.set(ContentType::json());
            headers.set(Authorization(format!("Bearer {}", self.token)));
            headers
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn has_token() {
            let token = String::from("1234");
            let client_test = PlatformAPI::new(token);
            assert_eq!(client_test.token, "1234");
        }

        #[test]
        fn has_headers() {
            let token = String::from("1234");
            let mut client_test = PlatformAPI::new(token);
            let headers = client_test.construct_headers();
            let auth = headers.get_raw("Authorization").unwrap();
            let accept = headers.get_raw("Accept").unwrap();
            let content_type = headers.get_raw("Content-Type").unwrap();

            assert_eq!(auth, "Bearer 1234");
            assert_eq!(accept, "application/vnd.heroku+json; version=3");
            assert_eq!(content_type, "application/json");
        }

    }
}
