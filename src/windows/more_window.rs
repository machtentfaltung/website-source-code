// Copyright © 2025 Matei Pralea <matei@machtentfaltung.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::app::Application;

use eframe::egui;

pub fn more_window(app: &mut Application, ctx: &egui::Context) {
    egui::Window::new(app.language.more_about_me())
        .id(egui::Id::new("more_window"))
        .collapsible(false)
        .resizable(false)
        .open(&mut app.window_configuration.show_more_window)
        .fixed_size([280., 100.])
        .min_size([280., 100.])
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                ui.label(app.language.more_description());
            });
        });
}
