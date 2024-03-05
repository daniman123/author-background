pub fn handle_bbc_byline(input: &str) -> Vec<String> {
    // Trim "By" and leading/trailing whitespaces
    let without_by = input.trim_start_matches("By").trim();

    // Define patterns to split the input string
    let patterns = ["&amp;", "and"];

    let names: Vec<String> = match patterns.iter().find(|&&p| without_by.contains(p)) {
        Some(&pattern) => without_by
            .split(pattern)
            .map(|s| s.trim().to_string())
            .collect(),
        None => vec![without_by.trim().to_string()], // If no pattern matches, use the whole string as the name
    };
    names
}
