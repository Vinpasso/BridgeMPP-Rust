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

    pub fn get_clients_mut(&mut self) -> &mut Vec<Client> {
        return &mut self.clients;
    }
}
