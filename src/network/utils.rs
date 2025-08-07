pub fn url_parse(url: String) -> super::URL {
    let url = url.trim();
    let without_scheme = url
        .trim_start_matches("http://")
        .trim_start_matches("https://");

    let path_parts: Vec<&str> = without_scheme.splitn(2, '/').collect();

    let host = path_parts[0];
    let path = path_parts
        .get(1)
        .copied()
        .map(|s| if s.is_empty() { "/" } else { s })
        .unwrap_or("/");

    println!("PATH: {}", path);

    super::URL {
        scheme: "https://".into(),
        host: host.into(),
        path: path.into(),
        port: 80,
    }
}
