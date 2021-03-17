#![allow(non_snake_case)]

mod settings;
mod gui;

use gui::*;
use iced::{Sandbox, Settings, window};

fn main() -> iced::Result {
  AllStatTabsSettings::run(Settings {
    antialiasing: true,
    window: window::Settings {
      size: (640, 800),
      resizable: false,
      ..window::Settings::default()
    },
    ..Settings::default()
  })
}
