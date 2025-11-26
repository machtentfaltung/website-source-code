// Copyright © 2025 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::app::Application;
use crate::extra_impl::extra_ui_impl::ExtraUiImpl;

use eframe::egui;

pub fn settings_window(app: &mut Application, ctx: &egui::Context) {
    let mut open = app.window_configuration.show_settings_window;
    let window = egui::Window::new(app.language.settings())
        .id(egui::Id::new("settings_window"))
        .collapsible(false)
        .resizable(false)
        .open(&mut open)
        .fixed_size([400., 100.])
        .min_size([400., 100.]);
    window.show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.theme_combo_box(&app.language);
            ui.language_combo_box(app);
        });
    });
    app.window_configuration.show_settings_window = open;
}
