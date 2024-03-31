use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // 1024 is the size of the buffer
    let mut buffer = [0; 1024];

    // reads the client data and stores it in buffer
    stream.read(&mut buffer)
        .expect("Failed to read from client!");

    // Converts the data in buffer to utf8 string
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    // Create a response and is in type bytes
    let response = "Hello, Client!".as_bytes();
    stream.write(response)
        .expect("Failed to respond to Client!");
}

fn main(){
    // Bind TCP listener to our handler
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8000");

    // Loop over incoming connections
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}