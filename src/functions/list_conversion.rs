use eframe::egui::{ComboBox, Ui};

use crate::radix::{format_for_radix, Radix};

#[derive(Default, Debug, Clone)]
pub struct ListConversion {
    input: String,
    input_separator: String,
    output: String,
    output_separator: String,
    from_radix: Radix,
    to_radix: Radix,
}

impl ListConversion {
    fn run_conversion(&mut self) {
        self.output = self
            .input
            .split(&self.input_separator)
            .collect::<Vec<&str>>()
            .iter()
            .flat_map(|a| u64::from_str_radix(a, self.from_radix.into()))
            .map(|a| format_for_radix(self.to_radix, a))
            .collect::<Vec<String>>()
            .join(&self.output_separator);
    }
}

impl ListConversion {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let label = ui.label("Input value separator:");
            let entry = ui
                .text_edit_singleline(&mut self.input_separator)
                .labelled_by(label.id);
            if entry.changed() {
                self.run_conversion();
            }
        });
        ui.horizontal(|ui| {
            let label = ui.label("Input radix");
            ComboBox::from_id_source(label.id)
                .selected_text(self.from_radix.get_text())
                .show_ui(ui, |ui| {
                    for radix in Radix::iter() {
                        let val =
                            ui.selectable_value(&mut self.from_radix, radix, radix.get_text());
                        if val.changed() {
                            self.run_conversion();
                        }
                    }
                })
        });
        ui.vertical(|ui| {
            let label = ui.label("Input:");
            let entry = ui
                .text_edit_multiline(&mut self.input)
                .labelled_by(label.id);
            if entry.changed() {
                self.run_conversion();
            }
        });
        ui.horizontal(|ui| {
            let label = ui.label("Output value separator:");
            let entry = ui
                .text_edit_singleline(&mut self.output_separator)
                .labelled_by(label.id);
            if entry.changed() {
                self.run_conversion();
            }
        });
        ui.horizontal(|ui| {
            let label = ui.label("Output radix");
            ComboBox::from_id_source(label.id)
                .selected_text(self.to_radix.get_text())
                .show_ui(ui, |ui| {
                    for radix in Radix::iter() {
                        let val = ui.selectable_value(&mut self.to_radix, radix, radix.get_text());
                        if val.changed() {
                            self.run_conversion();
                        }
                    }
                });
        });
        ui.vertical(|ui| {
            let label = ui.label("Output:");
            let entry = ui
                .text_edit_multiline(&mut self.output)
                .labelled_by(label.id);
            if entry.changed() {
                self.run_conversion();
            }
        });
    }
}
