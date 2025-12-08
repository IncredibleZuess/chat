use std::{io::{self, Write}, net::TcpStream};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message<'a> {
    by: &'a str,
    message_type: usize,
    message: &'a str,
    message_length: usize,
}

impl <'a>Message<'a> {
    fn new(user: &'a str, m_type: usize, content: &'a str, length: usize) -> Message<'a>{
        let msg = Message {
            by: user,
            message_type: m_type,
            message: content,
            message_length: length
        };
        msg
    }
}

fn main() -> std::io::Result<()>{
    let addr = "127.0.0.1:12345";
    
    let mut buf: Vec<u8> = Vec::new();
    let mut usr = String::new();
    let mut mes = String::new();

    io::stdout().write(b"Enter Username: ")?;
    io::stdout().flush()?;
    io::stdin().read_line(&mut usr).unwrap();

    io::stdout().write(b"Message: ")?;
    io::stdout().flush()?;
    io::stdin().read_line(&mut mes).unwrap();
    let message = Message::new(&usr, 0, &mes, mes.len());
    buf.write_all(serde_json::to_string(&message).unwrap().as_bytes())?;
    let mut stream = TcpStream::connect(addr)?;
    stream.write(&buf)?;
    Ok(())
}