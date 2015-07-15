use std::net::{TcpListener};
use std::thread;
use std::io::{BufReader, BufRead, Write};
use std::sync::{Arc, RwLock};

mod client;
mod state;

use state::State;
use client::Client;

fn main() {
    //let mut vec = Vec::new();
    //vec.push(1);
    //vec.push(2);
    //vec.push(3);
    //println!("{:?}", &vec);
    //let mut state = State::new(vec);
    //let arc = Arc::new(RwLock::new(state));

    //{
    //    let copy = arc.clone();
    //    let state = copy.read().unwrap();
    //    for i in state.clients {
    //        println!("{}", i);
    //    }
    //}

    //arc.clone().write().unwrap().clients.push(0);

    //for i in arc.clone().read().unwrap().clients {
    //    println!("{}", i);
    //}

    let clients = Vec::new();
    let state = State::new(clients);
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
                        let mut lock = local_state.write();
                        let mut state = lock.as_mut().unwrap();
                        for mut cl in state.getClients() {
                            cl.write(&l);
                        }
                    }
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}
