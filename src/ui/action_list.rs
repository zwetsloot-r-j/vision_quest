use ::state::{State, Client};
use ::ui::IdState;
use ::ui::action;
use ::actions::{Action, Message};
use std::collections::HashSet;
use conrod::{widget, UiCell, Positionable, Widget, Sizeable};
use conrod::widget::list_select::Event;

pub fn render(mut id_state: IdState, ui_cell: &mut UiCell, ref client: &Client, state: &State) -> IdState {
    let canvases = id_state.ids.client_canvases.to_vec();
    let parent_id = id_state.last(&canvases, "client_canvases").unwrap();

    let ids = id_state.ids.action_lists.to_vec();
    let id = id_state.next(&ids, "action_lists").unwrap();

    let (mut events, _scrollbar) = widget::ListSelect::multiple(client.history_item_amount())
         .flow_down()
         .item_size(50.0)
         .scrollbar_next_to()
         .w_h(250.0, 700.0)
         .top_left_with_margins_on(parent_id, 10.0, 10.0)
         .set(id, ui_cell)
         ;

    let selections = client.selections.clone();
    while let Some(event) = events.next(ui_cell, |i| selections.contains(&i)) {
        match event {
            Event::Item(item) => action::render(item, ui_cell, &client.history[item.i].action),
            Event::Selection(selection) => {
                let mut selections = HashSet::new();
                selection.update_index_set(&mut selections);

                let action = Action {
                    domain: String::from("action"),
                    invocation: String::from("select"),
                    message: Message::SelectAction((client.id.clone(), selections)),
                    sender: String::from("ui"),
                };

                state.dispatcher.send(action).unwrap();
            },
            _ => (),
        }
    }

    id_state
}
