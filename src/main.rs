use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    raw_response().await?;
    Ok(())
}

async fn raw_response() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://edition.cnn.com/2024/03/02/middleeast/israel-gaza-war-palestinian-parents-children-intl-cmd/index.html").await?.text().await?;
    // let html =
    parse_html(resp);
    // println!("{:#?}", html);
    Ok(())
}

fn parse_html(html: String) {
    let fragment = Html::parse_document(&html);
    let selector = Selector::parse("div.byline__names span").unwrap();
    for element in fragment.select(&selector) {
        println!("{:?}", element.inner_html());
    }
}
