use egui::{Layout, Align, Align2, AtomExt as _};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TimeTracker {
    // Example stuff:
    label: String,
    work_log_text: String,
}

impl Default for TimeTracker {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            work_log_text: String::new(),
            // value: 2.7,
        }
    }
}

impl TimeTracker {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TimeTracker {

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ui_builder = egui::UiBuilder::new();
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::Panel::top("top_panel").show_inside(ui, |ui| self.render_top_panel(ui));

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("No Project Selected");
                ui.heading("0H 0M 0S");
            });
            ui.separator();

            ui.scope_builder(ui_builder, |ui| {
                egui::Grid::new("Projects")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(false)
                    .show(ui, |ui| {
                        self.render_projects(ui);
                    });
            });
            ui.separator();
            
            let clear_id = egui::Id::new("clear_button");
            let clear_size = egui::Vec2::splat(ui.spacing().interact_size.y);

            let output = egui::TextEdit::multiline(&mut self.work_log_text)
                .hint_text("What are you working on?")
                // Atoms are centered by default, so we need to pass the right align here:
                .suffix(
                    egui::Atom::custom(clear_id, clear_size)
                        .atom_align(Align2([Align::RIGHT, Align::TOP])),
                )
                .horizontal_align(Align::LEFT)
                .vertical_align(Align::TOP)
                .show(ui);

            if let Some(rect) = output.response.rect(clear_id)
                && ui.place(rect, egui::Button::new("❌")).clicked()
            {
                self.work_log_text.clear();
            }

            if output.response.has_focus()
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
            {
                println!("{}", &self.work_log_text);
                self.work_log_text = "".to_owned();
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

impl TimeTracker {
    fn render_top_panel(&mut self, ui: &mut egui::Ui) {
        // The top panel is often a good place for a menu bar:
        egui::MenuBar::new().ui(ui, |ui| {
            // NOTE: no File->Quit on web pages!
            let is_web = cfg!(target_arch = "wasm32");
            if !is_web {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
    }


    fn render_projects(&mut self, ui: &mut egui::Ui) {
        if ui.link("Project Alpha").clicked() {
            println!("Start timer!")
        }
        ui.horizontal(|ui| {
            if ui.button("START").clicked() {
                println!("Start timer unending!");
            }
            if ui.button("25M").clicked() {
                println!("Start timer 25 min!");
            }
        });
        ui.end_row();
        if ui.link("Project Bravo").clicked() {
            println!("Start timer!")
        }
        ui.horizontal(|ui| {
            if ui.button("START").clicked() {
                println!("Start timer unending!");
            }
            if ui.button("25M").clicked() {
                println!("Start timer 25 min!");
            }
        });
        ui.end_row();
    }
}