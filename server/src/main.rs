use std::net::{TcpListener, TcpStream, IpAddr};
use std::io::{Result, Write, BufReader, BufRead};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::str;

// Clients are in groups and in a group, when a client sends a message all clients in the group
// recive the message
#[derive(Debug)]
struct Group {
    name: String,
    clients: Vec<TcpStream>,
}


impl Clone for Group {
    fn clone(&self) -> Self {
        let cloned_clients: Vec<TcpStream> = self.clients.iter()
            .map(|stream| {
                // Attempt to clone each TcpStream
                stream.try_clone().expect("Failed to clone TcpStream")
            })
            .collect();
        
        Group {
            name: self.name.clone(),
            clients: cloned_clients,
        }
    }
}


impl Group {
    fn new(name: String) -> Self {
        Group {
            name,
            clients: vec![],
        }
    }

    fn add_client(&mut self, client: TcpStream) {
        self.clients.push(client);
    }
    
    // sends a message to all clients in a group
    fn broadcast(&self, message: Message) {
        for mut stream in &self.clients {
            writeln!(stream, "{:?}", message);
        }
    }
}

#[derive(Debug)]
struct Message {
    message: String,
    username: String,
}

fn server_handler(recevier: Receiver<Message>, groups: Arc<Mutex<Vec<Group>>>) -> Result<()> {
    loop {
        let groups = groups.lock().unwrap(); 
        
        for mut client in &groups[0].clients {
            writeln!(&mut client, "{:?}", recevier.recv());
        }
    }
    Ok(())
}

fn connection_handler(mut stream: TcpStream, sender: Sender<Message>, groups: Arc<Mutex<Vec<Group>>>) -> Result<()> {
    writeln!(stream, "Welcome to the server!").map_err(|e|
        eprintln!("S_ ERROR: {}", e)
    ); 

    let mut groups = groups.lock().unwrap();
    groups[0].add_client(stream.try_clone()?);

    loop {
        let mut reader = BufReader::new(&mut stream);

        for data in reader.lines() {
            sender.send(Message {
                message: data.unwrap().to_string(),
                username: "User_1".to_string(),
            });
            // println!("{}", data.unwrap());
        } 
    }
}

fn main() -> Result<()> {
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr)?;

    println!("S_ INFO: Bind listener to: {}", addr);

    let mut groups = Arc::new(Mutex::new(vec![]));

    let mut test_group = Group::new("test".to_string());

    groups.lock().unwrap().push(test_group);

    let (sender, receiver) = channel();
    thread::spawn(move || {
        server_handler(receiver, groups.clone());
    });
fn main() -> Result<()> {
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr)?;

    println!("S_ INFO: Bind listener to: {}", addr);

    let mut groups = Arc::new(Mutex::new(vec![]));

    let mut test_group = Group::new("test".to_string());

    groups.lock().unwrap().push(test_group);

    let (sender, receiver) = channel();
    thread::spawn(move || {
        server_handler(receiver, groups.clone());
    });

    // handles incoming connection trys by unknown clients
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("S_ INFO: {} connected", stream.peer_addr()?);
                let sender = sender.clone();
                let groups = groups.clone();
                thread::spawn(move || {
                    connection_handler(stream, sender, groups);
                });
            },
            Err(e) => {
                eprintln!("S_ ERROR: {}", e);
            }
        }
    }

    Ok(())
}
    // handles incoming connection trys by unknown clients
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("S_ INFO: {} connected", stream.peer_addr()?);
                let sender = sender.clone();
                let groups = groups.clone();
                thread::spawn(move || {
                    connection_handler(stream, sender, groups);
                });
            },
            Err(e) => {
                eprintln!("S_ ERROR: {}", e);
            }
        }
    }

    Ok(())
}
