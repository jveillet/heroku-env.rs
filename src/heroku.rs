pub mod heroku {

    use reqwest;
    use reqwest::header::{Authorization, ContentType, Headers};
    use std::collections::HashMap;

    pub struct PlatformAPI {
        client: reqwest::Client,
        token: String,
    }

    impl PlatformAPI {
        pub fn new(token: String) -> PlatformAPI {
            let client = reqwest::Client::new();
            PlatformAPI {
                client: client,
                token: token,
            }
        }

        pub fn set_config_vars(&mut self, app_name: String, configs: HashMap<String, String>) {
            let url = format!("https://api.heroku.com/apps/{}/config-vars", app_name);

            let mut response = self.client
                .patch(&url)
                .headers(self.construct_headers())
                .json(&configs)
                .send()
                .expect("Failed to set the app config vars.");

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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn has_token() {
            let token = String::from("1234");
            let client_test = PlatformAPI::new(token);
            assert_eq!("1234", client_test.token);
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
