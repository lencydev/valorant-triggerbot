use crate::app::Triggerbot;

use eframe::egui::{
  Slider, Color32, CentralPanel, Button,
  DragValue, Context, RichText, ComboBox, ScrollArea
};

pub fn build(app: &mut Triggerbot, ctx: &Context) {
  CentralPanel::default().show(ctx, |ui| {
    ui.vertical(|ui| {
      ui.set_width(ui.available_width());

      ui.group(|ui| {
        ui.set_width(ui.available_width());

        ui.horizontal(|ui| {

          let mut width = app.settings.resolution.width;
          let mut height = app.settings.resolution.height;

          ui.label("Resolution:");
          if ui.add(DragValue::new(&mut width).speed(1).clamp_range(1..=5000)).changed() {
            app.set_resolution(width, app.settings.resolution.height);
          }
          ui.label("x");
          if ui.add(DragValue::new(&mut height).speed(1).clamp_range(1..=5000)).changed() {
            app.set_resolution(app.settings.resolution.width, height);
          }
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {
          ui.set_width(ui.available_width());

          ui.label("Trigger Key:");
          ComboBox::from_id_source("trigger_key_combo")
            .selected_text(app.settings.trigger_key.to_string())
            .width(150.0)
            .show_ui(ui, |ui| {
              ui.set_min_width(140.0);
              ui.set_max_height(80.0);
              ScrollArea::vertical().max_height(80.0).show(ui, |ui| {
                for key in app.get_available_keys().iter() {
                  ui.selectable_value(&mut app.settings.trigger_key, *key, key.to_string());
                }
              });
            });
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {
          ui.set_width(ui.available_width());

          ui.label("Target Color (R, G, B):");
          for i in 0..3 {
            ui.add(DragValue::new(&mut app.settings.target_color[i]).speed(1).clamp_range(0..=255));
          }
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {
          ui.set_width(ui.available_width());

          ui.label("Color Tolerance:");
          ui.add(Slider::new(&mut app.settings.color_tolerance, 0..=255));
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {
          ui.set_width(ui.available_width());

          ui.label("Trigger Delay (ms):");
          ui.add(Slider::new(&mut app.settings.trigger_delay, 0..=3000));
        });
      });

      ui.add_space(5.0);

      ui.horizontal(|ui| {
        ui.set_width(ui.available_width());

        let (button_text, button_color) = if app.enabled {
          ("Disable", Color32::from_rgb(180, 100, 100))
        } else {
          ("Enable", Color32::from_rgb(100, 180, 100))
        };

        if ui.add_sized([ui.available_width() / 2.0, 30.0], Button::new(RichText::new(button_text).color(Color32::WHITE)).fill(button_color)).clicked() {
          app.enabled = !app.enabled;
        };

        let is_default = app.is_default_settings();

        let reset_button_color = if is_default {
          Color32::DARK_GRAY
        } else {
          Color32::from_rgb(100, 100, 180)
        };

        let reset_button = Button::new(RichText::new("Reset Settings").color(Color32::WHITE))
          .fill(reset_button_color)
          .sense(if is_default {
            eframe::egui::Sense::hover()
          } else {
            eframe::egui::Sense::click()
          });

        if ui.add_sized([ui.available_width(), 30.0], reset_button).clicked() && !is_default {
          app.reset_settings();
        }
      });
    });
  });
}
