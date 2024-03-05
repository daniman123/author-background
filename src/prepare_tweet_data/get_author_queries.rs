pub fn queryrize_author_name(author_name: String) -> String {
    let author_name_split: Vec<&str> = author_name.as_str().split_whitespace().collect();

    let author_name_query = author_name_split.join("%20");

    let twitter_search_query = format!(
        "https://twitter.com/search?q={}&f=user",
        author_name_query
    );

    twitter_search_query
}
