use client::Client;

pub struct State {
    pub clients: Vec<Client>
}

impl State {
    pub fn new() -> State {
        State {
            clients: Vec::new(),
        }
    }
}
