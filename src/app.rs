use eframe::CreationContext;
use egui::Pos2;
#[allow(unused_imports)] // TODO: remove when time comes
use egui::{pos2, vec2, Align2, CentralPanel, Color32, Context, RichText, Shadow, Shape, SidePanel, Ui, Visuals, Window};
use crate::node::Cell;

mod ui;

pub struct GridPathFinder
{
  screen_width: f32,
  screen_height: f32,
  node_radius: f32,
  gap_size: f32,
  window_edge_offset: f32,
  ui_width: f32,
  /// How many cells are on the x axis
  grid_width: i32,
  /// How many cells are on the y axis
  grid_height: i32,

  grid: Vec<Cell>,

  mouse: Pos2,

  toggle_value: bool,
}

impl GridPathFinder
{
  /// Called once before the first frame.
  pub fn new(cc: &CreationContext<'_>) -> Self
  {
    // This is also where you can customize the look and feel of egui using
    // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

    cc.egui_ctx.set_visuals(Visuals
    {
      window_shadow: Shadow::NONE,
      hyperlink_color: Color32::from_rgb(128, 0, 255),
      ..Default::default()
    });

    return Default::default();
  }

  fn calc_x_cell_count(&mut self)
  {
    self.grid_width = ((self.screen_width - 2.*self.window_edge_offset - self.ui_width) / (2.*self.node_radius + self.gap_size)).floor() as i32 + 1;
  }

  fn calc_y_cell_count(&mut self)
  {
    self.grid_height = ((self.screen_height - 2.*self.window_edge_offset) / (2.*self.node_radius + self.gap_size)).floor() as i32 + 1;
  }

  fn update_cell_count(&mut self)
  {
    self.calc_x_cell_count();
    self.calc_y_cell_count();

    let new_cell_count = (self.grid_width * self.grid_height) as usize;

    if new_cell_count != self.grid.len()
    {
      self.grid = vec![Cell::Node; new_cell_count];
    }
  }

  fn get_mut(&mut self, coords: (i32, i32)) -> &mut Cell
  {
    let x = coords.0;
    let y = coords.1;
    let index = x + (y * self.grid_width);
    return self.grid.get_mut(index as usize).unwrap();
  }

  fn get(&mut self, coords: (i32, i32)) -> &Cell
  {
    let x = coords.0;
    let y = coords.1;
    let index = x + (y * self.grid_width);
    // debug stuff
    // println!("x: {x}    y: {y}");
    // println!("grid_width: {}    grid_height: {}", self.grid_width, self.grid_height);
    // println!("cell count: {}    get index: {}", (self.grid_width * self.grid_height), index as usize);
    return self.grid.get(index as usize).unwrap();
  }

  fn is_mouse_in_square(&self, x: f32, y: f32) -> bool
  {
    // `x` and `y` represent the centre of the square. `self.node_radius` represents the length of one side.
    // Utilise self.mouse

    if
      self.mouse.x > x - (self.node_radius + (0.5*self.gap_size)) &&
      self.mouse.y > y - (self.node_radius + (0.5*self.gap_size)) &&
      self.mouse.x < x + (self.node_radius + (0.5*self.gap_size)) &&
      self.mouse.y < y + (self.node_radius + (0.5*self.gap_size))
    { return true; }
    return false;
  }
}

impl Default for GridPathFinder
{
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
      grid_width: 0,
      grid_height: 0,
      grid: vec![],
      mouse: Pos2::default(),
      toggle_value: false,
    };

    default.calc_x_cell_count();
    default.calc_y_cell_count();

    default.grid = vec![Cell::Node; (default.grid_width * default.grid_height) as usize];

    return default;
  }
}
