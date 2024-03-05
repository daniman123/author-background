is_domain_twitter:

Requirements:
fn should take a ["url"&str] as input, and return struct TweetUrlData.

Fn Signature:
pub fn extract_tweet_url_data(url: &str) -> Result<TweetUrlData>

Functionality:
Arg, "url" of type &str, is passed and checked for validity by fn "get_domain_name".
"url", has id extracted from string - stored as "tweet_id"(i32).
Get-request using "url", to get tweet content.
Tweet content is checked for link to article - link is stored as "linked_article_url"(String).
"tweet_id" and "linked_article_url" passed to struct TweetUrlData.
