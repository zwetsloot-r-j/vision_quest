use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::io::Error;
use ::state::State;

pub fn run() -> Result<Sender<State>, Error> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let mut state = rx.recv().expect("ui failed to receive the application state");

        loop {
            match rx.try_recv() {
                Ok(new_state) => state = new_state,
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => panic!("ui disconnected from main thread"),
            };

            render(&state);
        }
    });

    Ok(tx)
}

fn render(state: &State) {
    ()
}
