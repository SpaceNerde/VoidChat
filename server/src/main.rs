use std::net::{TcpListener, TcpStream};
use std::io::{Result, Write, BufReader, BufRead};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::str;

struct Message;

fn server_handler(_recevier: Receiver<Message>) -> Result<()> {
    unimplemented!();
}

fn connection_handler(mut stream: TcpStream, _sender: Sender<Message>) -> Result<()> {
    writeln!(stream, "Welcome to the server!").map_err(|e|
        eprintln!("S_ ERROR: {}", e)
    ); 

    loop {
        let mut reader = BufReader::new(&stream);
        for data in reader.lines() {
            println!("{}", data.unwrap());
        } 
    }
}

fn main() -> Result<()> {
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr)?;

    println!("S_ INFO: Bind listener to: {}", addr);

    let (sender, receiver) = channel();
    thread::spawn(move || {
        server_handler(receiver);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("S_ INFO: {} connected", stream.peer_addr()?);
                let sender = sender.clone();
                thread::spawn(move || {
                    connection_handler(stream, sender);
                });
            },
            Err(e) => {
                eprintln!("S_ ERROR: {}", e);
            }
        }
    }

    Ok(())
}
