pub fn make_url(url_query: String) -> String {
    let query = format_query(url_query);
    return format!("http://openlibrary.org/search.json?q={}", query); //     the+lord+of+the+rings
}

fn format_query(query: String) -> String {
    return query.to_lowercase().replace(" ", "+");
}

#[cfg(test)]
mod tests {

    use super::make_url;
    #[test]
    fn format_query() {
        let result = make_url(String::from("Le petit poney"));
        assert_eq!(
            result,
            "http://openlibrary.org/search.json?q=le+petit+poney"
        );
    }
}
