use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use std::process;
use std::thread;


pub fn start(address: &str) {
    thread::spawn(|| { // TODO improve
        let mut client = super::client::new("127.0.0.1:19005").unwrap();
        client.action_loop();
    });
}

// A wrapper for a TCP stream.
pub struct Client {
    stream: TcpStream,
}

impl Client {

    pub fn get_stream<'a>(&'a mut self) -> &'a TcpStream {
        &self.stream
    }

    pub fn action_loop(&mut self) {
        println!("client mode");
        loop {
            let input = super::read_console();
            self.action(input.as_str());
        }
    }

    fn action(&mut self, input: &str) -> Result<(), io::Error> {
        match input {
            // general purpose commands
            "quit" => process::exit(0),
            "q" => process::exit(0),
            // Client specific commands
            // Retrieves a specific file
            "get" => self.easy_get_remove_later(),
            // Asks the server what files are available
            "ask" => self.stream.write_all(b"ask"),
            _ => {
                println!("'{}' command not found for client", input);
                Ok(())
            },

        }
    }

    // TODO remove later. don't be lazy.
    fn easy_get_remove_later(&mut self) -> Result<(), io::Error> {
        let path = "get examples/files/foo.txt"; // TODO user sent information
        println!("client: i want '{}'", path);
        super::get(&mut self.stream, path)?;
        Ok(())
    }

}

pub fn new(address: &str) -> Result<Client, io::Error> {
    let mut stream = super::connect(address)?;
    Ok(Client {stream})
}