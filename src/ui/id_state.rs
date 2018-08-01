use std::collections::HashMap;
use conrod::{UiCell, widget};
use ::ui::Ids;

pub struct IdState {
    pub ids: Ids,
    indices: HashMap<String, usize>,
}

impl IdState {
    pub fn new(ids: Ids) -> IdState {
        IdState {
            ids: ids,
            indices: HashMap::new(),
        }
    }

    pub fn next(&mut self, ref id_vec: &Vec<widget::Id>, key: &str) -> Option<widget::Id> {
        let current_index = self.indices.entry(String::from(key)).or_insert(0);
        match *current_index < id_vec.len() {
            true => {
                let id = id_vec[*current_index];
                *current_index += 1;
                Some(id)
            },
            false => None,
        }
    }

    pub fn last(&self, ref id_vec: &Vec<widget::Id>, key: &str) -> Option<widget::Id> {
        match self.indices.get(&String::from(key)) {
            Some(id) => {
                if id > &id_vec.len() {
                    None
                } else {
                    Some(id_vec[(*id) - 1])
                }
            },
            None => None,
        }
    }

    pub fn generate_client_widget_ids(&mut self, amount: usize, ui_cell: &mut UiCell) {
        self.generate_client_canvases(amount, ui_cell);
        self.generate_action_lists(amount, ui_cell);
        self.generate_payload_texts(amount, ui_cell);
        self.generate_item_state_texts(amount, ui_cell);
    }

    fn generate_client_canvases(&mut self, amount: usize, ui_cell: &mut UiCell) {
        self.ids.client_canvases.resize(amount, &mut ui_cell.widget_id_generator());
    }

    fn generate_action_lists(&mut self, amount: usize, ui_cell: &mut UiCell) {
        self.ids.action_lists.resize(amount, &mut ui_cell.widget_id_generator());
    }

    fn generate_payload_texts(&mut self, amount: usize, ui_cell: &mut UiCell) {
        self.ids.payload_texts.resize(amount, &mut ui_cell.widget_id_generator());
    }

    fn generate_item_state_texts(&mut self, amount: usize, ui_cell: &mut UiCell) {
        self.ids.item_state_texts.resize(amount, &mut ui_cell.widget_id_generator());
    }

    pub fn reset(&mut self) {
        self.indices.clear();
    }

    pub fn unwrap(self) -> Ids {
        self.ids
    }
}
