//! Main configuration interface to load a YML file into a structure
//!
//! # Example
//!
//! ```rust
//! let c = config::Config::from_path("/home/john/test.yml");
//! match c {
//!     Ok(hc) => println!("Config data: {:?}", hc),
//!     Err(err) => println!("Error: {}", err),
//! }
//! ```
pub mod config {

    use std;

    use std::collections::HashMap;

    use std::fs::File;
    use std::io::Read;
    use std::io::Write;

    use serde_yaml;

    static VERSION: &str = "1";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        /// The version of the YAML file
        pub version: String,
        /// The list of apps
        pub apps: Vec<App>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct App {
        /// The name of an app
        pub name: String,
        /// Key-value pair of settings
        #[serde(default)]
        pub settings: HashMap<String, String>,
    }

    impl App {
        /// Initialize a new App struct
        ///
        /// # Result
        /// * `App` - An App struct containing name and settings
        ///
        pub fn new() -> Self {
            let app_name: String = String::new();
            let settings: HashMap<String, String> = HashMap::new();
            App {
                name: app_name,
                settings: settings,
            }
        }
    }

    impl Config {
        /// Inititialize a new instance of a config Struct
        ///
        /// # Result
        /// `Config` - A Config struct
        ///
        pub fn new() -> Self {
            let app_list: Vec<App> = Vec::new();
            Config {
                version: VERSION.to_string(),
                apps: app_list,
            }
        }
        /// Initialize a configuration structure from an app and settings
        ///
        /// # Arguments
        /// * `app_name` - An app name
        /// * `settings` - A HashMap containing the settings of the app
        ///
        /// # Result
        /// * `Result<Config, String>` - A config structure or an Error message
        ///
        /// # Example
        /// ```rust
        /// let mut settings: HashMap<String, String> = HashMap::new();
        /// settings.insert("MY_VAR", "my_value");
        /// let app_name = String::from("app-name");
        /// let conf = config::Config::from_app(&app_name, settings);
        /// assert!(conf.is_ok());
        /// ```
        pub fn from_app(app_name: &str, settings: HashMap<String, String>) -> Result<Self, String> {
            let apps: App = App {
                name: app_name.to_string(),
                settings: settings,
            };
            let app_list = vec![apps];
            let config: Config = Config {
                version: VERSION.to_string(),
                apps: app_list,
            };
            Ok(config)
        }

        /// Load a configuration structure from a file path
        ///
        /// # Arguments
        /// * `path` - A string containing the path to the config file in YAML format
        ///
        /// # Result
        /// * `Result<Config, String>` - A config structure or an Error message
        ///
        /// # Example
        /// ```rust
        /// let conf = config::Config::from_path("/home/john/test.yml");
        /// assert!(conf.is_ok());
        /// ```
        pub fn from_path(path: String) -> Result<Self, String> {
            let yaml_file = match Config::load(path) {
                Ok(f) => f,
                Err(err) => return Err(err.to_string()),
            };

            let config: Config = match serde_yaml::from_str(&yaml_file) {
                Ok(c) => c,
                Err(err) => return Err(err.to_string()),
            };

            Ok(config)
        }

        /// Save a configuration struct to a YAML file
        ///
        /// # Arguments
        /// * `path` - A string containing the path to store the config file
        ///
        /// # Result
        /// * `Result<String, std::io::Error>` - A success string or an Error message
        ///
        /// # Example
        /// ```rust
        /// let mut settings: HashMap<String, String> = HashMap::new();
        /// settings.insert("MY_VAR", "my_value");
        /// let app_name = String::from("app-name");
        /// let conf = config::Config::from_app(&app-name, settings).unwrap();
        /// let result = conf.save("my-dir/myfile.yml");
        /// assert_eq!(result.unwrap(), "Successfully created config file at my-dir/myfile.yml");
        /// ```
        pub fn save(&mut self, path: &str) -> Result<String, std::io::Error> {
            let yaml_buffer = serde_yaml::to_string(&self).unwrap();
            match File::create(&path) {
                Ok(mut f) => match f.write(&mut yaml_buffer.into_bytes()) {
                    Ok(_success) => {
                        let output: String =
                            format!("Successfully created config file at {}", &path);
                        Ok(output)
                    }
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            }
        }

        /// Helper method to load the content of a file into a String
        ///
        /// # Arguments
        /// * `path` - A string containing the path to a file
        ///
        /// # Result
        /// * `Result<String, Error>` - The content of a file or an io::Error
        ///
        fn load(path: String) -> Result<String, std::io::Error> {
            match File::open(path) {
                Ok(mut f) => {
                    let mut data = String::new();
                    f.read_to_string(&mut data)
                        .expect("Unable to read the config.yml file.");
                    Ok(data)
                }
                Err(err) => Err(err),
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_instanciate_from_app() {
            let app_name = String::from("my-fuzzy-app");
            let mut settings: HashMap<String, String> = HashMap::new();
            settings.insert("MY_VAR".to_string(), "my_value".to_string());
            let config = Config::from_app(&app_name, settings);
            assert!(config.is_ok());
        }

        #[test]
        fn is_in_version_1() {
            let path = String::from("tests/config_test.yml");
            let test_config = Config::from_path(path);
            assert_eq!(test_config.unwrap().version, "1");
        }

        #[test]
        fn has_apps() {
            let path = String::from("tests/config_test.yml");
            let config = Config::from_path(path);
            assert!(config.is_ok());
            assert_eq!(config.unwrap().apps.len(), 2);
        }

        #[test]
        fn has_settings() {
            let path = String::from("tests/config_test.yml");
            let config = Config::from_path(path);
            assert!(config.is_ok());
            let first_app = &config.unwrap().apps[0];
            assert_eq!(first_app.settings.is_empty(), false);
        }

        #[test]
        fn has_no_settings() {
            let path = String::from("tests/config_test.yml");
            let config = Config::from_path(path);
            assert!(config.is_ok());
            let first_app = &config.unwrap().apps[1];
            assert_eq!(first_app.settings.is_empty(), true);
        }

        #[test]
        fn has_config_file() {
            let path = String::from("tests/config_test.yml");
            let config = Config::from_path(path);
            assert!(config.is_ok());
        }

        #[test]
        fn has_no_config_file() {
            let path = String::from("non_existent.yml");
            let config = Config::from_path(path);
            assert_eq!(config.is_err(), true);
            let msg: String = String::from("No such file or directory (os error 2)");
            assert_eq!(config.err(), Some(msg));
        }

        #[test]
        fn has_not_the_right_format() {
            let path = String::from("tests/config_wrong_test.yml");
            let config = Config::from_path(path);
            assert_eq!(config.is_err(), true);
            let msg: String = String::from("missing field `apps` at line 1 column 8");
            assert_eq!(config.err(), Some(msg));
        }

        #[test]
        fn should_be_saved_on_disk() {
            let app_name = String::from("my-fuzzy-app");
            let mut settings: HashMap<String, String> = HashMap::new();
            settings.insert("MY_VAR".to_string(), "my_value".to_string());
            let mut config = Config::from_app(&app_name, settings).unwrap();
            let result = config.save("config/test.yml");
            assert_eq!(
                result.unwrap(),
                "Successfully created config file at config/test.yml"
            );
        }
    }
}
