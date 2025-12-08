use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use serde::{Deserialize, Serialize};
#[derive(Serialize,Deserialize,Debug)]
struct Message {
    by: String,
    message_type: u8,
    message: String,
    message_length: u8,
}

fn handle_client(mut stream: TcpStream){
    //Start with a large buffer size that can handle wierd messages TODO actually test this
    let mut buf: [u8; 2048] = [0; 2048];
    //First get the size of the incoming message
    let dec_mes = stream.read(&mut buf).unwrap();
    //Then use this size to set the amount of the buffer to read
    let mes: Message = serde_json::from_str(str::from_utf8(&buf[..dec_mes]).unwrap()).unwrap();
    //Note Debug print
    println!("Message Content: {:?}", mes);
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:12345";

    let listener: std::io::Result<TcpListener> = match TcpListener::bind(addr){
        Ok(listener) => {println!("Server is running on {:?}", addr); Ok(listener)},
        Err(e) =>{ eprintln!("An error has occured: {}", e); Err(e)},
    };
    

    for stream in listener?.incoming(){
        handle_client(stream?);
    }
    Ok(())

}
