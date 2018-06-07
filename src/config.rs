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
        pub fn new(path: String) -> Self {
            let yaml_file = Config::load(path);

            let heroku_config: Config = serde_yaml::from_str(&yaml_file)
                .expect("The configuration file is not in the expected format.");

            heroku_config
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
}
