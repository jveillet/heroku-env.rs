pub mod config {

    use std::collections::HashMap;

    use std::fs::File;
    use std::io::Read;

    use serde_yaml;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        pub version: String,
        pub apps: Vec<Apps>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Apps {
        pub name: String,
        #[serde(default)]
        pub settings: HashMap<String, String>,
    }

    impl Config {
        pub fn from_path(path: String) -> Self {
            let yaml_file = Config::load(path);

            let config: Config = serde_yaml::from_str(&yaml_file)
                .expect("The configuration file is not in the expected format");

            config
        }

        /// Load the content of a configuration file
        ///
        /// # Result
        /// String containing the file content.
        ///
        fn load(path: String) -> String {
            let mut data = String::new();
            let mut f = File::open(path)
                .expect("Unable to open config.yml file. Please create the file in ~/.heroku-env/");
            f.read_to_string(&mut data)
                .expect("Unable to read the config.yml file.");
            data
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn is_in_version_1() {
            let path = String::from("tests/config_test.yml");
            let test_config = Config::from_path(path);
            assert_eq!("1", test_config.version);
        }

        #[test]
        fn has_apps() {
            let path = String::from("tests/config_test.yml");
            let test_config = Config::from_path(path);
            assert_eq!(test_config.apps.len(), 2);
        }

        #[test]
        fn has_settings() {
            let path = String::from("tests/config_test.yml");
            let test_config = Config::from_path(path);
            let first_app = &test_config.apps[0];
            assert_eq!(first_app.settings.is_empty(), false);
        }

        #[test]
        fn has_no_settings() {
            let path = String::from("tests/config_test.yml");
            let test_config = Config::from_path(path);
            let first_app = &test_config.apps[1];
            assert_eq!(first_app.settings.is_empty(), true);
        }

        #[test]
        #[should_panic]
        fn has_no_config_file() {
            let path = String::from("non_existent.yml");
            Config::from_path(path);
        }

        #[test]
        #[should_panic(expected = "The configuration file is not in the expected format")]
        fn has_not_the_right_format() {
            let path = String::from("tests/config_wrong_test.yml");
            Config::from_path(path);
        }
    }
}
