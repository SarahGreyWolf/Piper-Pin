use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::prelude::*;
use std::io;
use std::thread;
use piper::{FromReq, AsResp};

mod piper;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:60")?;
    for stream in listener.incoming() {
        thread::spawn(move ||{
            handle_client(stream.unwrap());
        });
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = piper::TextResponse(format!("Hello World\nRequest: {:?}", req));
    let req = piper::Request::read(buffer.as_slice());
    stream.write(&response.bytes()).unwrap();
    stream.flush().unwrap();
    println!("{:?}", req);
}
