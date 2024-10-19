use egui::{epaint::PathStroke, Color32, Pos2, Sense, Shape, Ui};

pub trait Element {
    fn update(&mut self, ui: &mut Ui);
    fn as_element_mut(&mut self) -> &mut dyn Element;
}

pub trait HasId {
    fn id(&self) -> u64;
}

pub trait Input: HasId + Element {
    fn value(&self) -> bool;
}

pub trait Output: Input {
    fn set_value(&mut self, value: bool);
}

pub trait Transformation: Element {
    fn inputs(&self) -> Vec<&dyn Input>;
    fn outputs_mut(&self) -> Vec<&mut dyn Output>;
}

pub struct Connection {
    input_id: u64,
    output_id: u64,
    last_input_position: Option<Pos2>,
    last_output_position: Option<Pos2>,
    last_input_value: bool,
}
impl Connection {
    pub fn new(input_id: u64, output_id: u64) -> Self {
        Self {
            input_id,
            output_id,
            last_input_position: None,
            last_output_position: None,
            last_input_value: false,
        }
    }

    pub fn pass_signal(
        &mut self,
        inputs: Vec<&dyn Input>,
        outputs: Vec<&mut dyn Output>,
    ) -> Result<(), ()> {
        let input = inputs.iter().find(|input| input.id() == self.input_id);
        let output = outputs
            .into_iter()
            .find(|output| output.id() == self.output_id);
        match (input, output) {
            (Some(input), Some(output)) => {
                self.last_input_value = input.value();
                output.set_value(self.last_input_value);
                Ok(())
            }
            _ => Err(()),
        }
    }
}
impl Element for Connection {
    fn update(&mut self, ui: &mut Ui) {
        if let (Some(input), Some(output)) = (self.last_input_position, self.last_output_position) {
            // TODO: consider some potentially weird coordinate transformations
            let (mut response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

            // let to_screen = emath::RectTransform::from_to(
            //     Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            //     response.rect,
            // );
            // let from_screen = to_screen.inverse();

            let colour = if self.last_input_value {
                Color32::from_rgb(0, 0, 128)
            } else {
                Color32::from_rgb(128, 0, 0)
            };
            painter.extend([Shape::line_segment(
                [input, output],
                PathStroke::new(5.0, colour),
            )]);
        }
    }

    fn as_element_mut(&mut self) -> &mut dyn Element {
        self as &mut dyn Element
    }
}
