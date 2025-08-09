use std::collections::HashMap;

pub struct HTMLElement {
    name: String,
    prev: Option<Box<HTMLElement>>,
    children: Vec<Box<HTMLElement>>,
}

pub fn parse_html(response: String) {
    let chars = response.chars();
    // Firstly we parse the headers into a data struct: dict?
    // Secondly we parse the html into a custom data struct
}

pub fn parse_response_headers(response: String) -> HashMap<String, String> {
    let mut headers = HashMap::<String, String>::new();
    let mut buf = String::new();

    let chars = response.chars();

    for char in chars {
        if char == '\n' {
            let header: Vec<&str> = buf.trim().split(':').collect();
            if header.len() < 2 {
                buf.clear();
            } else {
                let h_key = header[0];
                let h_val = header[1];
                headers.insert(h_key.into(), h_val.trim().into());
                buf.clear();
            }
            continue;
        }
        if char == '<' {
            return headers
        }
        buf.push(char)
    }
    headers
}
