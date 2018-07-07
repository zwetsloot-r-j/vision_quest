use std::collections::HashMap;

pub struct HistoryAction {
    domain: String,
    invocation: String,
    amplitude: String,
}

impl HistoryAction {
    pub fn new(domain: String, invocation: String, amplitude: String) -> HistoryAction {
        HistoryAction {
            domain: domain,
            invocation: invocation,
            amplitude: amplitude,
        }
    }
}

pub struct HistoryState {
    id: String,
    content: String,
}

impl HistoryState {
    pub fn new(id: String, content: String) -> HistoryState {
        HistoryState {
            id: id,
            content: content,
        }
    }
}

pub struct HistoryItem {
    action: HistoryAction,
    state: HistoryState,
}

impl HistoryItem {
    pub fn new(action: HistoryAction, state: HistoryState) -> HistoryItem {
        HistoryItem {
            action: action,
            state: state,
        }
    }
}

struct Client {
    history: Vec<HistoryItem>,
    observers: HashMap<String, fn(&Client)>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            history: Vec::new(),
            observers: HashMap::new(),
        }
    }

    pub fn push(&mut self, item: HistoryItem) {
        self.history.push(item);
        self.proc();
    }

    pub fn clear(&mut self) {
        self.history.clear();
        self.proc();
    }

    pub fn observe(&mut self, id: String, react: fn(&Client)) {
        self.observers.insert(id, react);
    }

    pub fn unsubscribe(&mut self, id: &String) {
        self.observers.remove(id);
    }

    fn proc(&self) {
        for (_, observer) in self.observers.iter() {
            observer(&self);
        };
    }
}

pub struct State {
    clients: HashMap<String, Client>,
    observers: HashMap<String, fn(&State)>,
}

impl State {
    pub fn new() -> State {
        State {
            clients: HashMap::new(),
            observers: HashMap::new(),
        }
    }

    pub fn add_client(&mut self, client: String) {
        self.clients
            .entry(client)
            .or_insert(Client::new())
            ;
        self.proc();
    }

    pub fn add_history_item(&mut self, client: String, item: HistoryItem) {
        self.clients
            .entry(client)
            .and_modify(|client| client.push(item))
            ;
        self.proc();
    }

    pub fn remove_client(&mut self, client: String) {
        self.clients
            .entry(client.clone())
            .and_modify(|client| client.clear())
            ;
        self.clients.remove(&client);
    }

    pub fn observe(&mut self, id: String, react: fn(&State)) {
        self.observers.insert(id, react);
    }

    pub fn unsubscribe(&mut self, id: &String) {
        self.observers.remove(id);
    }

    pub fn observe_client(&mut self, client: String, id: String, react: fn(&Client)) {
        self.clients
            .entry(client)
            .and_modify(|client| client.observe(id, react))
            ;
    }

    pub fn unsubscribe_client(&mut self, client: String, id: &String) {
        self.clients
            .entry(client)
            .and_modify(|client| client.unsubscribe(id))
            ;
    }

    fn proc(&self) {
        for (_, observer) in self.observers.iter() {
            observer(&self);
        };
    }
}
