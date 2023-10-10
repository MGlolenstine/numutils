use eframe::egui::Ui;

use crate::radix::{format_for_radix, Radix};

#[derive(Default, Debug, Clone)]
pub struct NumericConversions {
    text_hex: String,
    text_dec: String,
    text_oct: String,
    text_bin: String,
}

impl NumericConversions {
    fn clear_numbers(&mut self) {
        for radix in Radix::iter() {
            self.set_radix_text(radix, "");
        }
    }

    fn set_numbers(&mut self, value: u64) {
        for radix in Radix::iter() {
            self.set_radix_text(radix, format_for_radix(radix, value));
        }
    }

    fn get_mut_text_from_radix(&mut self, radix: Radix) -> &mut String {
        match radix {
            Radix::Binary => &mut self.text_bin,
            Radix::Octal => &mut self.text_oct,
            Radix::Decimal => &mut self.text_dec,
            Radix::Hexadecimal => &mut self.text_hex,
        }
    }

    fn get_text_from_radix(&self, radix: Radix) -> &str {
        match radix {
            Radix::Binary => self.text_bin.as_str(),
            Radix::Octal => self.text_oct.as_str(),
            Radix::Decimal => self.text_dec.as_str(),
            Radix::Hexadecimal => self.text_hex.as_str(),
        }
    }

    fn set_radix_text<S: ToString>(&mut self, radix: Radix, text: S) {
        match radix {
            Radix::Binary => self.text_bin = text.to_string(),
            Radix::Octal => self.text_oct = text.to_string(),
            Radix::Decimal => self.text_dec = text.to_string(),
            Radix::Hexadecimal => self.text_hex = text.to_string(),
        }
    }

    fn handle_text_change(&mut self, radix: Radix) {
        let inner_text = self.get_text_from_radix(radix).to_string();

        if let Ok(a) = u64::from_str_radix(&inner_text, radix.into()) {
            self.set_numbers(a);
        } else {
            self.clear_numbers();
            self.set_radix_text(radix, inner_text);
        }
    }
}

impl NumericConversions {
    pub fn ui(&mut self, ui: &mut Ui) {
        let mut add_entry = |nc: &mut Self, radix: Radix| {
            ui.horizontal(|ui| {
                let label = ui.label(format!("{}: ", radix.get_short_text()));
                let text = nc.get_mut_text_from_radix(radix);
                let entry = ui.text_edit_singleline(text).labelled_by(label.id);
                if entry.changed() {
                    nc.handle_text_change(radix);
                }
            });
        };

        add_entry(self, Radix::Hexadecimal);
        add_entry(self, Radix::Decimal);
        add_entry(self, Radix::Octal);
        add_entry(self, Radix::Binary);
    }
}
