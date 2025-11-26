// Copyright © 2025 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::app::Application;
use crate::extra_impl::bool_ext::BoolExt;

use eframe::egui;

pub fn top_panel(app: &mut Application, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top").show(ctx, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            if ui.button(app.language.about()).clicked() {
                app.window_configuration.show_about_window.toggle();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                if ui.button(app.language.settings()).clicked() {
                    app.window_configuration.show_settings_window.toggle();
                }
            });
        })
    });
}
