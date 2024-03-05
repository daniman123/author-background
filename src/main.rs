mod build_tweet;
mod build_tweet_data;
mod error;
mod parse_html;
mod prelude;
mod prepare_tweet_data;
mod twitter_query_builder;
mod utils;

use crate::prepare_tweet_data::get_message::get_tweet_message;
use crate::prepare_tweet_data::{get_domain::get_web_domain, get_organisation::get_org_name};
use crate::{parse_html::parse_html_document, prepare_tweet_data::get_authors::get_author_names};
use build_tweet::build_tweet::build_tweet;
use build_tweet_data::tweet_data_builder::TweetDataBuilder;
use utils::requests::fetch_response_text;

#[tokio::main]
async fn main() -> core::result::Result<(), error::Error> {
    // let url = "https://www.nytimes.com/2024/03/01/us/oregon-drug-decriminalization-rollback-measure-110.html";
    // let url = "https://edition.cnn.com/2024/03/02/middleeast/israel-gaza-war-palestinian-parents-children-intl-cmd/index.html";
    // let url = "https://apnews.com/article/israel-hamas-war-news-03-01-2024-86ab114fc0036d0b4fa5a69ed21964c6?taid=65e2003a99a0eb0001fe99ea&utm_campaign=TrueAnthem&utm_medium=AP&utm_source=Twitter";
    // let url= "https://news.sky.com/story/us-defence-seceretarys-comments-suggest-israel-is-nowhere-close-to-destroying-hamas-13084035";
    // let url = "https://www.theguardian.com/world/2024/mar/02/iran-election-turnout-drops-to-41-as-reformists-criticise-poll";
    // let url = "https://www.bbc.com/news/world-middle-east-68434443?xtor=AL-72-%5Bpartner%5D-%5Bbbc.news.twitter%5D-%5Bheadline%5D-%5Bnews%5D-%5Bbizdev%5D-%5Bisapi%5D&at_campaign_type=owned&at_format=link&at_link_type=web_link&at_ptr_name=twitter&at_link_id=AB665D86-D706-11EE-9BE1-1F884B3AC5C4&at_bbc_team=editorial&at_campaign=Social_Flow&at_link_origin=BBCWorld&at_medium=social";

    let url = "https://edition.cnn.com/2024/03/04/europe/un-team-sexual-abuse-oct-7-hostages-intl/index.html";

    let domain = get_web_domain(url).unwrap();
    let organisation_name = get_org_name(domain.clone()).unwrap_or_default();

    let response = fetch_response_text(url).await?;

    let author_vec = parse_html_document(response, domain.clone()).unwrap();

    let authors = get_author_names(author_vec);

    let message = get_tweet_message();

    let tweet_data = TweetDataBuilder::new()
        .organisation(organisation_name)
        .message(message)
        .authors(authors)
        .build()
        .unwrap();

    // println!("{:?}", tweet_data);
    build_tweet(tweet_data);

    Ok(())
}
