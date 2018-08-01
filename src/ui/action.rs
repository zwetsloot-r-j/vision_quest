use ::state::HistoryAction;
use conrod::{widget, color, UiCell, Labelable, Borderable, Colorable};
use conrod::widget::list::{Item, Down, Fixed};

pub fn render(parent: Item<Down, Fixed>, ui_cell: &mut UiCell, ref action: &HistoryAction) {
    let text = format!("{}:{}", action.domain, action.invocation);
    let (color, text_color) = match action.selected {
        true => (color::BLUE, color::BLACK),
        false => (color::LIGHT_BLUE, color::BLACK),
    };

    let button = widget::Button::new()
        .border(1.0)
        .color(color)
        .label(&text)
        .label_font_size(16)
        .label_color(text_color)
        ;

    parent.set(button, ui_cell);
}
