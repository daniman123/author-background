use crate::prelude::*;
use url::Url;

pub fn get_web_domain(url: &str) -> Result<String> {
    let parsed_url = Url::parse(url)?;
    let domain = parsed_url
        .domain()
        .ok_or_else(|| url::ParseError::EmptyHost)?;

    Ok(domain.to_string())
}
