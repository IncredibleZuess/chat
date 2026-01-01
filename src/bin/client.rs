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

fn send_message(address: &str, message: Message) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address).expect("Could not connect to address");
    let mut buf: Vec<u8> = Vec::new();
    buf.write_all(serde_json::to_string(&message).unwrap().as_bytes())?;
    stream.write_all(&buf).expect("Could not write to buffer");
    Ok(())
}

fn listen_server() {
    println!("Listening for server responses");
    let address = "127.0.0.1:12345";
    let mut stream = TcpStream::connect(address).expect("Could not connect to address");
    loop {
        let mut buf: [u8; 2048] = [0; 2048];
        let dec_stream = stream.read(&mut buf).expect("Could not read stream");
        println!("{:?}", &buf[..dec_stream]);
    }
}

fn main() -> std::io::Result<()> {
    //Construct all the variables to be used for input
    let addr = "127.0.0.1:12345";
    let mut usr = String::new();

    io::stdout().write_all(b"Enter Username: ")?;
    io::stdout().flush()?;
    io::stdin().read_line(&mut usr).unwrap();
    //First We tell the server a user has joined
    //So we construct a new message to do this
    //TODO get a better way of doing this
    thread::spawn(listen_server);
    loop {
        let mut mes = String::new();
        io::stdout().write_all(b"Message: ")?;
        io::stdout().flush()?;
        io::stdin().read_line(&mut mes).unwrap();
        if mes.trim() == "quit" {
            break;
        }
        let message = Message::new(&usr, 0, &mes, mes.len());
        send_message(addr, message)?;
    }
    Ok(())
}
