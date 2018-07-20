use ::state::{State, HistoryItem};
use ::ui::id_state::IdState;
use ::ui::json_inspector;
use conrod::{widget, color, UiCell, Colorable, Positionable, Widget, Sizeable};
use conrod::widget::Text;
use serde_json;

pub fn render(mut idState: IdState, ui_cell: &mut UiCell, ref item: &HistoryItem, state: &State) -> IdState {
    let action_lists = idState.ids.action_lists.to_vec();
    let sibling_id = idState.last(&action_lists, "action_lists").unwrap();

    let ids = idState.ids.payload_texts.to_vec();
    let id = idState.next(&ids, "payload_texts").unwrap();

    json_inspector::JsonInspector::new(serde_json::from_str(item.action.amplitude.as_str()).unwrap(), String::from("amplitude"))
        .right_from(sibling_id, 5.0)
        .w_h(700.0, 200.0)
        .scroll_kids()
        .set(id, ui_cell)
        ;

    idState
}
