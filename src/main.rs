pub mod ftp;

use std::{thread, time, env};

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
            let delay = time::Duration::from_millis(500);
            thread::sleep(delay);
        }
    } else {
        ftp::start_client();
    }

}