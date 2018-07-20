use ::state::{State, HistoryItem};
use ::ui::id_state::IdState;
use ::ui::json_inspector;
use conrod::{widget, color, UiCell, Colorable, Positionable, Widget, Sizeable};
use conrod::widget::Text;
use serde_json;

pub fn render(mut idState: IdState, ui_cell: &mut UiCell, ref item: &HistoryItem, state: &State) -> IdState {
    let payload_texts = idState.ids.payload_texts.to_vec();
    let sibling_id = idState.last(&payload_texts, "payload_texts").unwrap();

    let ids = idState.ids.item_state_texts.to_vec();
    let id = idState.next(&ids, "item_state_texts").unwrap();

    let json_value = serde_json::from_str(item.state.content.as_str()).unwrap();
    //let height = json_inspector::calculate_json_height(&json_value);

    json_inspector::JsonInspector::new(json_value, String::from("state"))
        .down_from(sibling_id, 20.0)
        .w_h(700.0, 450.0)
        .scroll_kids()
        .set(id, ui_cell)
        ;

    idState
}
