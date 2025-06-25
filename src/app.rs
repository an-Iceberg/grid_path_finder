use eframe::CreationContext;
use egui::Pos2;
#[allow(unused_imports)] // TODO: remove when time comes
use egui::{pos2, vec2, Align2, CentralPanel, Color32, Context, RichText, Shadow, Shape, SidePanel, Ui, Visuals, Window};
use crate::{node::Cell, CELL_SIZE, GAP, PADDING};

mod ui;

pub struct GridPathFinder
{
  space_width: f32,
  space_height: f32,
  /// How many cells are on the x axis
  grid_width: i32,
  /// How many cells are on the y axis
  grid_height: i32,
  grid: Vec<Cell>,
  toggle_value: bool, // dbg
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

  fn update_grid_width(&mut self, new_width: f32)
  {
    // Only update grid width if it actually changed
    if new_width != self.space_width
    {
      self.space_width = new_width;
      self.grid_width = ((new_width - 2.*PADDING) / (CELL_SIZE + GAP)).floor() as i32;
      self.update_cell_count();
    }
  }

  fn update_grid_height(&mut self, new_height: f32)
  {
    // Only update grid width if it actually changed
    if new_height != self.space_height
    {
      self.space_height = new_height;
      self.grid_height = ((new_height - 2.*PADDING) / (CELL_SIZE + GAP)).floor() as i32;
      self.update_cell_count();
    }
  }

  fn update_cell_count(&mut self)
  {
    let new_cell_count = (self.grid_width * self.grid_height) as usize;
    if new_cell_count != self.grid.len()
    { self.grid = vec![Cell::Node; new_cell_count]; }
  }

  fn get_mut(&mut self, coords: (i32, i32)) -> &mut Cell
  {
    let x = coords.0;
    let y = coords.1;
    let index = x + (y * self.grid_width);
    return self.grid.get_mut(index as usize).unwrap();
  }

  fn get(&mut self, coords: (i32, i32)) -> Option<&Cell>
  {
    let x = coords.0;
    let y = coords.1;
    let index = x + (y * self.grid_width);
    // debug stuff
    // println!("x: {x}    y: {y}");
    // println!("grid_width: {}    grid_height: {}", self.grid_width, self.grid_height);
    // println!("cell count: {}    get index: {}", (self.grid_width * self.grid_height), index as usize);
    return self.grid.get(index as usize);
  }
}

impl Default for GridPathFinder
{
  fn default() -> Self
  {
    let mut default = Self
    {
      space_width: 800.,
      space_height: 600.,
      grid_width: 0,
      grid_height: 0,
      grid: vec![Cell::Node; 300_usize],
      toggle_value: false,
    };

    default.update_grid_height(600.);
    default.update_grid_width(800.);
    default.update_cell_count();

    return default;
  }
}
