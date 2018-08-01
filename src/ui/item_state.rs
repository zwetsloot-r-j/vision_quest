use ::state::{State, HistoryItem};
use ::ui::id_state::IdState;
use ::ui::json_inspector;
use conrod::{UiCell, Positionable, Widget, Sizeable};
use serde_json;

pub fn render(mut id_state: IdState, ui_cell: &mut UiCell, ref item: &HistoryItem, _state: &State) -> IdState {
    let payload_texts = id_state.ids.payload_texts.to_vec();
    let sibling_id = id_state.last(&payload_texts, "payload_texts").unwrap();

    let ids = id_state.ids.item_state_texts.to_vec();
    let id = id_state.next(&ids, "item_state_texts").unwrap();

    let json_value = serde_json::from_str(item.state.content.as_str()).unwrap();

    json_inspector::JsonInspector::new(json_value, String::from("state"))
        .down_from(sibling_id, 20.0)
        .w_h(700.0, 450.0)
        .scroll_kids()
        .set(id, ui_cell)
        ;

    id_state
}
