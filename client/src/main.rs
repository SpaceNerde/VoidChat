use std::io::{Write, Result};
use std::net::TcpStream;
use std::io::{self, BufRead, BufReader};
use std::thread;

fn main() -> Result<()>{
    match TcpStream::connect("127.0.0.1:8888") {
        Ok(mut stream) => {
            println!("Connected to 127.0.0.1:8888");

            let handle = stream.try_clone()?;
            let reader = BufReader::new(handle);

            // Spawn a thread to read from the stream and print received data
            thread::spawn(move || {
                for line in reader.lines() {
                    println!("{}", line.unwrap());
                }
            });

            // Read from stdin and send data over the stream
            loop {
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer)?;
                
                print!("\x1B[2K\r");

                stream.write_all(buffer.as_bytes())?;
            }        
        },
        Err(e) => {
            Err(e)
        },
    }
}

