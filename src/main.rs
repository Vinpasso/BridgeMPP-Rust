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
                        let mut lock = local_state.write().unwrap();
                        lock.clients.push(client);
                    }

                    let br = BufReader::new(stream);

                    for line in br.lines() {
                        match line {
                            Ok(l) => {
                                println!("{}", l);
                                let lock = local_state.read();
                                let state = lock.unwrap();
                                for cl in &state.clients {
                                    /*
                                     * TODO: Solve this cleanly!
                                     * Right now, IO errors just panic here which is bad for 2 reasons:
                                     * a) As a panic, it's designed as an unrecoverable error which is not exactly what a dead client is.
                                     *    We shouldn't (have to) unwind the thread.
                                     * b) Note how we're holding the local_state lock: This poisons the lock and kills the entire server!
                                     */
                                    cl.write(&l).unwrap();
                                }
                            }
                            Err(_) => {
                                break; // RIP client
                            }
                        }
                    }
                    println!("Connection Closed");
                });
            }
            Err(_) => { /* connection failed */ }
        }
    }
}
