#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod functions;
pub mod radix;

use eframe::egui::{self, Context};
use functions::{list_conversion::ListConversion, numeric_conversions::NumericConversions};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
enum Panel {
    #[default]
    NumericConversions,
    ListConversion,
}

#[derive(Default)]
struct MyApp {
    open_panel: Panel,
    numeric_conversions: NumericConversions,
    list_conversion: ListConversion,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.open_panel,
                    Panel::NumericConversions,
                    "Numeric conversions",
                );
                ui.selectable_value(
                    &mut self.open_panel,
                    Panel::ListConversion,
                    "Convert list of values",
                );
            });
            ui.separator();

            match self.open_panel {
                Panel::NumericConversions => self.numeric_conversions.ui(ui),
                Panel::ListConversion => self.list_conversion.ui(ui),
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("Numutils", options, Box::new(|_cc| Box::<MyApp>::default()))
}
