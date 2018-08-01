use std::io::Error;
use actions::{Message, Action};
use state::{HistoryState, HistoryItem, HistoryAction};

extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
enum Payload {
    Json(String),
    HistoryItem(HistoryItemDto),
}

#[derive(Serialize, Deserialize, Debug)]
struct HistoryItemDto {
    Domain: String,
    Invocation: String,
    Amplitude: String,
    StateType: String,
    State: String,
}

impl Payload {
    fn to_message(self) -> Result<Message, Error> {
        match self {
            Payload::Json(content) => Ok(Message::Raw(content)),
            Payload::HistoryItem(content) => {
                let history_state = HistoryState::new(content.StateType.clone(), content.State.clone());
                let history_action = HistoryAction::new(
                    content.Domain.clone(),
                    content.Invocation.clone(),
                    content.Amplitude.clone()
                );
                let history_item = HistoryItem::new(history_action, history_state);
                Ok(Message::HistoryItem(history_item))
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Packet {
    Domain: String,
    Invocation: String,
    Payload: Payload,
}

pub fn parse(raw : String, sender : String) -> Result<Action, Error> {
    let packet : Packet = serde_json::from_str(&raw).expect("failed to parse");
    let message = packet.Payload.to_message()?;

    Ok(Action {
        domain: packet.Domain.clone(),
        invocation: packet.Invocation.clone(),
        message: message,
        sender: sender.clone(),
    })
}
