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

    #[derive(Debug, Deserialize)]
    pub struct PlatformError {
        /// Id of the error message
        pub id: String,
        /// Body of the error message
        pub message: String,
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
        /// # Result
        ///
        /// * `Result<Vec<String>, PlatformError>` - Vector with strings (key=value), or PaltformError struct
        ///
        /// # Example
        ///
        /// ```rust
        /// let mut client = heroku::heroku::PlatformAPI::new("1234");
        /// let settings = HashMap::new();
        /// settings.push("MY_VAR", "my value");
        /// let name = String::from("my-app");
        /// let result: Result<Vec<String>, PlatformError> = client.set_config_vars(name, settings);
        /// assert!(result.is_ok());
        /// ```
        pub fn set_config_vars(
            &mut self,
            app_name: String,
            configs: HashMap<String, String>,
        ) -> Result<Vec<String>, PlatformError> {
            let url = format!("https://api.heroku.com/apps/{}/config-vars", app_name);

            let mut response = self.client
                .patch(&url)
                .headers(self.construct_headers())
                .json(&configs)
                .send()
                .expect("Error: failed to set the app config vars.");

            // Read the body response from the API call in raw text
            let result = response.text().unwrap();

            if !response.status().is_success() {
                return Err(self.error_from_response(&result));
            }

            Ok(self.success_from_response(&result))
        }

        /// Map a successful response from the heroku API to a Vector
        ///
        /// # Arguments
        ///
        /// * `response` - JSON as text response from an API call
        ///
        /// # Result
        ///
        /// * `Vec<String>` - A vector containing strings, formated as "key=value"
        ///
        fn success_from_response(&mut self, response: &str) -> Vec<String> {
            // Parse the string response to JSON
            let serde_value: Value = serde_json::from_str(&response).unwrap();

            // Use this JSON as object to iterate on it
            let config_vars = serde_value.as_object().unwrap();

            let mut records = Vec::new();

            // Iterate over the config vars and put the result in a Vector
            for key in config_vars.keys() {
                let result: String = format!(
                    "{}={}",
                    key.to_string(),
                    config_vars.get(key).unwrap().as_str().unwrap()
                );
                records.push(result);
            }
            records
        }

        /// Map an error from an HTTP call into a PlatformError struct
        ///
        /// # Argruments
        ///
        /// * `response` - JSON as text response from an API call
        ///
        /// # Result
        ///
        /// * `PlatformError` - A struct containing the id of the error message, and the text message
        ///
        fn error_from_response(&mut self, response: &str) -> PlatformError {
            // Parse the string response to JSON and Deserialize in a PlatformError struct
            let platform_error: PlatformError = serde_json::from_str(&response).unwrap();
            platform_error
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

        #[test]
        fn should_fail_to_update_config_vars() {
            let token = String::from("1234");
            let mut client = PlatformAPI::new(token);
            let app_name = "fuzzy-app".to_string();
            let mut settings: HashMap<String, String> = HashMap::new();
            settings.insert("key".to_string(), "value".to_string());

            let result = client.set_config_vars(app_name, settings);
            assert!(result.is_err());
        }

        #[test]
        fn should_map_config_vars_as_vec() {
            let token = String::from("1234");
            let mut client = PlatformAPI::new(token);
            let http_response = "{ \"TEST\": \"VALUE\" }".to_string();
            let v: Vec<String> = client.success_from_response(&http_response);
            assert_eq!(v[0], "TEST=VALUE");
        }

        #[test]
        fn should_map_error_response() {
            let token = String::from("1234");
            let mut client = PlatformAPI::new(token);
            let http_response = "{ \"id\": \"Bad\", \"message\": \"This is bad\" }".to_string();
            let err: PlatformError = client.error_from_response(&http_response);
            assert_eq!(err.id, "Bad");
            assert_eq!(err.message, "This is bad");
        }
    }
}
