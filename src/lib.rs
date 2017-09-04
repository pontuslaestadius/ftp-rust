pub mod ftp {

    use std::io::prelude::*;
    use std::io;
    use std::net::TcpStream;
    use std::path::Path;

    pub fn connect(address: &str) -> Result<TcpStream, io::Error> { //TODO
        let mut stream = TcpStream::connect(address).unwrap();
        stream.write(&[1])?;
        stream.read(&mut [0; 128])?;
        Ok(stream)
    }

    pub fn get_buffer(path: &str) -> Buffer {
        //TODO

        Buffer {}
    }

    pub fn send(stream: &mut TcpStream, buf: Buffer) {
        // TODO
    }

    pub fn progress_bar(buf: Buffer) {
        // TODO
    }

    pub struct Buffer {
        // TODO
    }
}