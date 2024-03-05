use crate::prelude::*;
use crate::utils::shared::domain_handler::get_web_domain;

#[derive(Debug)]
pub struct TweetUrlData {
    pub tweet_id: i32,
    pub linked_article_url: Option<String>,
}

#[derive(Debug, Default)]
pub struct TweetUrlDataBuilder {
    pub tweet_id: Option<i32>,
    pub linked_article_url: Option<String>,
}

impl TweetUrlDataBuilder {
    pub fn new() -> Self {
        TweetUrlDataBuilder::default()
    }

    pub fn tweet_id(&mut self, tweet_id: impl Into<i32>) -> &mut Self {
        self.tweet_id = Some(tweet_id.into());
        self
    }

    pub fn linked_article_url(&mut self, linked_article_url: impl Into<String>) -> &mut Self {
        self.linked_article_url = Some(linked_article_url.into());
        self
    }

    pub fn build(&self) -> Result<TweetUrlData> {
        let Some(tweet_id) = self.tweet_id else {
            return Err(Error::EmptyDomainNameError);
        };

        Ok(TweetUrlData {
            tweet_id,
            linked_article_url: self.linked_article_url.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_constructor() {
        let tweet_id = 123;
        let linked_article_url = "yo";

        let mut t_builder = TweetUrlDataBuilder::new();
        let t_url_data = t_builder
            .tweet_id(tweet_id)
            .linked_article_url(linked_article_url)
            .build();

        println!("{:?}", t_url_data.unwrap());
    }
}
