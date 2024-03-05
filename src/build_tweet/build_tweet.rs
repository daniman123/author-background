use crate::{
    build_tweet::format_tweet::format_message, build_tweet_data::tweet_data_builder::TweetData,
};

pub fn build_tweet(tweet_data: TweetData) {
    let TweetData {
        authors,
        message,
        organisation,
    } = tweet_data;

    // println!("tweet_data: {:?}", tweet_data);

    format_message(message, &organisation, authors);
}

#[cfg(test)]
mod tests {
    use crate::build_tweet_data::tweet_data_builder::TweetDataBuilder;

    use super::*;
    #[test]
    fn test_build_tweet() {
        let authors = vec!["yooo".to_string(), "brahhh".to_string()];

        let tweet_data = TweetDataBuilder::new()
            .organisation("Org".to_string())
            .message("Message {} {}".to_string())
            .authors(authors)
            .build()
            .unwrap();

        build_tweet(tweet_data);
    }
}
