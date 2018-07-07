#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod tcp;
mod actions;
mod message_parser;
mod state;

fn main() {
    let rx = tcp::listen().expect("failed to make tcp connection");
    let mut application_state = state::State::new();

    loop {
        let action = rx.recv().expect("error receiving msg");
        application_state = actions::run(action, application_state).expect("Failed to run action");
    }
}
