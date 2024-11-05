#![windows_subsystem = "windows"]

mod app;
mod gui;

use app::Triggerbot;
use eframe::{
  NativeOptions,
  egui::{
    ViewportBuilder,
    IconData,
  }
};

fn main () -> Result<(), eframe::Error> {

  let options = NativeOptions {
    viewport: ViewportBuilder::default()
      .with_resizable(false)
      .with_transparent(true)
      .with_maximize_button(false)
      .with_inner_size([350.0, 212.0])
      .with_icon(IconData {
        rgba: image::load_from_memory(include_bytes!("../assets/icon.png")).unwrap().to_rgba8().to_vec(),
        width: 128,
        height: 128,
      }),
    ..Default::default()
  };

  eframe::run_native(
    "Valorant Triggerbot (v1.2.0)",
    options,
    Box::new(|_cc| Ok(Box::new(Triggerbot::default()))),
  )
}