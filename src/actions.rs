use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::net::TcpStream;
use std::collections::HashSet;
use tcp::receive;
use message_parser::parse;
use state::{State, Status, HistoryItem};

pub enum Message {
    Empty,
    Raw(String),
    Client(Arc<Mutex<TcpStream>>),
    HistoryItem(HistoryItem),
    Json(String),
    SelectAction((String, HashSet<usize>)),
}

impl Message {
    pub fn expect_raw(self) -> String {
        match self {
            Message::Raw(content) => Ok(content),
            _ => Err("Expected Message::Raw"),
        }.unwrap()
    }

    pub fn expect_client(self) -> Arc<Mutex<TcpStream>> {
        match self {
            Message::Client(content) => Ok(content),
            _ => Err("Expected Message::Client"),
        }.unwrap()
    }

    pub fn expect_json(self) -> String {
        match self {
            Message::Json(content) => Ok(content),
            _ => Err("Expected Message::Json"),
        }.unwrap()
    }

    pub fn expect_history_item(self) -> HistoryItem {
        match self {
            Message::HistoryItem(content) => Ok(content),
            _ => Err("Expected Message::HistoryItem"),
        }.unwrap()
    }

    pub fn expect_select_action(self) -> (String, HashSet<usize>) {
        match self {
            Message::SelectAction(content) => Ok(content),
            _ => Err("Expected Message::SelectAction"),
        }.unwrap()
    }
}

pub struct Action {
    pub domain: String,
    pub invocation: String,
    pub message: Message,
    pub sender: String
}

pub fn run(action: Action, mut state: State) -> Result<State, Error> {
    println!("domain: {}", action.domain);
    println!("invocation: {}", action.invocation);
    // TODO make formatter println!("message: {:?}", action.message);
    println!("sender: {}", action.sender);

    match (action.domain.as_str(), action.invocation.as_str()) {
        ("client", "add") => {
            let socket = action.message.expect_client();
            receive(socket, state.dispatcher.clone(), action.sender.clone())?;

            state.add_client(action.sender.clone());
            Ok(state)
        },
        ("client", "receive") => {
            let content = action.message.expect_raw();
            println!("RAW: {}", content);
            let action = parse(content, action.sender)?;
            state.dispatcher.send(action).expect("Failed to handle client action");

            Ok(state)
        },
        ("item", "add") => {
            let historyItem = action.message.expect_history_item();

            state.add_history_item(action.sender.clone(), historyItem);
            Ok(state)
        },
        ("application", "quit") => {
            state.status = Status::ShuttingDown;
            Ok(state)
        },
        ("action", "select") => {
            let (client_id, selections) = action.message.expect_select_action();

            state.clients
                .entry(client_id)
                .and_modify(|client| client.update_selections(selections))
                ;
            Ok(state)
        },
        ("ping", "pong") => {
            println!("pong");

            Ok(state)
        }
        _ => {
            Err(Error::new(ErrorKind::InvalidData, "domain and invocation not matched"))
        },
    }
}
