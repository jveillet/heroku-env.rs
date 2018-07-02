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

    use serde_yaml;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        /// The version of the YAML file
        pub version: String,
        /// The list of apps
        pub apps: Vec<Apps>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Apps {
        /// The name of an app
        pub name: String,
        /// Key-value pair of settings
        #[serde(default)]
        pub settings: HashMap<String, String>,
    }

    impl Config {
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
    }
}
