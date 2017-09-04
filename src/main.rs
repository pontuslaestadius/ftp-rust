
pub mod server;


use server::*;
use std::thread;
use std::net::TcpStream;
use std::io::prelude::*;



fn main() {

    // Handles the backend server.

    println!("Starting server thread.");
    //start_server();

    println!("Connecting client.");
    let address = "127.0.0.1:9005";
    let mut stream = match ftp::connect(address) {
        Ok(t)  => t,
        Err(e) => {
            eprintln!("caught an error while connecting, {}", e);
            panic!("caught an error while connecting");
        },
    };
    println!("Connected");


    let path = "examples/receive/foo.txt";
    //let mut buf = ftp::get_buffer(path).unwrap();
    ftp::receive(&mut stream, path);

    //server_thread.join();
}


fn start_server() {
    let mut server = server::Server::new("127.0.0.1", "9005").unwrap();

    println!("Server up.");

    server.start();
}

pub mod ftp {

    use std::net::TcpStream;
    use std::io::prelude::*;
    use std::io;
    use std::fs::{File, OpenOptions};

    use std::thread;

    // Connects through TCP and return the stream.
    pub fn connect(address: &str) -> Result<TcpStream, io::Error> { //TODO
        let mut stream = TcpStream::connect(address)?;
        Ok(stream)
    }

    // Returns a Buffer with the content of the file.
    pub fn get_buffer(path: &str) -> Result<Buffer, io::Error> {
        let mut f = OpenOptions::new()
            .read(true)
            .truncate(false)
            .open(path)?;

        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf)?;
        Ok(Buffer::new(buf))
    }

    // Writes all the content from the buffer to the stream.
    pub fn send(stream: &mut TcpStream, buf: &mut Buffer) -> Result<(), io::Error> {
        stream.write_all(buf.get())?;
        Ok(())
    }

    // A non-blocking recieve.
    pub fn receive(stream: &mut TcpStream, path: &str) -> Result<(), io::Error>{
        stream.write_all(path.as_bytes());
        let mut buf = Vec::new();
        let res = stream.read_to_end(&mut buf);
        let mut f = File::create(path); // TODO use a different name path.
        Ok(())
    }

    pub fn progress_bar(buf: Buffer) {
        // TODO (Optional)
    }

    pub struct Buffer {
        buf: Vec<u8>,
    }

    impl Buffer {
        pub fn new(buf: Vec<u8>) -> Buffer {
            Buffer {
                buf
            }
        }

        pub fn get(&mut self) -> &mut Vec<u8> {
            &mut self.buf
        }
    }
}