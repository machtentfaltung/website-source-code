// Copyright © 2025 Matei Pralea <matei@machtentfaltung.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeMap;

use crate::lang::Language;
use eframe::egui::{self, Button, FontData, FontDefinitions, FontFamily, FontId, Id, RichText};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Application {
    show_more_window: bool,
    show_about_window: bool,
    show_settings_window: bool,
    compact: bool,
    language: Language,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            show_more_window: false,
            show_about_window: false,
            show_settings_window: false,
            compact: false,
            language: Language::English,
        }
    }
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "DINish".to_owned(),
            FontData::from_static(include_bytes!("../assets/DINish-Regular.ttf")).into(),
        );

        fonts.font_data.insert(
            "Autobahn".to_owned(),
            FontData::from_static(include_bytes!("../assets/Autobahn.ttf")).into(),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "DINish".to_owned());

        let mut new_family = BTreeMap::new();

        new_family.insert(
            FontFamily::Name("Autobahn".into()),
            vec!["Autobahn".to_owned()],
        );

        fonts.families.append(&mut new_family);

        cc.egui_ctx.set_fonts(fonts);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

fn custom_heading(ui: &mut egui::Ui, text: impl ToString, strong: bool) {
    ui.add_space(3.0);

    let mut text = egui::RichText::new(text.to_string()).font(FontId {
        size: 24.0,
        family: FontFamily::Name("Autobahn".into()),
    });

    if strong {
        text = text.strong();
    }

    ui.heading(text);
}

fn main_window_ui(app: &mut Application, ui: &mut egui::Ui) {
    custom_heading(ui, app.language.main_heading(), true);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(format!("{} ", app.language.my_name_is()));
        ui.label(egui::RichText::new("Matei Pralea.").strong());
    });
    ui.add_space(4.);
    ui.hyperlink_to(
        format!(
            "{} github.com/machtentfaltung",
            egui::special_emojis::GITHUB
        ),
        "https://github.com/machtentfaltung",
    );

    if app.compact {
        ui.add_space(5.);
        let text = if app.show_more_window {
            app.language.show_less_about_me()
        } else {
            app.language.show_more_about_me()
        };

        if ui.add_sized([170., 25.], Button::new(text)).clicked() {
            app.show_more_window = !app.show_more_window;
        }
    } else {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
                let text = if app.show_more_window {
                    app.language.show_less_about_me()
                } else {
                    app.language.show_more_about_me()
                };

                if ui.add_sized([170., 25.], Button::new(text)).clicked() {
                    app.show_more_window = !app.show_more_window;
                }
            });
        });
    }
}

impl eframe::App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_size = ctx.input(|i| i.screen_rect());
        self.compact = if screen_size.width() <= 450. {
            true
        } else {
            false
        };

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button(self.language.about()).clicked() {
                    self.show_about_window = !self.show_about_window;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui.button(self.language.settings()).clicked() {
                        self.show_settings_window = !self.show_settings_window;
                    }
                });
            })
        });

        if self.compact {
            egui::CentralPanel::default().show(ctx, |ui| main_window_ui(self, ui));
        } else {
            egui::Window::new(self.language.my_website())
                .id(Id::new("main_window"))
                .collapsible(false)
                .resizable(false)
                .fixed_size([300., 100.])
                .min_size([300., 100.])
                .show(ctx, |ui| main_window_ui(self, ui));
        }

        if self.show_more_window {
            egui::Window::new(self.language.more_about_me())
                .id(Id::new("more_window"))
                .collapsible(false)
                .resizable(false)
                .open(&mut self.show_more_window)
                .fixed_size([265., 100.])
                .min_size([265., 100.])
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                        ui.label(self.language.more_description());
                    });
                });
        }

        if self.show_settings_window {
            let mut open = self.show_settings_window;
            let window = egui::Window::new(self.language.settings())
                .id(Id::new("settings_window"))
                .collapsible(false)
                .resizable(false)
                .open(&mut open)
                .fixed_size([400., 100.])
                .min_size([400., 100.]);
            window.show(ctx, |ui| {
                ui.horizontal(|ui| {
                    custom_theme_buttons(self, ui);
                    ui.vertical(|ui| {
                        ui.label(self.language.language());
                        egui::ComboBox::from_id_salt("language_combo_box")
                            .selected_text(self.language.language_name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.language,
                                    Language::English,
                                    Language::English.language_name(),
                                );
                                ui.selectable_value(
                                    &mut self.language,
                                    Language::Romanian,
                                    Language::Romanian.language_name(),
                                );
                            });
                    })
                });
            });
            self.show_settings_window = open;
        }

        if self.show_about_window {
            egui::Window::new(self.language.about())
                .id(Id::new("about_window"))
                .collapsible(false)
                .resizable(false)
                .fixed_size([350., 220.])
                .min_size([350., 220.])
                .open(&mut self.show_about_window)
                .show(ctx, |ui| {
                    custom_heading(ui, self.language.my_website(), true);
                    ui.link("https://machtentfaltung.de");
                    ui.separator();
                    ui.hyperlink_to(
                        format!(
                            "{} {}",
                            egui::special_emojis::GITHUB,
                            self.language.website_source_code()
                        ),
                        "https://github.com/machtentfaltung/website-source-code",
                    );
                });
        }
    }
}

fn custom_theme_buttons(app: &Application, ui: &mut egui::Ui) {
    let mut theme_preference = ui.ctx().options(|opt| opt.theme_preference);
    theme_preference_custom_buttons(&mut theme_preference, app, ui);
    ui.ctx().set_theme(theme_preference);
}

fn theme_preference_custom_buttons(
    theme_preference: &mut egui::ThemePreference,
    app: &Application,
    ui: &mut egui::Ui,
) {
    ui.vertical(|ui| {
        ui.label(app.language.theme());
        egui::ComboBox::from_id_salt("theme_combo_box")
            .selected_text(match theme_preference {
                egui::ThemePreference::System => app.language.system(),
                egui::ThemePreference::Dark => app.language.dark(),
                egui::ThemePreference::Light => app.language.light(),
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    theme_preference,
                    egui::ThemePreference::System,
                    app.language.system(),
                );
                ui.selectable_value(
                    theme_preference,
                    egui::ThemePreference::Dark,
                    app.language.dark(),
                );
                ui.selectable_value(
                    theme_preference,
                    egui::ThemePreference::Light,
                    app.language.light(),
                );
            });
    });
}

