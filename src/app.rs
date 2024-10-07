use crate::gui;

use screenshots::Screen;
use std::cmp::PartialEq;
use device_query::{ DeviceQuery, DeviceState, Keycode };
use enigo::{ Enigo, KeyboardControllable, Key };
use eframe::{ App, CreationContext, Frame, egui::Context };

#[derive(PartialEq)]
pub struct Settings {
  pub trigger_key: Keycode,
  pub target_color: [i32; 3],
  pub color_tolerance: i32,
}

impl Default for Settings {
  fn default () -> Self {
    Self {
      trigger_key: Keycode::LShift,
      target_color: [250, 100, 250],
      color_tolerance: 70,
    }
  }
}

pub struct TriggerArea {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
}

pub struct Triggerbot {
  pub enabled: bool,
  pub device_state: DeviceState,
  pub enigo: Enigo,
  pub screen: Screen,
  pub trigger_area: TriggerArea,
  pub settings: Settings,
}

impl Triggerbot {

  pub fn new (_cc: &CreationContext<'_>) -> Self {

    let screen = Screen::from_point(0, 0).unwrap();
    let (width, height) = (screen.display_info.width as i32, screen.display_info.height as i32);

    Self {
      enabled: false,
      device_state: DeviceState::new(),
      enigo: Enigo::new(),
      screen,
      trigger_area: TriggerArea {
        x: width / 2 - 5,
        y: height / 2 - 5,
        width: width / 2 + 5,
        height: height / 2 + 5,
      },
      settings: Settings::default(),
    }
  }

  pub fn reset_settings (&mut self) {
    self.settings = Settings::default();
  }

  pub fn is_default_settings (&self) -> bool {
    self.settings == Settings::default()
  }

  pub fn get_available_keys (&self) -> Vec<Keycode> {
    vec![
      Keycode::LShift,
      Keycode::LControl,
      Keycode::LAlt,
    ]
  }

  pub fn shoot (&mut self) {

    let keys: Vec<Keycode> = self.device_state.get_keys();

    if keys.contains(&self.settings.trigger_key) {

      if self.is_target_color_present() {

        self.enigo.key_click(Key::Layout('k'));
      }
    }
  }

  fn is_target_color_present (&self) -> bool {

    let capture = self.screen.capture_area(
      self.trigger_area.x,
      self.trigger_area.y,
      (self.trigger_area.width - self.trigger_area.x) as u32,
      (self.trigger_area.height - self.trigger_area.y) as u32,
    ).unwrap();

    let pixels = capture.pixels();

    let matching_pixels: Vec<_> = pixels.filter(|p| {
      p.0.iter().zip(&self.settings.target_color).all(|(a, b)| ((*a as i32) - b).abs() <= self.settings.color_tolerance)
    }).collect();

    !matching_pixels.is_empty()
  }
}

impl App for Triggerbot {

  fn update (&mut self, ctx: &Context, _frame: &mut Frame) {

    gui::build(self, ctx);

    if self.enabled {
      self.shoot();
    }

    ctx.request_repaint();
  }
}