mod network;
mod utils;
mod types;

use crate::network::Request;

fn main() {
    let url = network::URL::new("http://example.com");
    match url.get() {
        Ok(resp) => println!("Response:\n{}", resp),
        Err(e) => eprintln!("Error performing GET: {}", e),
    }
}
