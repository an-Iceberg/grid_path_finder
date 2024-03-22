mod ui;
mod node;
mod utils;
mod grid;

use egui_macroquad::macroquad::telemetry::disable;
use grid::Grid;
use macroquad::prelude::*;

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

#[macroquad::main(window_configuration)]
async fn main()
{
  disable();

  let mut grid = Grid::new();
  let mut mouse_mode = MouseMode::Obstacle;
  let mut animate = true;
  let mut finding_path = true;
  let mut ratio: f64 = 0.5;
  let mut speed = 1;

  let mut unvisited_nodes: Vec<Vec2> = vec![];
  let mut path: Vec<Vec2> = vec![];

  // https://www.youtube.com/watch?v=9W8hNdEUFbc
  loop
  {
    // Process keys, mouse etc.
    if animate
    {
      for _ in 1..=speed
      {
        if !unvisited_nodes.is_empty()
        { grid.a_star_step(&mut unvisited_nodes); }
      }
    }
    else
    {
      while !unvisited_nodes.is_empty()
      { grid.a_star_step(&mut unvisited_nodes); }
    }

    clear_background(Color::from_hex(BG_COLOR));

    draw_rectangle(0., 0., screen_width() - 200., screen_height(), Color::from_hex(GRID_BG_COLOR));

    for x in 1..=GRID_WIDTH
    {
      for y in 1..=GRID_HEIGHT
      {
        let node = grid.node_at(x-1, y-1);
        if node.is_obstacle { draw_circle((x * 12) as f32, (y * 12) as f32, RADIUS, BLACK); }
        if !node.is_obstacle && node.visited { draw_circle((x * 12) as f32, (y * 12) as f32, RADIUS, Color::from_hex(VISITED_COLOR)); }

        if utils::is_point_in_square(mouse_position().0, mouse_position().1, (x * 12) as f32, (y * 12) as f32, RADIUS)
        {
          // Outlining hovered node
          if !node.is_obstacle
          { draw_circle_lines((x * 12) as f32, (y * 12) as f32, RADIUS, 1., BLACK); }

          if is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_down(MouseButton::Left)
          {
            match mouse_mode
            {
              MouseMode::Node => node.set_to_node(),
              MouseMode::Obstacle => node.set_to_obstacle(),
              // FIX: these panic for the last grid row
              MouseMode::Start => grid.set_start(x-1, y-1),
              MouseMode::End => grid.set_end(x-1, y-1)
            }
          }
        }
      }
    }

    // Paints the path
    if let (Some(start), Some(end)) = (grid.get_start(), grid.get_end())
    {
      draw_circle((start.0*12) as f32, (start.1*12) as f32, RADIUS, Color::from_hex(PATH_COLOR));
      let current_node = grid.node_at(start.0, start.1).parent;
    }

    if let Some((x, y)) = grid.get_start()
    {
      draw_circle((x*12) as f32, (y*12) as f32, 4., Color::from_hex(START_COLOR));
    }

    if let Some((x, y)) = grid.get_end()
    {
      draw_circle((x*12) as f32, (y*12) as f32, 4., Color::from_hex(END_COLOR));
    }

    // draw_circle(60., 60., 5., ORANGE);
    // draw_circle(120., 120., 5., ORANGE);
    // draw_circle(120., 320., 5., ORANGE);
    // draw_circle(400., 60., 5., ORANGE);

    // draw_line(60., 60., 120., 120., 10., ORANGE);
    // draw_line(120., 120., 120., 320., 10., ORANGE);
    // draw_line(120., 120., 400., 60., 10., ORANGE);

    // draw_hexagon(500., 500., 10., 0., true, ORANGE, ORANGE);
    // draw_hexagon(550., 600., 10., 0., true, ORANGE, ORANGE);

    // draw_line(500., 500., 550., 600., 20., ORANGE);

    ui::paint(
      &mut mouse_mode,
      &mut grid,
      &mut animate,
      &mut finding_path,
      &mut ratio,
      &mut unvisited_nodes,
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
