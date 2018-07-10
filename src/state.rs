use std::collections::HashMap;
use std::sync::mpsc::Sender;
use ::actions::Action;

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

impl Clone for HistoryAction {
    fn clone(&self) -> HistoryAction {
        HistoryAction {
            domain: self.domain.clone(),
            invocation: self.invocation.clone(),
            amplitude: self.amplitude.clone(),
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

impl Clone for HistoryState {
    fn clone(&self) -> HistoryState {
        HistoryState {
            id: self.id.clone(),
            content: self.content.clone(),
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

impl Clone for HistoryItem {
    fn clone(&self) -> HistoryItem {
        HistoryItem {
            action: self.action.clone(),
            state: self.state.clone(),
        }
    }
}

struct Client {
    history: Vec<HistoryItem>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            history: Vec::new(),
        }
    }

    pub fn push(&mut self, item: HistoryItem) {
        self.history.push(item);
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }
}

impl Clone for Client {
    fn clone(&self) -> Client {
        Client {
            history: self.history.iter().map(|x| x.clone()).collect(),
        }
    }
}

pub enum Status {
    Initializing,
    Running,
    Paused,
    ShuttingDown,
}

impl Clone for Status {
    fn clone(&self) -> Status {
        match self {
            Status::Initializing => Status::Initializing,
            Status::Running => Status::Running,
            Status::Paused => Status::Paused,
            Status::ShuttingDown => Status::ShuttingDown,
        }
    }
}

pub struct State {
    clients: HashMap<String, Client>,
    pub status: Status,
    pub dispatcher: Sender<Action>,
}

impl State {
    pub fn new(dispatcher: Sender<Action>) -> State {
        State {
            clients: HashMap::new(),
            status: Status::Initializing,
            dispatcher: dispatcher,
        }
    }

    pub fn add_client(&mut self, client: String) {
        self.clients
            .entry(client)
            .or_insert(Client::new())
            ;
    }

    pub fn add_history_item(&mut self, client: String, item: HistoryItem) {
        self.clients
            .entry(client)
            .and_modify(|client| client.push(item))
            ;
    }

    pub fn remove_client(&mut self, client: String) {
        self.clients
            .entry(client.clone())
            .and_modify(|client| client.clear())
            ;
        self.clients.remove(&client);
    }
}

impl Clone for State {
    fn clone(&self) -> State {
        State {
            clients: self.clients.clone(),
            status: self.status.clone(),
            dispatcher: self.dispatcher.clone(),
        }
    }
}
