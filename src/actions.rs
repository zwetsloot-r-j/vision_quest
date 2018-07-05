use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::net::TcpStream;
use tcp::receive;

pub enum Message {
    Empty,
    Raw((String, Sender<Action>)),
    Client((Arc<Mutex<TcpStream>>, Sender<Action>)),
}

impl Message {
    pub fn expect_raw(self) -> (String, Sender<Action>) {
        match self {
            Message::Raw(content) => Ok(content),
            _ => Err("Expected Message::Raw"),
        }.unwrap()
    }

    pub fn expect_client(self) -> (Arc<Mutex<TcpStream>>, Sender<Action>) {
        match self {
            Message::Client(content) => Ok(content),
            _ => Err("Expected Message::Client"),
        }.unwrap()
    }
}

pub struct Action {
    pub domain: String,
    pub invocation: String,
    pub message: Message,
    pub sender: String
}

pub fn run(action : Action) -> Result<(), Error> {
    println!("domain: {}", action.domain);
    println!("invocation: {}", action.invocation);
    // TODO make formatter println!("message: {:?}", action.message);
    println!("sender: {}", action.sender);

    match (action.domain.as_str(), action.invocation.as_str()) {
        ("client", "add") => {
            println!("matched new client");
            let (socket, tx) = action.message.expect_client();
            receive(socket, tx, action.sender.clone())
        },
        ("client", "receive") => {
            let (content, _tx) = action.message.expect_raw();
            println!("{}", content);
            Ok(())
        },
        _ => {
            Err(Error::new(ErrorKind::InvalidData, "domain and invocation not matched"))
        },
    }
}
