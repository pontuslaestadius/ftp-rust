fn main() {
    let address = "127.0.0.1:34254";
    let stream = match ftp::connect(address) {
        Ok(t)  => t,
        Err(e) => {
            eprintln!("caught an error while connecting, {}", e);
            panic!("caught an error while connecting");
        },
    };
}

pub mod ftp {

    use std::io::prelude::*;
    use std::io;
    use std::net::TcpStream;
    use std::path::Path;
    use std::fs::OpenOptions;

    use std::fs::File;
    use std::fs::*;

    // Connects through TCP and return the stream.
    pub fn connect(address: &str) -> Result<TcpStream, io::Error> { //TODO
        let mut stream = TcpStream::connect(address)?;
        stream.write(&[1])?;
        stream.read(&mut [0; 128])?;
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

    pub fn send(stream: &mut TcpStream, buf: Buffer) {
        // TODO
    }

    pub fn progress_bar(buf: Buffer) {
        // TODO
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
    }
}