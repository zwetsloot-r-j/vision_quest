use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;
use ::actions::Action;

pub struct HistoryAction {
    pub domain: String,
    pub invocation: String,
    pub amplitude: String,
    pub selected: bool,
}

impl HistoryAction {
    pub fn new(domain: String, invocation: String, amplitude: String) -> HistoryAction {
        HistoryAction {
            domain: domain,
            invocation: invocation,
            amplitude: amplitude,
            selected: false,
        }
    }
}

impl Clone for HistoryAction {
    fn clone(&self) -> HistoryAction {
        HistoryAction {
            domain: self.domain.clone(),
            invocation: self.invocation.clone(),
            amplitude: self.amplitude.clone(),
            selected: self.selected,
        }
    }
}   

pub struct HistoryState {
    pub id: String,
    pub content: String,
    pub selected: bool,
}

impl HistoryState {
    pub fn new(id: String, content: String) -> HistoryState {
        HistoryState {
            id: id,
            content: content,
            selected: false,
        }
    }
}

impl Clone for HistoryState {
    fn clone(&self) -> HistoryState {
        HistoryState {
            id: self.id.clone(),
            content: self.content.clone(),
            selected: self.selected,
        }
    }
}

pub struct HistoryItem {
    pub action: HistoryAction,
    pub state: HistoryState,
    pub selected: bool,
}

impl HistoryItem {
    pub fn new(action: HistoryAction, state: HistoryState) -> HistoryItem {
        HistoryItem {
            action: action,
            state: state,
            selected: false,
        }
    }

    pub fn select(&mut self) {
        self.action.selected = true;
        self.state.selected = true;
        self.selected = true;
    }

    pub fn deselect(&mut self) {
        self.action.selected = false;
        self.state.selected = false;
        self.selected = false;
    }
}

impl Clone for HistoryItem {
    fn clone(&self) -> HistoryItem {
        HistoryItem {
            action: self.action.clone(),
            state: self.state.clone(),
            selected: self.selected,
        }
    }
}

pub struct Client {
    pub id: String,
    pub history: Vec<HistoryItem>,
    pub selections: HashSet<usize>,
}

impl Client {
    pub fn new(id: String) -> Client {
        Client {
            id: id,
            history: Vec::new(),
            selections: HashSet::new(),
        }
    }

    pub fn push(&mut self, item: HistoryItem) {
        self.history.push(item);
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }

    pub fn update_selections(&mut self, selections: HashSet<usize>) {
        for i in self.history.iter_mut() {
            i.deselect();
        }
        for ref i in &selections {
            if let Some(item) = self.history.get_mut(**i) {
                item.select();
            }
        }
        self.selections = selections;
    }

    pub fn history_item_amount(&self) -> usize {
        self.history.len()
    }

    pub fn selected_history_item(&self) -> Option<&HistoryItem> {
        self.history.iter()
            .find(|item| (**item).selected)
    }
}

impl Clone for Client {
    fn clone(&self) -> Client {
        Client {
            id: self.id.clone(),
            history: self.history.iter().map(|x| x.clone()).collect(),
            selections: self.selections.clone(),
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
    pub clients: HashMap<String, Client>,
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
            .entry(client.clone())
            .or_insert(Client::new(client))
            ;
    }

    pub fn add_history_item(&mut self, client: String, item: HistoryItem) {
        self.clients
            .entry(client)
            .and_modify(|client| client.push(item))
            ;
    }

    pub fn total_history_item_amount(&self) -> usize {
        self.clients.iter()
            .map(|(_, ref client)| client.history_item_amount())
            .sum()
    }

    pub fn client_amount(&self) -> usize {
        self.clients.len()
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
