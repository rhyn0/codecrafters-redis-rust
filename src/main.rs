use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    stream.write_all("+PONG\r\n".as_bytes()).unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(_stream);
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
