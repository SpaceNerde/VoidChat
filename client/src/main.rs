use std::io::{Write, Result};
use std::net::TcpStream;
use std::io::{self, BufRead};

fn main() -> Result<()>{
    match TcpStream::connect("127.0.0.1:8888") {
        Ok(mut stream) => {
            println!("Connected to 127.0.0.1:8888");
            
            loop {
                let mut buffer = String::new();
                let mut handle = io::stdin().lock();
                handle.read_line(&mut buffer)?;
                
                stream.write(&mut buffer.as_bytes())?;
            }
        },
        Err(e) => {
            Err(e)
        },
    }
}
