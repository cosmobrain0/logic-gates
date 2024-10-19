use egui::{Button, Color32, Label, Pos2, Rect, RichText, Ui, Vec2};

use super::transformation::{Element, HasId, Input, Output};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Toggle {
    value: bool,
    id: u64,
    position: Pos2,
}
impl Toggle {
    pub fn new(position: Pos2, id: u64) -> Self {
        Self {
            value: false,
            id,
            position,
        }
    }
}
impl Element for Toggle {
    fn update(&mut self, ui: &mut Ui) {
        let colour = if self.value {
            Color32::from_rgb(0, 0, 128)
        } else {
            Color32::from_rgb(128, 0, 0)
        };
        if ui
            .put(
                Rect::from_center_size(self.position, Vec2::new(10.0, 15.0)),
                Button::new(if self.value { "1" } else { "0" }).fill(colour),
            )
            .clicked()
        {
            self.value = !self.value;
        }
    }

    fn as_element_mut(&mut self) -> &mut dyn Element {
        self as &mut dyn Element
    }
}
impl HasId for Toggle {
    fn id(&self) -> u64 {
        self.id
    }
}
impl Input for Toggle {
    fn value(&self) -> bool {
        self.value
    }
}

pub struct OutputNode {
    position: Pos2,
    id: u64,
    value: bool,
}
impl OutputNode {
    pub fn new(position: Pos2, id: u64) -> Self {
        Self {
            value: false,
            id,
            position,
        }
    }
}
impl Element for OutputNode {
    fn update(&mut self, ui: &mut Ui) {
        // TODO: extract these colours into constants
        let colour = if self.value {
            Color32::from_rgb(0, 0, 128)
        } else {
            Color32::from_rgb(128, 0, 0)
        };
        ui.put(
            Rect::from_center_size(self.position, Vec2::new(20.0, 30.0)),
            Label::new(
                if self.value {
                    RichText::new("1")
                } else {
                    RichText::new("0")
                }
                .background_color(colour),
            ),
        );
    }

    fn as_element_mut(&mut self) -> &mut dyn Element {
        self as &mut dyn Element
    }
}
impl HasId for OutputNode {
    fn id(&self) -> u64 {
        self.id
    }
}
impl Input for OutputNode {
    fn value(&self) -> bool {
        self.value
    }
}
impl Output for OutputNode {
    fn set_value(&mut self, value: bool) {
        self.value = value;
    }
}
