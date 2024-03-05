pub fn get_tweet_message() -> String {
    read_json_message()
}

use crate::utils::read_from_data_dir::read_data;

#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
pub struct Messages {
    pub PrimaryMessage: String,
}

fn read_json_message() -> String {
    let json_content = read_data("messages.json".to_string());

    let parsed_json: Vec<Messages> =
        serde_json::from_str(&json_content).expect("Failed to parse JSON");

    let primary_message = &parsed_json[0].PrimaryMessage;
    primary_message.to_string()
}
