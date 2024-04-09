mod ui;
mod node;
mod utils;
mod grid;

use egui_macroquad::macroquad::telemetry::disable;
use grid::Grid;
use macroquad::prelude::*;
use utils::offset_vec;

use crate::utils::offset;

fn window_configuration() -> Conf
{
  return Conf
  {
    window_title: "Grid Path Finder".to_string(),
    window_width: 1600,
    window_height: 900,
    fullscreen: false,
    window_resizable: false,
    sample_count: 8,
    ..Conf::default()
  };
}

pub(crate) const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub(crate) const AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");

pub(crate) const BG_COLOR: u32 = 0x786478;
pub(crate) const GRID_BG_COLOR: u32 = 0xc0a6c0;
pub(crate) const START_COLOR: u32 = 0xff8000;
pub(crate) const PATH_COLOR: u32 = 0xff8000;
pub(crate) const END_COLOR: u32 = 0x00ff00;
pub(crate) const VISITED_COLOR: u32 = 0x804080;
pub(crate) const UNVISITED_COLOR: u32 = 0xff80ff;
pub(crate) const GRID_WIDTH: usize = 116;
pub(crate) const GRID_HEIGHT: usize = 74;
pub(crate) const OFFSET: f32 = 12.;
pub(crate) const RADIUS: f32 = 4.;
pub(crate) const DIRECTIONS: [Vec2; 8] = [
  Vec2::new(-1., 0.),
  Vec2::new(-1., 1.),
  Vec2::new(0., 1.),
  Vec2::new(1., 1.),
  Vec2::new(1., 0.),
  Vec2::new(1., -1.),
  Vec2::new(0., -1.),
  Vec2::new(-1., -1.),
];

// TODO: don't draw unvisited nodes
// TODO: make unvisited_nodes a data field of the Grid struct
// TODO: tests

#[macroquad::main(window_configuration)]
async fn main()
{
  disable();

  let mut grid = Grid::new();
  let mut mouse_mode = MouseMode::Obstacle;
  let mut animate = true;
  let mut ratio: f64 = 0.5;
  let mut speed = 1;

  // https://www.youtube.com/watch?v=9W8hNdEUFbc
  loop
  {
    // TODO: extract all functionality into their own functions
    // Process keys, mouse etc.

    if grid.finding_path()
    {
      if animate
      {
        for _ in 1..=speed
        {
          if grid.has_unvisited_nodes() { grid.a_star_step(); }
        }
      }
      else
      {
        while grid.has_unvisited_nodes() { grid.a_star_step(); }
      }
    }

    clear_background(Color::from_hex(BG_COLOR));

    draw_rectangle(0., 0., screen_width() - 196., screen_height(), Color::from_hex(GRID_BG_COLOR));

    for x in 0..GRID_WIDTH
    {
      for y in 0..GRID_HEIGHT
      {
        let coordinates = Vec2::new(x as f32, y as f32);
        // let coords_minus_one = coordinates - Vec2::ONE;

        let node = grid.node_at(&coordinates);
        if node.is_obstacle { draw_circle(offset(coordinates.x), offset(coordinates.y), RADIUS, BLACK); }
        if !node.is_obstacle && node.visited { draw_circle(offset(x as f32), offset(y as f32), RADIUS, Color::from_hex(VISITED_COLOR)); }
        // ToDo {optional}: draw thin line from node to parent
        // if let Some(parent) = node.parent
        // {
        //   draw_line(offset(coordinates.x), offset(coordinates.y), offset(parent.x), offset(parent.y), 1., RED);
        // }

        let mouse = Vec2::new(mouse_position().0, mouse_position().1);
        if utils::is_point_in_square(&mouse, &offset_vec(&coordinates), RADIUS)
        {
          // Outlining hovered node
          if !node.is_obstacle
          { draw_circle_lines(offset(coordinates.x), offset(coordinates.y), RADIUS, 1., BLACK); }

          if is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_down(MouseButton::Left)
          {
            match mouse_mode
            {
              MouseMode::Node => node.set_to_node(),
              MouseMode::Obstacle => node.set_to_obstacle(),
              MouseMode::Start => grid.set_start(&coordinates),
              MouseMode::End => grid.set_end(&coordinates)
            }
          }
        }
      }
    }

    // TODO: improve drawing
    // TODO: use sets for visited/unvisited nodes
    // TODO: use separatedata structures for algorithm and painting

    // TODO: paint the path from current_node, if some, else from grid.end
    // Paints the path
    if let Some(path) = grid.get_current_path()
    {
      path.iter().zip(path.iter().skip(1))
        .for_each(|(a, b)|
        {
          draw_line(offset(a.x), offset(a.y), offset(b.x), offset(b.y), RADIUS * 2., Color::from_hex(PATH_COLOR));
          draw_circle(offset(a.x), offset(a.y), RADIUS, Color::from_hex(PATH_COLOR));
        });
    }

    if let Some(start) = grid.get_start()
    {
      draw_circle(offset(start.x), offset(start.y), 4., Color::from_hex(START_COLOR));
    }

    if let Some(end) = grid.get_end()
    {
      draw_circle(offset(end.x), offset(end.y), 4., Color::from_hex(END_COLOR));
    }

    ui::paint(
      &mut mouse_mode,
      &mut grid,
      &mut animate,
      &mut ratio,
      &mut speed,
    );

    // Draw things before egui

    egui_macroquad::draw();

    // Draw things after egui

    next_frame().await;
  }
}

#[derive(PartialEq, Eq)]
pub(crate) enum MouseMode
{ Node, Obstacle, Start, End }
