use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use std::process;
use std::thread;


pub fn start(address: &str) {
    thread::spawn(|| { // TODO improve
        let mut client = Client{address:"127.0.0.1:19005"};
        client.action_loop();
    });
}

// A wrapper for a TCP stream.
pub struct Client<'a> {
    address: &'a str,
}

impl<'b> Client<'b> {

    pub fn action_loop(&mut self) {
        loop {
            let input = super::read_console();
            self.action(input.as_str());
        }
    }

    fn action(&mut self, input: &str) -> Result<(), io::Error> {
        let (mut stream, client) = super::client::new(self.address)?;
        let mut command = input.split(' ');
        match command.next().unwrap() {
            // general purpose commands
            "quit" => process::exit(0),
            "q" => process::exit(0),
            // Client specific commands
            // Retrieves a specific file
            "get" => super::get(&mut stream, command.next().unwrap()),
            // Asks the server what files are available
            "ask" => stream.write_all(b"ask"),
            _ => {
                println!("'{}' command not found for client", input);
                Ok(())
            },
        }
    }

}

pub fn new(address: &str) -> Result<(TcpStream, Client), io::Error> {
    let mut stream = super::connect(address)?;
    Ok((stream, Client {address}))
}