use crate::gui;

use screenshots::Screen;
use device_query::{ DeviceQuery, DeviceState, Keycode };
use inputbot::MouseButton;
use eframe::{ App, CreationContext, Frame, egui::Context };

use std::{
  thread,
  time::Duration,
  cmp::PartialEq,
  fmt::{ Display, Formatter, Result },
};

#[derive(PartialEq, Debug)]
pub enum TriggerKey {
  Keyboard(Keycode),
  Mouse(MouseButton)
}

#[derive(PartialEq)]
pub struct Settings {
  pub trigger_keys: Vec<TriggerKey>,
  pub target_color: [i32; 3],
  pub color_tolerance: i32,
  pub trigger_delay: u64,
  pub resolution: Resolution,
}

impl Default for Settings {
  fn default () -> Self {
    Self {
      trigger_keys: vec![TriggerKey::Keyboard(Keycode::LShift)],
      target_color: [250, 100, 250],
      color_tolerance: 70,
      trigger_delay: 50,
      resolution: Resolution { width: 1920, height: 1080 },
    }
  }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Resolution {
  pub width: u32,
  pub height: u32,
}

pub struct TriggerArea {
  pub x_percent: f32,
  pub y_percent: f32,
  pub width_percent: f32,
  pub height_percent: f32,
}

pub struct Triggerbot {
  pub enabled: bool,
  pub device_state: DeviceState,
  pub screen: Screen,
  pub trigger_area: TriggerArea,
  pub settings: Settings,
}

impl Triggerbot {

  pub fn new (_cc: &CreationContext<'_>) -> Self {

    let screen = Screen::from_point(0, 0).unwrap();

    let mut triggerbot = Self {
      enabled: false,
      device_state: DeviceState::new(),
      screen,
      trigger_area: TriggerArea {
        x_percent: 0.0,
        y_percent: 0.0,
        width_percent: 0.0,
        height_percent: 0.0,
      },
      settings: Settings::default(),
    };

    triggerbot.update_trigger_area();

    triggerbot
  }

  pub fn reset_settings (&mut self) {
    self.settings = Settings::default();
  }

  pub fn is_default_settings (&self) -> bool {
    self.settings == Settings::default()
  }

  pub fn set_resolution (&mut self, width: u32, height: u32) {

    self.settings.resolution = Resolution { width, height };

    self.update_trigger_area();
  }

  pub fn shoot (&mut self) {

    let keys = self.device_state.get_keys();

    if self.settings.trigger_keys.iter().any(|trigger| {
      match trigger {
        TriggerKey::Keyboard(key) => keys.contains(key),
        TriggerKey::Mouse(button) => button.is_pressed(),
      }
    }) {

      if self.is_target_color_present() {

        MouseButton::LeftButton.press();
        MouseButton::LeftButton.release();

        if self.settings.trigger_delay > 0 {

          thread::sleep(Duration::from_millis(self.settings.trigger_delay));
        }
      }
    }
  }

  fn is_target_color_present (&self) -> bool {

    let (width, height) = (self.settings.resolution.width, self.settings.resolution.height);

    let x = (self.trigger_area.x_percent * width as f32) as i32;
    let y = (self.trigger_area.y_percent * height as f32) as i32;
    let w = (self.trigger_area.width_percent * width as f32) as u32;
    let h = (self.trigger_area.height_percent * height as f32) as u32;

    let capture = self.screen.capture_area(x, y, w, h).unwrap();

    let pixels = capture.pixels();

    let matching_pixels: Vec<_> = pixels.filter(|p| p.0.iter().zip(&self.settings.target_color).all(|(a, b)| ((*a as i32) - b).abs() <= self.settings.color_tolerance)).collect();

    !matching_pixels.is_empty()
  }

  pub fn update_trigger_area (&mut self) {

    let (width, height) = (self.settings.resolution.width, self.settings.resolution.height);

    let fixed_width = 5.0;
    let fixed_height = 5.0;

    self.trigger_area = TriggerArea {
      x_percent: 0.5 - (fixed_width / 2.0 / width as f32),
      y_percent: 0.5 - (fixed_height / 2.0 / height as f32),
      width_percent: fixed_width / width as f32,
      height_percent: fixed_height / height as f32,
    };
  }

  pub fn get_keys (&self) -> Vec<TriggerKey> {
    let mut triggers = vec![
      TriggerKey::Mouse(MouseButton::X1Button),
      TriggerKey::Mouse(MouseButton::X2Button),
    ];

    for key in vec![
      Keycode::LShift,
      Keycode::RShift,
      Keycode::LControl,
      Keycode::RControl,
      Keycode::LAlt,
      Keycode::RAlt,

      Keycode::A,
      Keycode::B,
      Keycode::C,
      Keycode::D,
      Keycode::E,
      Keycode::F,
      Keycode::G,
      Keycode::H,
      Keycode::I,
      Keycode::J,
      Keycode::K,
      Keycode::L,
      Keycode::M,
      Keycode::N,
      Keycode::O,
      Keycode::P,
      Keycode::Q,
      Keycode::R,
      Keycode::S,
      Keycode::T,
      Keycode::U,
      Keycode::V,
      Keycode::W,
      Keycode::X,
      Keycode::Y,
      Keycode::Z,
    ] {
      triggers.push(TriggerKey::Keyboard(key));
    }

    triggers
  }

  pub fn get_keys_display_name (&self, trigger: &TriggerKey) -> String {
    match trigger {
      TriggerKey::Keyboard(key) => match key {
        Keycode::LShift => "Left Shift".to_string(),
        Keycode::RShift => "Right Shift".to_string(),
        Keycode::LControl => "Left Control".to_string(),
        Keycode::RControl => "Right Control".to_string(),
        Keycode::LAlt => "Left Alt".to_string(),
        Keycode::RAlt => "Right Alt".to_string(),
        _ => format!("{:?}", key),
      },
      TriggerKey::Mouse(MouseButton::X1Button) => "Mouse Backward (X1)".to_string(),
      TriggerKey::Mouse(MouseButton::X2Button) => "Mouse Forward (X2)".to_string(),
      _ => format!("{:?}", trigger),
    }
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

impl Display for Resolution {

  fn fmt (&self, f: &mut Formatter<'_>) -> Result {

    write!(f, "{}x{}", self.width, self.height)
  }
}
