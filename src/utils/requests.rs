use crate::prelude::*;
use reqwest::get;

// Define your utility function
pub async fn fetch_response_text(url: &str) -> Result<String> {
    // Make a GET request to the URL
    let response = get(url).await.unwrap();

    // Check if the request was successful (status code 2xx)
    if response.status().is_success() {
        // Read the response body as text and return it
        Ok(response.text().await.unwrap())
    } else {
        // If the request was not successful, return an error message
        Err(Error::EmptyDomainNameError)
    }
}