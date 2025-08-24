use super::*;
use std::fs;

#[test]
fn test_parser() {
    let html_doc = fs::read("test_data/html/doc_1.html").unwrap();
    let result = parse_html(html_doc.as_slice());
    println!("{:#?}", result);
}
