use std::collections::HashMap;

// Define a struct to represent domain and selector pairs
#[derive(Debug, Clone, Copy)]
pub struct DomainSelector {
    pub selector: &'static str,
}

// Static map of domain selectors
lazy_static::lazy_static! {
    static ref DOMAIN_SELECTORS: HashMap<&'static str, DomainSelector> = {
        let mut map = HashMap::new();
        map.insert("edition.cnn.com", DomainSelector { selector: "div.byline__names span" });
        map.insert("www.nytimes.com", DomainSelector { selector: "span.byline-prefix" });
        map.insert("www.apnews.com", DomainSelector { selector: "div.Page-authors span" });
        map.insert("news.sky.com", DomainSelector { selector: "span.sdc-article-author__name a" });
        map.insert("www.theguardian.com", DomainSelector { selector: "address > div.dcr-ygtsjm > a" });
        map.insert("www.bbc.com", DomainSelector { selector: "div.ssrcss-68pt20-Text-TextContributorName.e8mq1e96" });
        map
    };
}

// Function to get selector for a domain
pub fn get_selector(domain: &str) -> Option<&'static str> {
    DOMAIN_SELECTORS.get(domain).map(|ds| ds.selector)
}
