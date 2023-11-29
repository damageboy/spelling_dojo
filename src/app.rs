//use egui::{Align2, CentralPanel, ComboBox, CtxRef, Frame, Label, Layout, TopBottomPanel, Ui, Vec2};
//use eframe::{App, FrameContext, IntegrationInfo, setup_native, TextureAllocator};


use egui::{CentralPanel, Label, TextStyle, TopBottomPanel};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SpellingDojoApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    question_num: usize,
    questions: Vec<String>,
    answers: Vec<String>,
    user_answers: Vec<String>,


}

impl Default for SpellingDojoApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,

            question_num: 0,
            user_answers: vec!["".to_string(), "".to_string()],

            questions: vec![
                "component".to_string(),
                "spelling".to_string(),
            ],
            answers: vec![
                "component".to_string(),
                "spelling".to_string(),
            ],
        }
    }
}

impl SpellingDojoApp {
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

    fn check_answers(&self) -> Vec<bool> {
        let mut results = Vec::new();
        for (i, answer) in self.answers.iter().enumerate() {
            results.push(answer == &self.questions[i]);
        }
        results
    }
}

impl eframe::App for SpellingDojoApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }


    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add(Label::new("Spelling Quiz"));
            ui.horizontal(|ui| {
                ui.add(Label::new(&self.questions[self.question_num]));
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let answer = &self.answers[self.question_num];
                for i in 0..answer.len() {
                    ui.add(egui::TextEdit::singleline(&mut self.user_answers[i]));
                }
                
                if ui.button("Submit Answer").clicked() {
                    let errors = self.check_answers();
                    let false_count = errors.iter().filter(|&b| *b == false).count();
                    ui.add(Label::new(format!("{} false answers", false_count)));
                    self.question_num = (self.question_num + 1) % self.questions.len();
                    self.user_answers.clear();
                    self.user_answers.resize(self.answers[self.question_num].len(), "".to_string());
                }
            });
        });



    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
