mod ui;
mod node;

use egui_macroquad::macroquad::telemetry::disable;
use macroquad::{prelude::*, ui::DrawList};

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

#[macroquad::main(window_configuration)]
async fn main()
{
  disable();

  loop
  {
    // Process keys, mouse etc.

    for x in 1..=119
    {
      for y in 1..=74
      {
        draw_circle((x * 12) as f32, (y * 12) as f32, 4., Color::from_hex(0x00ffff));
      }
    }

    draw_circle(60., 60., 5., ORANGE);
    draw_circle(120., 120., 5., ORANGE);
    draw_circle(120., 320., 5., ORANGE);
    draw_circle(400., 60., 5., ORANGE);

    draw_line(60., 60., 120., 120., 10., ORANGE);
    draw_line(120., 120., 120., 320., 10., ORANGE);
    draw_line(120., 120., 400., 60., 10., ORANGE);

    draw_hexagon(500., 500., 10., 0., true, ORANGE, ORANGE);
    draw_hexagon(550., 600., 10., 0., true, ORANGE, ORANGE);

    draw_line(500., 500., 550., 600., 20., ORANGE);

    ui::paint();

    // Draw things before egui

    egui_macroquad::draw();

    // Draw things after egui

    next_frame().await;
  }
}
