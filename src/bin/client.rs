use serde::{Deserialize, Serialize};
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    thread,
};

#[derive(Serialize, Deserialize, Debug)]
struct Message<'a> {
    by: &'a str,
    message_type: usize,
    message: &'a str,
    message_length: usize,
}

impl<'a> Message<'a> {
    fn new(user: &'a str, m_type: usize, content: &'a str, length: usize) -> Message<'a> {
        Message {
            by: user,
            message_type: m_type,
            message: content,
            message_length: length,
        }
    }
}

// Function to run in a separate thread: listens for incoming messages
fn listen_to_server(mut stream: TcpStream) {
    let mut buf = [0; 2048];
    loop {
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    println!("Server closed connection.");
                    break;
                }
                // Convert bytes to string and print
                // In a real app, you might deserialize the Message struct here
                let response = String::from_utf8_lossy(&buf[..n]);
                println!("\n[Server]: {}", response);
                print!("Message: "); // Re-prompt for UI consistency
                io::stdout().flush().unwrap();
            }
            Err(_) => break,
        }
    }
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:12345";
    let mut usr = String::new();

    io::stdout().write_all(b"Enter Username: ")?;
    io::stdout().flush()?;
    io::stdin().read_line(&mut usr).unwrap();
    let usr = usr.trim().to_string(); // Own the string

    // 1. Connect ONCE
    let mut stream = TcpStream::connect(addr).expect("Could not connect to address");

    // 2. Clone the stream for the listening thread
    let stream_clone = stream.try_clone().expect("Could not clone stream");

    // 3. Spawn the listener thread
    thread::spawn(move || {
        listen_to_server(stream_clone);
    });

    // 4. Main loop sends messages using the original stream
    loop {
        let mut mes = String::new();
        // Note: The listener thread might print over this prompt, dealing with that
        // usually requires a TUI library like 'ratatui' hint hint
        // For now, simple console I/O:
        io::stdout().write_all(b"Message: ")?;
        io::stdout().flush()?;

        io::stdin().read_line(&mut mes).unwrap();
        let mes = mes.trim();
        if mes == "quit" {
            break;
        }

        let message = Message::new(&usr, 0, mes, mes.len());

        // Write directly to the persistent stream
        let serialized = serde_json::to_string(&message).unwrap();
        stream.write_all(serialized.as_bytes())?;
        stream.flush()?;
    }
    Ok(())
}
