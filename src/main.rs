pub mod ftp;

use std::{thread, time, env};
use std::io;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // Handle environment variables.
    if args.len() > 1 {
        while args.len() > 1 {
            let query = args.swap_remove(1);
            match query.as_str() {
                "server" => ftp::start_server(),
                "client" => ftp::start_client(),
                _ => eprintln!("unknown command '{}'", query)
            };

            // Sleep makes sure a client isn't created
            // before a server has time to bind the socket.
            let delay = time::Duration::from_millis(300);
            thread::sleep(delay);
        }
    }

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let len = input.len()-1;
        input.truncate(len);
        match input.as_str() {
            "quit" => break,
            "q" => break,
            "client" => ftp::start_client(),
            "server" => ftp::start_server(),
            _ => println!("'{}' command not found", input),
        };
    }

}