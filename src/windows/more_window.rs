// Copyright © 2025 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::app::Application;

use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, PartialEq)]
pub enum MoreWindowTab {
    #[default]
    Programming,
    OtherInterests,
}

pub fn more_window(app: &mut Application, ctx: &egui::Context) {
    egui::Window::new(app.language.more_about_me())
        .id(egui::Id::new("more_window"))
        .collapsible(false)
        .resizable(false)
        .open(&mut app.window_configuration.show_more_window)
        .fixed_size([300., 100.])
        .min_size([300., 100.])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(
                        app.window_configuration.more_window_tab == MoreWindowTab::Programming,
                        app.language.more_window_programming_tab(),
                    )
                    .clicked()
                {
                    app.window_configuration.more_window_tab = MoreWindowTab::Programming;
                }

                if ui
                    .selectable_label(
                        app.window_configuration.more_window_tab == MoreWindowTab::OtherInterests,
                        app.language.more_window_other_interests_tab(),
                    )
                    .clicked()
                {
                    app.window_configuration.more_window_tab = MoreWindowTab::OtherInterests;
                }
            });

            ui.separator();

            match app.window_configuration.more_window_tab {
                MoreWindowTab::Programming => {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.label(app.language.more_window_programming_text());
                    });
                }
                MoreWindowTab::OtherInterests => {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.label(app.language.more_window_other_interests_text());
                    });
                }
            }
        });
}
