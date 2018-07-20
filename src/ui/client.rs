use ::state::{State, Client};
use ::ui::{action_list, payload, item_state};
use ::ui::id_state::IdState;
use conrod::{widget, Widget, color, Colorable, Positionable, Sizeable, UiCell};

pub fn render(mut idState: IdState, ui_cell: &mut UiCell, ref client: &Client, state: &State) -> IdState {
    let canvases = idState.ids.client_canvases.to_vec();
    idState.next(&canvases, "client_canvases").unwrap();

    idState = action_list::render(idState, ui_cell, client, state);
    idState = match client.selected_history_item() {
        Some(item) => {
            let mut idState = payload::render(idState, ui_cell, item, state);
            item_state::render(idState, ui_cell, item, state)
        },
        None => idState,
    };

    idState
}
