// Copyright © 2025 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::app::Application;
use crate::language::Language;
use eframe::egui;

pub trait ExtraUiImpl {
    fn theme_combo_box(&mut self, language: &Language);
    fn language_combo_box(&mut self, app: &mut Application);
    fn custom_heading(&mut self, text: impl ToString);
}

impl ExtraUiImpl for egui::Ui {
    fn theme_combo_box(&mut self, language: &Language) {
        let mut ui_theme_preference = self.ctx().options(|opt| opt.theme_preference);

        self.vertical(|ui| {
            ui.label(language.theme());
            egui::ComboBox::from_id_salt("theme_combo_box")
                .selected_text(match ui_theme_preference {
                    egui::ThemePreference::System => language.system(),
                    egui::ThemePreference::Dark => language.dark(),
                    egui::ThemePreference::Light => language.light(),
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut ui_theme_preference,
                        egui::ThemePreference::System,
                        language.system(),
                    );
                    ui.selectable_value(
                        &mut ui_theme_preference,
                        egui::ThemePreference::Dark,
                        language.dark(),
                    );
                    ui.selectable_value(
                        &mut ui_theme_preference,
                        egui::ThemePreference::Light,
                        language.light(),
                    );
                });
        });
        self.ctx().set_theme(ui_theme_preference);
    }

    fn language_combo_box(&mut self, app: &mut Application) {
        self.vertical(|ui| {
            ui.label(app.language.language());
            egui::ComboBox::from_id_salt("language_combo_box")
                .selected_text(app.language.language_name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut app.language,
                        Language::English,
                        Language::English.language_name(),
                    );
                    ui.selectable_value(
                        &mut app.language,
                        Language::Romanian,
                        Language::Romanian.language_name(),
                    );
                });
        });
    }

    fn custom_heading(&mut self, text: impl ToString) {
        self.add_space(3.0);

        let text = egui::RichText::new(text.to_string())
            .size(20.0)
            .strong();

        self.heading(text);
    }
}
