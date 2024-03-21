use egui_macroquad::{egui::{epaint::Shadow, Align2, Slider, Vec2, Visuals, Window}, ui};
use crate::{grid::Grid, MouseMode, AUTHORS, VERSION};

pub(crate) fn paint(
  mouse_mode: &mut MouseMode,
  grid: &mut Grid,
  animate: &mut bool,
  finding_path: &mut bool,
  ratio: &mut f64,
)
{
  ui(|egui_context|
  {
    egui_context.set_visuals(Visuals
    {
      window_shadow: Shadow::NONE,
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
        ui.add_space(10.);

        ui.label("Set cell(s) to:");
        ui.horizontal(|ui|
        {
          ui.selectable_value(mouse_mode, MouseMode::Node, "Node");
          ui.selectable_value(mouse_mode, MouseMode::Obstacle, "Obstacle");
          // Colour start and end with their node colors
          ui.selectable_value(mouse_mode, MouseMode::Start, "Start");
          ui.selectable_value(mouse_mode, MouseMode::End, "End");
        });

        ui.add_space(10.);
        ui.separator();
        ui.add_space(10.);

        ui.horizontal(|ui|
        {
          if ui.button("Clear «Start»").clicked()
          { grid.clear_start(); }
          if ui.button("Clear «End»").clicked()
          { grid.clear_end(); }
        });

        ui.add_space(10.);
        ui.separator();
        ui.add_space(10.);

        if ui.button("Fill grid with random obstacles").clicked()
        { grid.set_random_obstacles(*ratio); }
        ui.add(Slider::new(ratio, 0.0..=1.0).text("Ratio"));

        if grid.get_start().is_some() && grid.get_end().is_some()
        {
          ui.add_space(10.);
          ui.separator();
          ui.add_space(10.);

          ui.horizontal(|ui|
          {
            if ui.button("Find path").clicked()
            { *finding_path = true; }
            ui.checkbox(animate, "Animate");
          });
        }

        ui.add_space(10.);
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
