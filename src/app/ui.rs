use eframe::{App, Frame};
#[allow(unused_imports)] // TODO: remove when time comes
use egui::{Margin, CornerRadius, Stroke};
#[allow(unused_imports)] // TODO: remove when time comes
use egui::{pos2, vec2, Align2, CentralPanel, Color32, Context, RichText, Shadow, Shape, SidePanel, Ui, Visuals, Window};
use crate::components::{toggle::toggle, cell::cell};
use crate::{AUTHORS, REPOSITORY, VERSION};
use super::GridPathFinder;

impl App for GridPathFinder
{
  /// Called each time the UI needs repainting, which may be many times per second.
  fn update(&mut self, ctx: &Context, frame: &mut Frame)
  {
    let δ_time = frame.info().cpu_usage.unwrap_or(0.001);
    let screen_rect = ctx.input(|i: &egui::InputState| i.screen_rect());
    let (old_screen_width, old_screen_height) = (self.screen_width, self.screen_height);
    (self.screen_width, self.screen_height) = (screen_rect.width(), screen_rect.height());

    // For later
    self.mouse = ctx.pointer_hover_pos().unwrap_or_default();

    // Screen size changed, update screen & recalculate grid size
    if old_screen_height != self.screen_height || old_screen_width != self.screen_width
    {
      // Recalculate grid size
      // Todo: calculate amount of nodes in x and y direction
      self.update_cell_count();

      // println!("{:?}", SystemTime::now());

      // Reset graph
    }

    // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
    // For inspiration and more examples, go to https://emilk.github.io/egui

    SidePanel::right("my_right_panel")
      .resizable(false)
      .exact_width(self.ui_width)
    .show(ctx, |ui|
    {
      ui.separator();
      ui.heading("Debug stuff");
      ui.add(toggle(&mut self.toggle_value));
      ui.add(cell(&mut self.toggle_value));
      ui.monospace(RichText::new(format!("width: {}", self.screen_width)));
      ui.monospace(RichText::new(format!("height: {}", self.screen_height)));
      ui.monospace(RichText::new(format!("x_cell_count: {}", self.grid_width)));
      ui.monospace(RichText::new(format!("y_cell_count: {}", self.grid_height)));
      // debug
      if ui.button("Toggle cell ig").clicked()
      {
        self.get_mut((0,0)).toggle();
      }
      let cell_type = if self.get((0, 0)).is_node() { "node" } else { "obstacle" };
      ui.monospace(RichText::new(format!("cell type: {cell_type}")));
      ui.separator();

      ui.heading("Grid Path Finder");
      ui.separator();
      ui.add_space(20.);

      ui.heading("Controls");
      ui.add_space(20.);

      ui.heading("Stats");
      ui.monospace(RichText::new(format!("fps:{:>7.2}", 1./δ_time)));
      ui.monospace(RichText::new(format!("# cells: {}", self.grid.len())));
      ui.monospace(RichText::new(format!("# columns: {}", self.grid_width)));
      ui.monospace(RichText::new(format!("# rows: {}", self.grid_height)));
      ui.add_space(20.);

      credits(ui);
    });

    CentralPanel::default().show(ctx, |ui|
    {
      /*
      // Canvas to draw on
      egui::Frame::canvas(ui.style())
        .fill(Color32::from_rgb(110, 80, 30))
        // .inner_margin(Margin::ZERO)
        // .outer_margin(Margin::ZERO)
        // .rounding(CornerRadius{nw: 20., ne: 20., sw: 20., se: 20.})
      .show(ui, |ui|
      {
        ui.allocate_space(ui.available_size());

        let mut shapes = vec![];

        // Todo: calculate this and adjust gap for pleasant appearance

        // Todo: instead of doing all this create a custom widget TωT
        // howto: https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/toggle_switch.rs
        // helpful: https://github.com/emilk/egui/discussions/3696
        for x in 0..self.grid_width {
        for y in 0..self.grid_height{
          let is_node = self.get((x, y)).is_node();

          let (mut x, mut y) = (x as f32, y as f32);
          // Calculating the center of the circle
          (x, y) = (x * ((self.node_radius * 2.) + self.gap_size), y * ((self.node_radius * 2.) + self.gap_size));
          (x, y) = (x + self.window_edge_offset, y + self.window_edge_offset);

          // Todo: Figure out mouse hover

          // Drawing the circles
          // If the mouse is hovering over the node
          if self.is_mouse_in_square(x, y)
          {
            shapes.push(Shape::circle_stroke(pos2(x, y), self.node_radius, Stroke::new(1., Color32::from_rgb(255, 0, 255))));
          }
          else if is_node { shapes.push(Shape::circle_stroke(pos2(x, y), self.node_radius, Stroke::new(1., Color32::WHITE))); }
          else { shapes.push(Shape::circle_filled(pos2(x, y), self.node_radius, Color32::WHITE)); }


          // shapes.push(Shape::circle_stroke(pos2(x, y), self.node_radius, Stroke::new(1., Color32::WHITE)));
        }
        }

        // Drawing everything
        ui.painter().extend(shapes);

        // Todo: Make more efficient use of this
        ui.ctx().request_repaint();
      }); */
    });
  }
}

fn credits(ui: &mut Ui)
{
  ui.heading("Credits");

  // Me 💙🩷🤍🩷💙
  ui.horizontal(|ui|
  {
    ui.monospace(RichText::new(format!("v{}", VERSION.unwrap_or("unknown"))));
    ui.separator();
    ui.spacing_mut().item_spacing.x = 0.0;
    ui.label("Made by ");
    ui.hyperlink_to("Priscilla", "https://github.com/an-Iceberg")
      .on_hover_text(AUTHORS.unwrap());
  });

  ui.hyperlink_to("GitHub repository link", REPOSITORY.unwrap())
    .on_hover_text(REPOSITORY.unwrap());

  // egui & eframe
  ui.horizontal(|ui|
  {
    let egui_link = "https://github.com/emilk/egui#readme";
    let eframe_link = "https://github.com/emilk/eframe_template#readme";

    ui.spacing_mut().item_spacing.x = 0.0;
    ui.label("Powered by ");
    ui.hyperlink_to("egui", egui_link).on_hover_text(egui_link);
    ui.label(" & ");
    ui.hyperlink_to("eframe", eframe_link,).on_hover_text(eframe_link);
  });
}
