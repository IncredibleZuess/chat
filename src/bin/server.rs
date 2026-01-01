use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    by: String,
    message_type: u8,
    message: String,
    message_length: u8,
}

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buf = [0; 2048];

    loop {
        // Read the message from the stream
        match stream.read(&mut buf) {
            Ok(size) => {
                // If size is 0, the client disconnected
                if size == 0 {
                    break;
                }

                // Parse the message
                let msg_str = match std::str::from_utf8(&buf[..size]) {
                    Ok(s) => s.trim(),
                    Err(_) => continue,
                };

                if let Ok(mes) = serde_json::from_str::<Message>(msg_str) {
                    println!("{}: {}", mes.by.trim(), mes.message.trim());

                    // BROADCAST: Lock the clients list and write to everyone
                    let mut streams = clients.lock().unwrap();
                    for client_stream in streams.iter_mut() {
                        // We ignore errors here (e.g., if a client disconnected)
                        let _ = client_stream.write_all(&buf[..size]);
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:12345";
    let listener = TcpListener::bind(addr)?;
    println!("Server is running on {:?}", addr);

    // A shared list of all connected clients
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected");

                // Clone the stream to add to our list
                let stream_clone = stream.try_clone().expect("Could not clone stream");

                // Add to list
                clients.lock().unwrap().push(stream_clone);

                // Clone the Arc to pass into the thread
                let clients_inner = Arc::clone(&clients);

                thread::spawn(move || {
                    handle_client(stream, clients_inner);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

