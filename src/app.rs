use eframe::{App, CreationContext, Frame};
#[allow(unused_imports)] // TODO: remove when time comes
use egui::{Margin, Rounding, Stroke};
#[allow(unused_imports)] // TODO: remove when time comes
use egui::{pos2, vec2, Align2, CentralPanel, Color32, Context, RichText, Shadow, Shape, SidePanel, Ui, Visuals, Window};
use crate::{node::Cell, AUTHORS, REPOSITORY, VERSION};

pub struct GridPathFinder
{
  screen_width: f32,
  screen_height: f32,
  node_radius: f32,
  gap_size: f32,
  window_edge_offset: f32,
  ui_width: f32,
  x_cell_count: i32,
  y_cell_count: i32,

  grid: Vec<Cell>,
}

impl GridPathFinder
{
  /// Called once before the first frame.
  #[allow(clippy::needless_return)]
  pub fn new(cc: &CreationContext<'_>) -> Self
  {
    // This is also where you can customize the look and feel of egui using
    // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

    cc.egui_ctx.set_visuals(Visuals
    {
      window_shadow: Shadow::NONE,
      ..Default::default()
    });

    return Default::default();
  }

  fn calc_x_cell_count(&mut self)
  {
    self.x_cell_count = ((self.screen_width - 2.*self.window_edge_offset - self.ui_width) / (2.*self.node_radius + self.gap_size)).floor() as i32 + 1;
  }

  fn calc_y_cell_count(&mut self)
  {
    self.y_cell_count = ((self.screen_height - 2.*self.window_edge_offset) / (2.*self.node_radius + self.gap_size)).floor() as i32 + 1;
  }

  fn update_cell_count(&mut self)
  {
    self.calc_x_cell_count();
    self.calc_y_cell_count();

    let new_cell_count = (self.x_cell_count * self.y_cell_count) as usize;

    if new_cell_count != self.grid.len()
    {
      self.grid = vec![Cell::Node; new_cell_count];
    }
  }
}

impl Default for GridPathFinder
{
  #[allow(clippy::needless_return)]
  fn default() -> Self
  {
    let mut default = Self
    {
      screen_width: 800.,
      screen_height: 600.,
      node_radius: 5.,
      gap_size: 10.,
      window_edge_offset: 20., // Actual offset from window edge is 15.
      ui_width: 170.,
      x_cell_count: 0,
      y_cell_count: 0,
      grid: vec![],
    };

    default.calc_x_cell_count();
    default.calc_y_cell_count();

    default.grid = vec![Cell::Node; (default.x_cell_count * default.y_cell_count) as usize];

    return default;
  }
}

impl App for GridPathFinder
{
  /// Called each time the UI needs repainting, which may be many times per second.
  fn update(&mut self, ctx: &Context, frame: &mut Frame)
  {
    let δ_time = frame.info().cpu_usage.unwrap_or(0.001);
    let screen_rect = ctx.input(|i: &egui::InputState| i.screen_rect());
    let (old_screen_width, old_screen_height) = (self.screen_width, self.screen_height);
    (self.screen_width, self.screen_height) = (screen_rect.width(), screen_rect.height());

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
      ui.monospace(RichText::new(format!("width: {}", self.screen_width)));
      ui.monospace(RichText::new(format!("height: {}", self.screen_height)));
      ui.monospace(RichText::new(format!("cell count: {}", self.grid.len())));
      ui.separator();

      ui.heading("Grid Path Finder");
      ui.separator();
      ui.add_space(20.);

      ui.heading("Controls");
      ui.add_space(20.);

      ui.heading("Stats");
      ui.monospace(RichText::new(format!("fps:{:>7.2}", 1./δ_time)));
      ui.add_space(20.);

      credits(ui);
    });

    CentralPanel::default().show(ctx, |ui|
    {
      // Canvas to draw on
      egui::Frame::canvas(ui.style())
        .fill(Color32::from_rgb(110, 80, 30))
        // .inner_margin(Margin::ZERO)
        // .outer_margin(Margin::ZERO)
        // .rounding(Rounding{nw: 20., ne: 20., sw: 20., se: 20.})
      .show(ui, |ui|
      {
        ui.allocate_space(ui.available_size());

        let mut shapes = vec![];

        // Todo: calculate this and adjust gap for pleasant appearance

        // Creating cartesian product for cell coordinates
        let coords = (0..self.x_cell_count)
          .flat_map(|x| (0..self.y_cell_count)
            .map(|y| (x, y)).collect::<Vec<_>>()
          ).collect::<Vec<_>>();

        // «Drawing» the cells
        shapes.append(
          &mut coords.iter()
            .map(|(x, y)| (*x as f32, *y as f32))
            .map(|(x, y)| (x * ((self.node_radius * 2.) + self.gap_size), y * ((self.node_radius * 2.) + self.gap_size)))
            .map(|(x, y)| (x + self.window_edge_offset, y + self.window_edge_offset))
            .map(|(x, y)| Shape::circle_stroke(pos2(x, y), self.node_radius, Stroke::new(1., Color32::WHITE)))
            .collect()
        );

        // Drawing everything
        ui.painter().extend(shapes);

        // Todo: Make more efficient use of this
        ui.ctx().request_repaint();
      });
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
    ui.hyperlink_to("Sandra", "https://github.com/an-Iceberg")
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
