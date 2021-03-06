pub mod ftp;

use std::{thread, time, env};
use std::process;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // Retreives any environmental variables ran with the program.
    if args.len() > 1 {
        while args.len() > 1 {
            let query = args.swap_remove(1);
            action(query.as_str());
        }

        // Retrieves any input the user writes in during run-time.
    } else {
        println!("general mode");
        let input = ftp::read_console();
        action(input.as_str());
    }

    loop {/* TODO */}
}

// Matches an input str with the correlating action.
fn action(input: &str) {
    match input {
        "quit" => process::exit(0),
        "q" => process::exit(0),
        "server" => ftp::start_server("127.0.0.1", "19005"),
        "wait"   => thread::sleep(time::Duration::from_millis(1000)), // Used for environmental commands.
        "client" => ftp::client::start("127.0.0.1:19005"),
        // Client specific commands
        _ => println!("'{}' command not found", input),
    };
}
