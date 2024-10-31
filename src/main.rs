#![windows_subsystem = "windows"]

mod app;
mod gui;

use app::Triggerbot;
use eframe::{ egui::Vec2, NativeOptions, IconData };

fn main () -> Result<(), eframe::Error> {

  let window_size = Option::from(Vec2::new(350.0, 186.0));

  let options = NativeOptions {
    initial_window_size: window_size,
    max_window_size: window_size,
    min_window_size: window_size,
    resizable: false,
    maximized: false,
    centered: true,
    icon_data: Some(IconData::try_from_png_bytes(include_bytes!("../assets/favicon.ico")).unwrap()),
    ..Default::default()
  };

  eframe::run_native("Valorant Triggerbot (v1.1.0)", options, Box::new(|cc| Box::new(Triggerbot::new(cc))))
}