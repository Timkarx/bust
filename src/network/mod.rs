use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

mod utils;

pub struct URL {
    scheme: String,
    host: String,
    path: String,
    port: u16,
}

pub trait Request {
    fn get(&self) -> std::io::Result<String>;
}

impl URL {
    pub fn new<S: Into<String>>(s: S) -> Self {
        utils::url_parse(s.into())
    }
}

impl Request for URL {
    fn get(&self) -> std::io::Result<String> {
        let addr = format!("{}:80", self.host);
        let mut stream = TcpStream::connect(addr)?;

        // Form and send the HTTP/1.0 GET request
        let request = format!(
            "GET {} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
            self.path, self.host
        );
        println!("{}", request);
        stream.write_all(request.as_bytes())?;

        // Read the response
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        stream
            .shutdown(Shutdown::Both)
            .expect("Shudown call failed");

        Ok(response)
    }
}
