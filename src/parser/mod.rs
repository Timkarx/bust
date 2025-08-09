use std::collections::HashMap;

pub struct HTMLElement {
    name: String,
    prev: Option<Box<HTMLElement>>,
    children: Vec<Box<HTMLElement>>,
}

pub fn parse_html(bytes: &[u8]) {
    let mut stack = Vec::<String>::new();
    let mut buf = Vec::new();
    let mut i = 0;
    let n = bytes.len();

    while i < n {
        // Find the first tag opening <
        if bytes[i] != b'<' {
            while i < n && bytes[i] != b'<' {
                i += 1;
            }
        }
        // Add everything inside tag to buff until > char
        // TODO: Add logic for self closing tags
        while i < n && bytes[i] != b'>' {
            buf.push(bytes[i]);
            i += 1;
        }

        if i == n { break }

        if bytes[i] == b'>' {
            buf.push(bytes[i]);
            let Ok(tag_name) = String::from_utf8(buf.clone()) else {
                panic!("Oopsie")
            };
            stack.push(tag_name);
            buf.clear();
        }

        // Now look for closing tag
    }

    println!("{:#?}", stack);
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
            return headers;
        }
        buf.push(char)
    }
    headers
}
