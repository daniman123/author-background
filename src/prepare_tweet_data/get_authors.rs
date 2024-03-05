use super::get_author_queries::queryrize_author_name;

pub fn get_author_names(string_vec: Vec<String>) -> Vec<String> {
    let authors = string_vec
        .iter()
        .map(|auth| set_author_name_tuple(auth))
        .collect();
    authors
}

fn set_author_name_tuple(author: &str) -> String {
    let author_cred = (
        author.to_string(),
        queryrize_author_name(author.to_string()),
    );
    let combined_string = format!("{} | {}", author_cred.0, author_cred.1);
    combined_string
}
