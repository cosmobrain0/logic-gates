#![feature(trait_upcasting)]
use egui::Pos2;
mod gates;
mod toggle;
mod transformation;

use toggle::Toggle;
use transformation::*;

use self::gates::Nand;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
/// FIXME: can't serialize any of the elements!
pub struct TemplateApp {
    #[serde(skip)]
    inputs: Vec<Box<dyn Input>>,
    #[serde(skip)]
    outputs: Vec<Box<dyn Output>>,
    #[serde(skip)]
    transformations: Vec<Box<dyn Transformation>>,
    #[serde(skip)]
    connections: Vec<Connection>,
    #[serde(skip)]
    previous_id: u64,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let mut id = 0;
        let generate_id = || {
            id = id + 1;
            id - 1
        };
        Self {
            transformations: vec![Box::new(Nand::new(Pos2::new(100.0, 100.0), generate_id))],
            inputs: vec![],
            outputs: vec![],
            connections: vec![],
            previous_id: 0,
        }
    }
}

impl TemplateApp {
    pub fn all_inputs(&self) -> Vec<&dyn Input> {
        self.inputs
            .iter()
            .map(|x| x.as_ref())
            .chain(
                self.transformations
                    .iter()
                    .map(|x| x.inputs().into_iter())
                    .flatten(),
            )
            .collect()
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            for element in self
                .inputs
                .iter_mut()
                .map(|x| x.as_mut().as_element_mut())
                .chain(self.outputs.iter_mut().map(|x| x.as_mut().as_element_mut()))
                .chain(
                    self.transformations
                        .iter_mut()
                        .map(|x| x.as_mut().as_element_mut()),
                )
            {
                element.update(ui);
            }
            for connection in &mut self.connections {
                connection
                    .pass_signal(
                        self.all_inputs(),
                        self.outputs
                            .iter_mut()
                            .map(|x| x.as_mut() as &mut dyn Output)
                            .chain(
                                self.transformations
                                    .iter()
                                    .map(|x| x.outputs_mut().into_iter())
                                    .flatten(),
                            )
                            .collect(),
                    )
                    .expect("passing signals failed");
            }
        });
    }
}
