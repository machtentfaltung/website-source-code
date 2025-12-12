// Copyright © 2025 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::extra_impl::extra_ctx_impl::ExtraCtxImpl;
use crate::language::Language;
use crate::windows::about_window::about_window;
use crate::windows::main_window::main_window;
use crate::windows::more_window::{MoreWindowTab, more_window};
use crate::windows::settings_window::settings_window;
use crate::windows::top_panel::top_panel;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Application {
    pub window_configuration: WindowConfiguration,
    pub language: Language,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct WindowConfiguration {
    pub compact: bool,
    pub show_more_window: bool,
    pub show_about_window: bool,
    pub show_settings_window: bool,
    pub more_window_tab: MoreWindowTab,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            window_configuration: WindowConfiguration::default(),
            language: Language::English,
        }
    }
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        let regular_font =
            egui::FontData::from_static(include_bytes!("../assets/DINishExpanded-Regular.ttf"));

        fonts
            .font_data
            .insert("DINishExpanded-Regular".to_owned(), regular_font.into());

        let bold_font =
            egui::FontData::from_static(include_bytes!("../assets/DINishExpanded-Black.ttf"));

        fonts
            .font_data
            .insert("DINishExpanded-Black".to_owned(), bold_font.into());

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "DINishExpanded-Regular".to_owned());

        let mut new_family = BTreeMap::new();

        new_family.insert(
            egui::FontFamily::Name("DINishExpanded-Black".into()),
            vec!["DINishExpanded-Black".to_owned()],
        );

        fonts.families.append(&mut new_family);

        cc.egui_ctx.set_fonts(fonts);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.toggle_compact(&mut self.window_configuration.compact);

        top_panel(self, ctx);
        main_window(self, ctx);

        if self.window_configuration.show_more_window {
            more_window(self, ctx);
        }

        if self.window_configuration.show_settings_window {
            settings_window(self, ctx);
        }

        if self.window_configuration.show_about_window {
            about_window(self, ctx);
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
