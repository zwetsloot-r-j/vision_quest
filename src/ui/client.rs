use ::state::{State, Client};
use ::ui::{action_list, payload, item_state};
use ::ui::id_state::IdState;
use conrod::UiCell;

pub fn render(mut id_state: IdState, ui_cell: &mut UiCell, ref client: &Client, state: &State) -> IdState {
    let canvases = id_state.ids.client_canvases.to_vec();
    id_state.next(&canvases, "client_canvases").unwrap();

    id_state = action_list::render(id_state, ui_cell, client, state);
    id_state = match client.selected_history_item() {
        Some(item) => {
            let mut id_state = payload::render(id_state, ui_cell, item, state);
            item_state::render(id_state, ui_cell, item, state)
        },
        None => id_state,
    };

    id_state
}
