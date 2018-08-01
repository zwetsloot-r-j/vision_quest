use ::state::{State, HistoryItem};
use ::ui::id_state::IdState;
use ::ui::json_inspector;
use conrod::{UiCell, Positionable, Widget, Sizeable};
use serde_json;

pub fn render(mut id_state: IdState, ui_cell: &mut UiCell, ref item: &HistoryItem, _state: &State) -> IdState {
    let action_lists = id_state.ids.action_lists.to_vec();
    let sibling_id = id_state.last(&action_lists, "action_lists").unwrap();

    let ids = id_state.ids.payload_texts.to_vec();
    let id = id_state.next(&ids, "payload_texts").unwrap();

    json_inspector::JsonInspector::new(serde_json::from_str(item.action.amplitude.as_str()).unwrap(), String::from("amplitude"))
        .right_from(sibling_id, 5.0)
        .w_h(700.0, 200.0)
        .scroll_kids()
        .set(id, ui_cell)
        ;

    id_state
}
