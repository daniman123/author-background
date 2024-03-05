use crate::prelude::*;
use crate::utils::read_from_data_dir::read_data;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use url::{form_urlencoded::parse, Url};

struct TwitterURLQueries {
    data: TwitterURLs,
}
#[derive(Default)]
struct TwitterURLQueriesBuilder {
    data: Option<Vec<TwitterURLs>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterURLs {
    twitter_base_url: String,
    search_user: TwitterPathQuery,
    compose_post: TwitterPathQuery,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterPathQuery {
    path: String,
    query: HashMap<String, String>,
}

impl TwitterURLQueriesBuilder {
    fn new() -> Self {
        TwitterURLQueriesBuilder::default()
    }

    fn get_json_data(&mut self) -> &mut Self {
        let query_data = read_data("twitter_url_queries.json".to_string());

        let parsed_json: Vec<TwitterURLs> =
            serde_json::from_str(&query_data).expect("Failed to parse JSON");
        self.data = Some(parsed_json);
        self
    }

    fn build_compose_tweet_query(
        &self,
        in_reply_to_id: i32,
        reply_tweet_percent_encoded: String,
    ) -> Result<String> {
        let Some(json_data) = self.data.as_ref() else {
            return Err(Error::EmptyDomainNameError);
        };

        let data = &json_data[0];

        let base_url = &data.twitter_base_url;
        let path_compose_post = &data.compose_post.path;

        let base_url_compose_path = base_url.to_owned() + path_compose_post;

        let in_reply_to = "in_reply_to=".to_owned() + in_reply_to_id.to_string().as_str();
        let text = "text=".to_owned() + reply_tweet_percent_encoded.as_str();

        let query = in_reply_to + "&" + text.as_str();

        let url = base_url_compose_path + "?" + &query;

        Ok(url)
    }
}

// fn build_search_twitter_query(){}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_json_data() {
        let mut q_builder = TwitterURLQueriesBuilder::new();
        let query = q_builder
            .get_json_data()
            .build_compose_tweet_query(123, "reply_tweet_percent_encoded".to_string())
            .unwrap();

        println!("{:?}", query);
    }
}
