use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

pub mod models;
use models::user::user::User;

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec = Vec::new();
    for stream in listener.incoming() {
        let stream = stream.expect("failed #1");
        let handle = thread::spawn(move || {
            handle_connection(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let user: User = serde_json::from_slice(&buf[..bytes_read]).unwrap();
        println!("{:?}", user);
        stream.write(&buf[..bytes_read])?;
    }
}
