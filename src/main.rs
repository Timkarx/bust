mod network;
mod parser;
mod types;
mod utils;

use crate::network::Request;

fn main() {
    let url = network::URL::new("http://example.com");
    match url.get() {
        Ok(resp) => {
            println!("Response:\n{}", resp);
            let bytes = resp.as_bytes();
            parser::parse_html(bytes);
            let headers = parser::parse_response_headers(resp);
            println!("{:#?}", headers);

        }
        Err(e) => eprintln!("Error performing GET: {}", e),
    }
}
