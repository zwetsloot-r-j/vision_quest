use std::f64;
use conrod::{self, widget, Widget, Colorable, Labelable, Point, Positionable, Sizeable, Borderable, UiCell};
use conrod::widget::{Text, Button};
use serde_json;

#[derive(WidgetCommon)]
pub struct JsonInspector {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    style: Style,
    key: String,
    content: serde_json::Value,
    current_item: usize,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod::Color>,
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<conrod::Color>,
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<conrod::FontSize>,
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<conrod::text::font::Id>>,
}

widget_ids! {
    struct Ids {
        key,
        item,
        json,
        button,
        scroll_horizontal,
        scroll_vertical,
    }
}

pub struct State {
    ids: Ids,
    opened: bool,
    height: f64,
    children_heights: Option<Vec<f64>>,
}

pub fn calculate_json_height(value: &serde_json::Value) -> f64 {
    match *value {
        serde_json::Value::Array(ref v) => {
            let height = v.iter()
                .map(|v| calculate_json_height(v))
                .sum()
                ;
            f64::max(25.0, height)
        },
        serde_json::Value::Object(ref v) => {
            let height = v.iter()
                .map(|(_, v)| calculate_json_height(v))
                .sum()
                ;
            f64::max(25.0, height)
        },
        _ => 25.0,
    }
}

impl JsonInspector {
    pub fn new(content: serde_json::Value, mut key: String) -> Self {
        key.push_str(": ");

        JsonInspector {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            key: key,
            content: content,
            current_item: 0,
        }
    }

    fn key_id(&self, state: &State) -> widget::Id {
        match self.content {
            serde_json::Value::Array(_) | serde_json::Value::Object(_) => state.ids.button,
            _ => state.ids.key,
        }
    }

    fn make_key(&self, parent_id: widget::Id, state: &State, key: String, ui: &mut UiCell) -> Option<()> {
        match self.content {
            serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                let color = if state.opened { conrod::color::LIGHT_BLUE } else { conrod::color::BLUE };
                let button = widget::Button::new()
                    .color(color)
                    .w_h(150.0, 25.0)
                    .top_left_of(parent_id)
                    .label(key.as_str())
                    .set(state.ids.button, ui)
                    ;

                if button.was_clicked() { Some(()) } else { None }
            },
            _ => {
                widget::Text::new(key.as_str())
                    .top_left_of(parent_id)
                    .set(state.ids.key, ui)
                    ;

                None
            },
        }
    }

    fn make_string_content(&self, id: widget::Id, key_id: widget::Id, content: String, ui: &mut UiCell) {
        widget::Text::new(content.as_str())
            .right_from(key_id, 1.0)
            .set(id, ui)
            ;
    }

    fn make_list_content(&self, state: &State, ui: &mut UiCell) -> Option<Vec<f64>> {
        if (state.opened) {
            let (mut items, scrollbar) = widget::List::flow_down(self.item_amount())
                .w_h(700.0, state.height)
                .right_from(self.key_id(state), 5.0)
                .set(state.ids.json, ui)
                ;

            let mut children_heights = Vec::new();
            while let Some(item) = items.next(ui) {
                let (key, json_item) = self.get_item(item.i).expect("json item index out of range");
                let json_insp = JsonInspector::new(json_item, key.clone())
                    .h(self.get_item_height(item.i, state));

                let child_height = item.set(json_insp, ui);

                children_heights.push(child_height);
            };

            if children_heights.len() == 0 { None } else { Some(children_heights) }
        } else {
            None
        }
    }

    fn item_amount(&self) -> usize {
        match self.content {
            serde_json::Value::Array(ref v) => v.len(),
            serde_json::Value::Object(ref v) => v.len(),
            _ => 1,
        }
    }

    fn unwrap_item(&self) -> Option<String> {
        match self.content {
            serde_json::Value::Null => Some(String::from("null")),
            serde_json::Value::Bool(ref v) => {
                if *v {
                    Some(String::from("true"))
                } else {
                    Some(String::from("false"))
                }
            },
            serde_json::Value::Number(ref v) => Some(v.to_string()),
            serde_json::Value::String(ref v) => Some(v.clone()),
            _ => None,
        }
    }

    fn get_item(&self, index: usize) -> Option<(String, serde_json::Value)> {
        match self.content {
            serde_json::Value::Array(ref v) => {
                v.get(index).map(|ref item| (index.to_string(), (*item).clone()))
            },
            serde_json::Value::Object(ref v) => {
                v.iter().nth(index).map(|(ref key, ref value)| ((*key).clone(), (*value).clone()))
            },
            _ => None,
        }
    }

    fn get_item_height(&self, index: usize, state: &State) -> f64 {
        *state.children_heights
            .as_ref()
            .unwrap_or(&Vec::new())
            .get(index)
            .unwrap_or(&25.0)
    }

    fn next_item(&mut self) -> Option<(String, serde_json::Value)> {
        let current_item_index = self.current_item;
        self.current_item += 1;
        match self.content {
            serde_json::Value::Array(ref v) => {
                v.get(current_item_index).map(|ref item| (current_item_index.to_string(), (*item).clone()))
            },
            serde_json::Value::Object(ref v) => {
                v.iter().nth(current_item_index).map(|(ref key, ref value)| ((*key).clone(), (*value).clone()))
            },
            _ => None,
        }
    }
}

impl Widget for JsonInspector {
    type State = State;
    type Style = Style;
    type Event = f64;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        let mut ids = Ids::new(id_gen);
        State {
            ids: ids,
            opened: false,
            height: 25.0,
            children_heights: None,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, ui, style, .. } = args;
        let mut height = 0.0;

        widget::Scrollbar::x_axis(id).auto_hide(true).set(state.ids.scroll_horizontal, ui);
        widget::Scrollbar::y_axis(id).auto_hide(true).set(state.ids.scroll_vertical, ui);

        //self.make_key(state.ids.key, id, self.key.clone(), ui);
        match self.make_key(id, state, self.key.clone(), ui) {
            Some(()) => state.update(|state| state.opened = !state.opened),
            _ => (),
        };

        match self.content {
            serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                let children_heights = self.make_list_content(&state, ui);
                state.update(|state| state.children_heights = children_heights);
                let sum: f64 = state.children_heights.as_ref().unwrap_or(&vec![25.0]).iter().sum();
                height += sum;
            },
            _ => {
                let item = self.unwrap_item().unwrap();
                self.make_string_content(state.ids.item, state.ids.key, item, ui);
                height += 25.0;
            },
        }

        state.update(|state| state.height = height);

        height
    }
}
