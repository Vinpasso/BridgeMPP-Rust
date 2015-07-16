use std::net::{TcpListener};
use std::thread;
use std::io::{BufReader, BufRead, Write};
use std::sync::{Arc, RwLock};

mod client;
mod state;

use state::State;
use client::Client;

fn main() {
    let state = State::new();
    let arc = Arc::new(RwLock::new(state));

    println!("Starting Server");
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    println!("Started");


    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New Connection");
                let client = Client::new(stream.try_clone().unwrap());
                let local_state = arc.clone();
                thread::spawn(move|| {
                    {
                        let mut lock = local_state.write();
                        lock.as_mut().unwrap().clients.push(client);
                    }

                    let br = BufReader::new(stream);

                    for line in br.lines() {
                        let l = line.unwrap();
                        println!("{}", l);
                        let lock = local_state.read();
                        let state = lock.unwrap();
                        for cl in &state.clients {
                            cl.write(&l);
                        }
                    }
                    println!("Connection Closed");
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
