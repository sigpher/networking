use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};
use std::{io, str};
pub mod models;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        stream
            .write(input.as_bytes())
            .expect("Failed to write buffer");

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Failed to read into buffer");

        println!("Response from server:{}", str::from_utf8(&buffer).unwrap());
    }
}

