use crate::prelude::*;

pub fn get_web_domain(url: &str) -> Result<String> {
    let parsed = url::Url::parse(url);
    if !parsed.is_ok() {
        return Err(Error::InvalidUrl);
    }
    let parsed_url = parsed?;
    let domain: String = parsed_url.domain().unwrap().to_string();

    Ok(domain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_web_domain_valid_url() {
        let domain = get_web_domain("https://example.com");
        assert!(domain.is_ok());
        let domain_ok = domain.unwrap();
        assert_eq!(domain_ok, "example.com");
    }

    #[test]
    fn test_get_web_domain_invalid_url() {
        let domain = get_web_domain("invalidUrl");
        assert!(domain.is_err());
    }
}
