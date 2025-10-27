// Copyright © 2025 Matei Pralea <matei.pralea@proton.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub enum Language {
    English,
    Romanian,
}

impl Language {
    pub fn main_heading(&self) -> &'static str {
        match self {
            Language::English => "Hello there!",
            Language::Romanian => "Salutare!",
        }
    }

    pub fn language(&self) -> &'static str {
        match self {
            Language::English => "Language",
            Language::Romanian => "Limbă",
        }
    }

    pub fn language_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Romanian => "Română",
        }
    }

    pub fn my_website(&self) -> &'static str {
        match self {
            Language::English => "My Website",
            Language::Romanian => "Website-ul meu",
        }
    }

    pub fn my_name_is(&self) -> &'static str {
        match self {
            Language::English => "My name is",
            Language::Romanian => "Numele meu este",
        }
    }

    pub fn show_more_about_me(&self) -> &'static str {
        match self {
            Language::English => "Show More About Me",
            Language::Romanian => "Arată mai multe despre mine",
        }
    }

    pub fn show_less_about_me(&self) -> &'static str {
        match self {
            Language::English => "Show Less About Me",
            Language::Romanian => "Arată mai puține despre mine",
        }
    }

    pub fn more_about_me(&self) -> &'static str {
        match self {
            Language::English => "More About Me",
            Language::Romanian => "Mai multe despre mine",
        }
    }

    pub fn website_source_code(&self) -> &'static str {
        match self {
            Language::English => "Website Source Code",
            Language::Romanian => "Codul sursă al website-ului",
        }
    }

    pub fn about(&self) -> &'static str {
        match self {
            Language::English => "About",
            Language::Romanian => "Despre",
        }
    }

    pub fn theme(&self) -> &'static str {
        match self {
            Language::English => "Theme",
            Language::Romanian => "Temă",
        }
    }

    pub fn light(&self) -> &'static str {
        match self {
            Language::English => "Light",
            Language::Romanian => "Luminoasă",
        }
    }

    pub fn dark(&self) -> &'static str {
        match self {
            Language::English => "Dark",
            Language::Romanian => "Întunecată",
        }
    }

    pub fn system(&self) -> &'static str {
        match self {
            Language::English => "System",
            Language::Romanian => "De sistem",
        }
    }

    pub fn more_description(&self) -> &'static str {
        match self {
            Language::English => {
                "I started my programming journey in 2020 with C# and .NET, building WinForms applications as a hobby. Since then, I've expanded my skills to languages like JavaScript, Python, Swift, C, and Rust, developing everything from Discord bots and iOS apps to cross-platform desktop software."
            }
            Language::Romanian => {
                "Am început să programez în 2020 cu C# și .NET, creând aplicații WinForms inițial ca pasiune. De atunci, mi-am extins competențele către limbaje precum JavaScript, Python, Swift, C și Rust, dezvoltând de la boți pentru Discord și aplicații iOS până la software de desktop cross-platform."
            }
        }
    }

    pub fn settings(&self) -> &'static str {
        match self {
            Language::English => "Settings",
            Language::Romanian => "Setări",
        }
    }
}
