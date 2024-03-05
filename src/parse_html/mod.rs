use scraper::{Html, Selector};

use crate::{prelude::*, prepare_tweet_data::get_author_bbc::handle_bbc_byline, utils::domain_selector::get_selector};

pub fn parse_html_document(response: String, domain: String) -> Result<Vec<String>> {
    let fragment = Html::parse_document(&response);

    let selector_from_domain = get_selector(&domain).unwrap();
    let selector = Selector::parse(&selector_from_domain)?;

    let element_iter = fragment.select(&selector);

    let mut peek_ele_iter = element_iter.clone().peekable();

    if peek_ele_iter.peek().is_none() {
        println!("NO AUTHORS FOUND");
        return Err(Error::SelectorNotInDomError);
    };

    let mut vec: Vec<String> = vec![];

    for element in element_iter {
        let author_name = element.inner_html();
        if author_name.is_empty() {
            return Err(Error::EmptyAuthorNameError);
        }
        vec.push(author_name)
    }

    let bbc = "www.bbc.com".to_owned();

    let author_vec = match domain.as_str() {
        d if d == bbc => handle_bbc_byline(vec[0].as_str()),
        _ => vec,
    };

    Ok(author_vec)
}
