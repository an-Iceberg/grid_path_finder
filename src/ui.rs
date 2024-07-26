use egui_macroquad::{egui::{epaint::Shadow, Align2, Slider, Vec2, Visuals, Window}, ui};
use crate::{grid::Grid, MouseMode, AUTHORS, VERSION};

pub(crate) fn paint(
  mouse_mode: &mut MouseMode,
  grid: &mut Grid,
  animate: &mut bool,
  ratio: &mut f64,
  // unvisited_nodes: &mut Vec<macroquad::math::Vec2>,
  speed: &mut u8,
)
{
  ui(|egui_context|
  {
    egui_context.set_visuals(Visuals
    {
      window_shadow: Shadow::NONE,
      ..Default::default()
    });

    Window::new("Grid Path Finder (A*)")
      .anchor(Align2::RIGHT_TOP, Vec2::new(0., 0.))
      .constrain(true)
      .collapsible(false)
      .movable(false)
      .resizable(false)
      .fixed_size(Vec2::new(150., 0.))
      .show(egui_context, |ui|
      {
        ui.label("Left click action:");
        ui.horizontal(|ui|
        {
          ui.selectable_value(mouse_mode, MouseMode::Node, "Node");
          ui.selectable_value(mouse_mode, MouseMode::Obstacle, "Obstacle");
          // ToDo: Colour start and end with their node colors
          ui.selectable_value(mouse_mode, MouseMode::Start, "Start");
          ui.selectable_value(mouse_mode, MouseMode::End, "End");
        });

        // ToDo: implement this functionality
        if *mouse_mode == MouseMode::Obstacle
        {
          ui.separator();
          let mut obstacle_mode = 1;
          ui.horizontal(|ui|
          {
            ui.label("Obstacle count:");
            ui.selectable_value(&mut obstacle_mode, 1, " 1 ");
            ui.selectable_value(&mut obstacle_mode, 4, " 4 ");
            ui.selectable_value(&mut obstacle_mode, 9, " 9 ");
          });
        }

        ui.separator();

        ui.horizontal(|ui|
        {
          if ui.button("Clear «Start»").clicked()
          { grid.clear_start(); }
          if ui.button("Clear «End»").clicked()
          { grid.clear_end(); }
        });

        ui.separator();

        if ui.button("Fill grid with random obstacles").clicked()
        { grid.set_random_obstacles(*ratio); }

        ui.label("Obstacle ratio");
        ui.add(Slider::new(ratio, 0.0..=1.0));

        if ui.button("Clear grid").clicked() { grid.clear(); }

        if grid.get_start().is_some() && grid.get_end().is_some()
        {
          ui.separator();

          ui.horizontal(|ui|
          {
            if ui.button("Find path with A*").clicked()
            {
              grid.clear_path_data();
              grid.find_path();
            }
            ui.checkbox(animate, "Animate");
          });
          if ui.button("Clear path").clicked() { grid.clear_path_data(); }
          ui.label("Speed");
          ui.add(Slider::new(speed, 1..=10));
        }

        ui.separator();

        // --- CREDITS (!important) ---
        ui.horizontal(|ui|
        {
          ui.label(format!("v{}", VERSION.unwrap_or("unknown")));
          ui.separator();
          ui.label("Made by");
          ui.hyperlink_to(AUTHORS.unwrap_or("unknown").to_string(), "https://github.com/an-Iceberg");
        });
      });
  });
}
