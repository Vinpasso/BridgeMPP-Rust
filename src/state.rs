use client::Client;

pub struct State {
    pub clients: Vec<Client>
}

impl State {
    pub fn new(clients: Vec<Client>) -> State {
        State {
            clients: clients,
        }
    }

    pub fn getClients(&mut self) -> &mut Vec<Client> {
        return &mut self.clients;
    }
}
