use egui::{Label, Pos2, Rect, Ui, Vec2};

use super::{toggle::OutputNode, transformation::*};

pub struct Nand {
    inputs: [OutputNode; 2],
    output: OutputNode,
    position: Pos2,
}
impl Nand {
    pub fn new(position: Pos2, mut generate_id: impl FnMut() -> u64) -> Self {
        Self {
            inputs: [
                OutputNode::new(position - Vec2::new(30.0, 30.0), generate_id()),
                OutputNode::new(position - Vec2::new(30.0, -30.0), generate_id()),
            ],
            output: OutputNode::new(position + Vec2::new(30.0, 0.0), generate_id()),
            position,
        }
    }
}
impl Element for Nand {
    fn update(&mut self, ui: &mut Ui) {
        self.inputs[0].update(ui);
        self.inputs[1].update(ui);

        self.output
            .set_value(!(self.inputs[0].value() & self.inputs[1].value()));

        self.output.update(ui);

        ui.put(
            Rect::from_center_size(self.position, Vec2::new(60.0, 30.0)),
            Label::new("NAND"),
        );
    }

    fn as_element_mut(&mut self) -> &mut dyn Element {
        self as &mut dyn Element
    }
}
impl Transformation for Nand {
    fn inputs(&self) -> Vec<&dyn Input> {
        vec![&self.inputs[0] as &dyn Input, &self.inputs[1] as &dyn Input]
    }

    fn outputs_mut(&mut self) -> Vec<&mut dyn Output> {
        vec![&self.output as &dyn Output]
    }
}
