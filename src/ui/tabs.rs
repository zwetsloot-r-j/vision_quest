use ::state::State;
use ::ui::IdState;
use conrod::{widget, color, UiCell, Colorable, Positionable, Widget, Sizeable};

pub fn render(id_state: IdState, ui_cell: &mut UiCell, ref state: &State) -> IdState {
    let mut tabs: Vec<(widget::Id, &str)> = Vec::new();

    for (index, (id, _client)) in state.clients.iter().enumerate() {
        tabs.push((id_state.ids.client_canvases[index], id));
    };

    widget::Tabs::new(&tabs)
        .wh_of(ui_cell.window)
        .mid_top_of(ui_cell.window)
        .layout_horizontally()
        .color(color::LIGHT_BLUE)
        .bar_thickness(50.0)
        .starting_tab_idx(0)
        .starting_canvas(id_state.ids.client_canvases[0])
        .set(id_state.ids.tab, ui_cell)
        ;

    id_state
}
