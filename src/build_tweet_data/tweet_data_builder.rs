use crate::prelude::*;

#[derive(Debug)]
pub struct TweetData {
    pub organisation: String,
    pub message: String,
    pub authors: Vec<String>,
}

#[derive(Default)]
pub struct TweetDataBuilder {
    pub organisation: Option<String>,
    pub message: Option<String>,
    pub authors: Vec<String>,
}

impl TweetDataBuilder {
    pub fn new() -> Self {
        TweetDataBuilder::default()
    }

    pub fn organisation(&mut self, organisation: impl Into<String>) -> &mut Self {
        self.organisation = Some(organisation.into());
        self
    }
    pub fn message(&mut self, message: impl Into<String>) -> &mut Self {
        self.message = Some(message.into());
        self
    }
    pub fn authors(&mut self, authors: Vec<String>) -> &mut Self {
        for author_cred in authors {
            self.authors.push(author_cred);
        }
        self
    }

    pub fn build(&self) -> Result<TweetData> {
        let Some(organisation) = self.organisation.as_ref() else {
            return Err(Error::EmptyDomainNameError);
        };

        let Some(message) = self.message.as_ref().cloned() else {
            return Err(Error::EmptyMessageError);
        };

        Ok(TweetData {
            organisation: organisation.to_string(),
            authors: self.authors.clone(),
            message,
        })
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_build_tweet_data() {
//         let authors = vec![
//             ("Author1".to_string(), "Twitter1".to_string()),
//             ("Author2".to_string(), "Twitter2".to_string()),
//         ];

//         let tweet_data = TweetDataBuilder::new()
//             .organisation("Org".to_string())
//             .message("Message".to_string())
//             .authors(authors)
//             .build()
//             .unwrap();

//         println!("{:?}", tweet_data);

//         assert_eq!(tweet_data.organisation, "Org");
//         assert_eq!(tweet_data.message, "Message");
//         assert_eq!(tweet_data.authors.len(), 2);
//         assert_eq!(
//             tweet_data.authors[0],
//             ("Author1".to_string(), "Twitter1".to_string())
//         );
//         assert_eq!(
//             tweet_data.authors[1],
//             ("Author2".to_string(), "Twitter2".to_string())
//         );
//     }

//     #[test]
//     fn test_build_tweet_data_missing_organisation() {
//         let authors = vec![
//             ("Author1".to_string(), "Twitter1".to_string()),
//             ("Author2".to_string(), "Twitter2".to_string()),
//         ];

//         let result = TweetDataBuilder::new()
//             .message("Message".to_string())
//             .authors(authors)
//             .build();

//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_build_tweet_data_missing_message() {
//         let authors = vec![
//             ("Author1".to_string(), "Twitter1".to_string()),
//             ("Author2".to_string(), "Twitter2".to_string()),
//         ];

//         let result = TweetDataBuilder::new()
//             .organisation("Org".to_string())
//             .authors(authors)
//             .build();

//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_build_tweet_data_staggered() {
//         let authors = vec![
//             ("Author1".to_string(), "Twitter1".to_string()),
//             ("Author2".to_string(), "Twitter2".to_string()),
//         ];

//         let mut tweet_data_builder = TweetDataBuilder::new();

//         tweet_data_builder
//             .organisation("Org".to_string())
//             .message("Message".to_string());

//         tweet_data_builder.authors(authors);

//         let tweet_data = tweet_data_builder.build().unwrap();

//         println!("{:?}", tweet_data);

//         assert_eq!(tweet_data.organisation, "Org");
//         assert_eq!(tweet_data.message, "Message");
//         assert_eq!(tweet_data.authors.len(), 2);
//         assert_eq!(
//             tweet_data.authors[0],
//             ("Author1".to_string(), "Twitter1".to_string())
//         );
//         assert_eq!(
//             tweet_data.authors[1],
//             ("Author2".to_string(), "Twitter2".to_string())
//         );
//     }
// }
