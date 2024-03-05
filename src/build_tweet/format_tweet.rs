use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub fn format_message(primary_message: String, org: &str, authors: Vec<String>) -> String {
    let mut to_format = primary_message.clone();

    to_format = to_format.replace("{org}", org);

    let authors_str = authors.join("\n");
    to_format = to_format.replace("{authors}", &authors_str);

    let result = utf8_percent_encode(to_format.as_str(), NON_ALPHANUMERIC).to_string();
    let tweet_url = format!("https://twitter.com/compose/post?text={}", result);
    println!("\n\n{}\n\n", tweet_url);

    println!("primary_message:\n\n{}\n", to_format);

    to_format
}
