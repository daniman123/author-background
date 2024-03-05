pub fn get_org_name(domain: String) -> Option<String> {
    domain
        .find('.')
        .and_then(|first_dot_index| {
            domain
                .rfind('.')
                .map(|last_dot_index| (first_dot_index, last_dot_index))
        })
        .map(|(first_dot_index, last_dot_index)| {
            domain[first_dot_index + 1..last_dot_index].to_string()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_org_name() {
        let domain = "edition.cnn.com".to_string();
        let org_name = get_org_name(domain).unwrap_or_default();

        assert_eq!(org_name, "cnn")
    }
}
