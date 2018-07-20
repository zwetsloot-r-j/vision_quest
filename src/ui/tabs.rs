use ::state::State;
use ::ui::IdState;
use conrod::{widget, color, UiCell, Colorable, Positionable, Widget, Sizeable};

pub fn render(mut idState: IdState, ui_cell: &mut UiCell, ref state: &State) -> IdState {
    //idState.generate_client_canvases(state.clients.len(), ui_cell);

    let mut tabs: Vec<(widget::Id, &str)> = Vec::new();

    for (index, (id, _client)) in state.clients.iter().enumerate() {
        tabs.push((idState.ids.client_canvases[index], id));
    };

    widget::Tabs::new(&tabs)
        .wh_of(ui_cell.window)
        .mid_top_of(ui_cell.window)
        .layout_horizontally()
        .color(color::LIGHT_BLUE)
        .bar_thickness(50.0)
        .starting_tab_idx(0)
        .starting_canvas(idState.ids.client_canvases[0])
        .set(idState.ids.tab, ui_cell)
        ;

    idState
}
