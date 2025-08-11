use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

static SELF_CLOSING_TAGS: [&str; 14] = [
    "area",
    "base",
    "br",
    "col",
    "embed",
    "hr",
    "img",
    "input",
    "link",
    "meta",
    "param",
    "source",
    "track",
    "wbr",
];

pub trait Parser {}

#[derive(Debug)]
pub struct HTMLElement {
    name: String,
    children: Vec<Rc<RefCell<HTMLElement>>>,
    parent: Option<Weak<RefCell<HTMLElement>>>,
}

pub struct HTMLTextElement(String);

pub fn parse_html(bytes: &[u8]) {
    let mut stack = Vec::<Rc<RefCell<HTMLElement>>>::new();
    let mut buf = Vec::new();
    let mut i = 0;
    let n = bytes.len();

    while i < n {
        // Find the first tag opening <
        if bytes[i] == b'<' {
            if !buf.is_empty() {
                //Add text
            }
            buf.clear();
        }
        // Add everything inside tag to buff until > char
        // TODO: Add logic for self closing tags
        else if bytes[i] == b'>' {
            let Ok(tag_name) = String::from_utf8(buf.clone()) else {
                panic!("Oopsie")
            };
            add_tag(tag_name, &mut stack);
            buf.clear();
        } else {
            buf.push(bytes[i]);
            i += 1;
        }
    }

    println!("{:#?}", stack);
}

fn parser_result(stack: Vec<Rc<RefCell<HTMLElement>>>) {}

fn add_tag(tag: String, stack: &mut Vec<Rc<RefCell<HTMLElement>>>) {
    if tag.is_empty() {
        return;
    }
    let tag_as_bytes = tag.as_bytes();
    // If tag is closing
    if tag_as_bytes[0] == b'/' {
        if stack.is_empty() {
            return;
        }
        let node = stack.pop().unwrap();
        let parent = stack.last().unwrap();
        parent.borrow_mut().children.push(node);
    } else if {

    } else {
        // Stack is a vec of ref counted pointers to HTMLElement
        // We want parent to be either a pointer to an HTMLElement, or construct a None
        let parent = stack.last().map(|x| Rc::downgrade(x));
        let node = Rc::new(RefCell::new(HTMLElement {
            name: tag,
            children: Vec::new(),
            parent,
        }));
        stack.push(node);
    }
    // TODO: handle self-closing tags
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
