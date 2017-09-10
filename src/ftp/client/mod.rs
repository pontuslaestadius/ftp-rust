use std::net::TcpStream;
use std::io;
use std::process;


// A wrapper for a TCP stream.
struct Client {
    stream: TcpStream,
}

impl Client {

    fn get_stream<'a>(&'a mut self) -> &'a TcpStream {
        self.stream
    }

    fn action_loop(&mut self) {
        println!("client mode");
        loop {
            let input = super::read_console();
            self.action(input);
        }
    }

    fn action(&mut self, input: &str) -> Result<(), io::Error> {
        match input {
            "quit" => process::exit(0),
            "q" => process::exit(0),
            // Client specific commands
            "ask" => super::send(stream, "ask"),
            _ => {
                println!("'{}' command not found for client", input);
                Ok(())
            },
        }
    }

}

pub fn new(address: &str) -> Result<Client, io::Error> {
    let mut stream = super::connect(address)?;
    Ok(Client {stream})
}