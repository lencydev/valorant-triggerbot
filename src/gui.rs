use crate::app::Triggerbot;

use eframe::egui::{
  Slider, Color32, CentralPanel, Button,
  DragValue, Context, RichText, ScrollArea, ComboBox,
};

pub fn build (app: &mut Triggerbot, ctx: &Context) {

  CentralPanel::default().show(ctx, |ui| {

    ui.vertical(|ui| {

      ui.set_width(ui.available_width());

      ui.group(|ui| {

        ui.set_width(ui.available_width());

        ui.horizontal(|ui| {

          let mut width = app.settings.resolution.width;
          let mut height = app.settings.resolution.height;

          ui.label("Resolution:");
          if ui.add(DragValue::new(&mut width).speed(1).range(1..=5000)).changed() {
            app.set_resolution(width, app.settings.resolution.height);
          }
          ui.label("x");
          if ui.add(DragValue::new(&mut height).speed(1).range(1..=5000)).changed() {
            app.set_resolution(app.settings.resolution.width, height);
          }
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {

          ui.set_width(ui.available_width());

          ui.label("Trigger Keys:");

          ComboBox::from_id_salt("combobox_trigger_keys")
            .selected_text(format!("{} keys selected", app.settings.trigger_keys.len()))
            .width(160.0)
            .show_ui(ui, |ui| {

              ui.set_min_width(160.0);
              ui.set_max_height(80.0);

              ScrollArea::vertical().max_height(200.0).show(ui, |ui| {

                let available_triggers = app.get_keys();

                for trigger in available_triggers {

                  let mut is_selected = app.settings.trigger_keys.contains(&trigger);
                  let display_name = app.get_keys_display_name(&trigger);

                  if ui.checkbox(&mut is_selected, display_name).changed() {

                    if is_selected && !app.settings.trigger_keys.contains(&trigger) {

                      app.settings.trigger_keys.push(trigger);

                    } else if !is_selected {

                      if let Some(pos) = app.settings.trigger_keys.iter().position(|x| *x == trigger) {

                        app.settings.trigger_keys.remove(pos);
                      }
                    }
                  }
                }
              });
            });
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {

          ui.set_width(ui.available_width());

          ui.label("Trigger Delay (ms):");
          ui.add(Slider::new(&mut app.settings.trigger_delay, 0..=3000));
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {

          ui.set_width(ui.available_width());

          ui.label("Trigger Area:");
          if ui.add(Slider::new(&mut app.settings.trigger_area, 5.0..=100.0)).changed() {
            app.update_trigger_area();
          }
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {

          ui.set_width(ui.available_width());

          ui.label("Target Color (R, G, B):");
          for i in 0..3 {
            ui.add(DragValue::new(&mut app.settings.target_color[i]).speed(1).range(0..=255));
          }
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {

          ui.set_width(ui.available_width());

          ui.label("Color Tolerance:");
          ui.add(Slider::new(&mut app.settings.color_tolerance, 0..=255));
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
