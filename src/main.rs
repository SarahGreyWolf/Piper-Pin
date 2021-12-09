use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::prelude::*;
use std::io;
use std::thread;
use piper::{FromReq, AsResp};
use thread_pool::ThreadPool;

mod piper;
mod thread_pool;

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

    let req = piper::Request::read(buffer.as_slice());
    let response = text_response!("Hello World!\n{:?}", req);
    //let response = piper::TextResponse(format!("Hello World\n{:?}", req));
    stream.write(&response.bytes()).unwrap();
    stream.flush().unwrap();
    println!("{:?}", req);
}
