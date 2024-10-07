use crate::app::Triggerbot;

use eframe::egui::{
  Slider, ComboBox, Color32, CentralPanel, Button,
  DragValue, Context, RichText
};

pub fn build (app: &mut Triggerbot, ctx: &Context) {

  CentralPanel::default().show(ctx, |ui| {

    ui.vertical(|ui| {

      ui.set_width(ui.available_width());

      ui.group(|ui| {

        ui.set_width(ui.available_width());

        ui.horizontal(|ui| {

          ui.set_width(ui.available_width());

          ui.label("Trigger Key:");
          ComboBox::from_label("")
            .selected_text(format!("{:?}", app.settings.trigger_key))
            .show_ui(ui, |ui| {
              for key in app.get_available_keys().iter() {
                ui.selectable_value(&mut app.settings.trigger_key, *key, key.to_string());
              }
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
      });

      ui.add_space(10.0);

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
          Color32::from_rgb(100, 100  , 180)
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