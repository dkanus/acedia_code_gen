const DEFAULT_UNICODE_DATA_PATH: &str = "UnicodeData.txt";
const DEFAULT_TEMPLATE_PATH: &str = "template.uc";

pub struct Config {
    pub unicode_data_path: String,
    pub template_path: String,
}

impl Config {
    pub fn new(arguments: Vec<String>) -> Config {
        Config {
            unicode_data_path: arguments
                .get(1)
                .unwrap_or(&String::from(DEFAULT_UNICODE_DATA_PATH))
                .clone(),
            template_path: arguments
                .get(2)
                .unwrap_or(&String::from(DEFAULT_TEMPLATE_PATH))
                .clone(),
        }
    }
}
