use egui_macroquad::{egui::{epaint::Shadow, Align2, Rounding, Vec2, Visuals, Window}, ui};
use crate::{AUTHORS, VERSION};

pub(crate) fn paint()
{
  ui(|egui_context|
  {
    egui_context.set_visuals(Visuals
    {
      window_shadow: Shadow::NONE,
      // window_rounding: Rounding
      // { nw: 10., ne: 0., sw: 10., se: 0. },
      // window_fill: Color32::from_rgb(32, 0, 64),
      // window_stroke: Stroke::new(2., Color32::from_rgb(0, 192, 192)),
      // override_text_color: Some(Color32::from_rgb(255, 210, 255)),
      ..Default::default()
    });

    Window::new("Grid Path Finder")
      .anchor(Align2::RIGHT_TOP, Vec2::new(0., 0.))
      .constrain(true)
      .collapsible(false)
      .movable(false)
      .resizable(false)
      .fixed_size(Vec2::new(150., 0.))
      .show(egui_context, |ui|
      {
        // ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(0, 64, 64);
        // ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::from_rgb(0, 64, 64);
        // ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(0, 128, 128);
        // ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::from_rgb(0, 128, 128);
        // ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::from_rgb(0, 192, 192);
        // ui.style_mut().visuals.widgets.active.bg_fill = Color32::from_rgb(0, 192, 192);
        ui.label("Set random obstacles");
        ui.label("% of obstacles (slider)");
        ui.label("find path (button)");

        ui.separator();

        ui.add_space(200.);

        ui.separator();

        // --- CREDITS (!important) ---
        ui.horizontal(|ui|
        {
          ui.label(format!("v{}", VERSION.unwrap_or("unknown")));
          ui.separator();
          ui.label("Made by");
          ui.hyperlink_to(format!("{}", AUTHORS.unwrap_or("unknown")), "https://github.com/an-Iceberg");
        });
      });
  });
}
