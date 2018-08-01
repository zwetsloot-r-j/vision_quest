use std::sync::mpsc::{channel};
use std::thread;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;

mod tcp;
mod actions;
mod message_parser;
mod state;
mod ui;

fn main() {
    let (ui_tx, ui_rx) = channel();

    thread::spawn(move || {
        let (tx, rx) = tcp::listen().expect("failed to make tcp connection");
        let mut application_state = state::State::new(tx);

        loop {
            let action = rx.recv().expect("error receiving msg");
            application_state = actions::run(action, application_state).expect("Failed to run action");
            ui_tx.send(application_state.clone()).expect("Failed to send application state to the ui");

            match application_state.status {
                state::Status::ShuttingDown => break,
                _ => (),
            };
        }
    });

    ui::run(ui_rx);
}
