use scraper::error::SelectorErrorKind;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("domain name is not set")]
    EmptyDomainNameError,
    #[error("Tweet message is not set")]
    EmptyMessageError,
    #[error("selector was not found in dom")]
    SelectorNotInDomError,
    #[error("selector is valid/found but does not contain valid author name")]
    EmptyAuthorNameError,
    #[error("selector is invalid")]
    SelectorError(#[from] SelectorErrorKind<'static>),
    #[error("could not parse url")]
    ParseError(#[from] url::ParseError),
}
