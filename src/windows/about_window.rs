// Copyright © 2025 Matei Pralea <matei@machtentfaltung.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::app::Application;
use crate::extra_impl::extra_ui_impl::ExtraUiImpl;

use eframe::egui;

pub fn about_window(app: &mut Application, ctx: &egui::Context) {
    egui::Window::new(app.language.about())
        .id(egui::Id::new("about_window"))
        .collapsible(false)
        .resizable(false)
        .fixed_size([350., 220.])
        .min_size([350., 220.])
        .open(&mut app.window_configuration.show_about_window)
        .show(ctx, |ui| {
            ui.custom_heading(app.language.my_website());
            let _ = ui.link("https://machtentfaltung.de");
            ui.separator();
            ui.hyperlink_to(
                format!(
                    "{} {}",
                    egui::special_emojis::GITHUB,
                    app.language.website_source_code()
                ),
                "https://github.com/machtentfaltung/website-source-code",
            );
        });
}
