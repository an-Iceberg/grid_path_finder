use rand::{rngs::ThreadRng, Rng};

use crate::{node::{Cell, Node}, GRID_HEIGHT, GRID_WIDTH};

pub(crate) struct Grid
{
  grid: [Cell; GRID_WIDTH as usize * GRID_HEIGHT as usize],
  start: Option<(usize, usize)>,
  end: Option<(usize, usize)>,
  rng: ThreadRng,
}

impl Grid
{
  pub fn new() -> Self
  {
    return Grid
    {
      grid: [Cell::Node(Node::new()); GRID_WIDTH as usize * GRID_HEIGHT as usize],
      start: None,
      end: None,
      rng: rand::thread_rng()
    };
  }

  pub fn get_cell(&self, x: usize, y: usize) -> Cell
  {
    return self.grid[y * GRID_WIDTH as usize + x];
  }

  pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell)
  {
    self.grid[y * GRID_WIDTH as usize + x] = cell;
  }

  pub fn set_start(&mut self, x: usize, y: usize)
  {
    self.start = Some((x, y));
    self.set_cell(x, y, Cell::Node(Node::new()));
  }

  pub fn set_end(&mut self, x: usize, y: usize)
  {
    self.end = Some((x, y));
    self.set_cell(x, y, Cell::Node(Node::new()));
  }

  pub fn clear_start(&mut self)
  {
    self.start = None;
  }

  pub fn clear_end(&mut self)
  {
    self.end = None;
  }

  pub fn get_start(&self) -> Option<(usize, usize)>
  {
    return self.start;
  }

  pub fn get_end(&self) -> Option<(usize, usize)>
  {
    return self.end;
  }

  pub fn clear(&mut self)
  {
    self.grid = [Cell::Node(Node::new()); GRID_WIDTH as usize * GRID_HEIGHT as usize];
    self.start = None;
    self.end = None;
  }

  pub fn set_random_obstacles(&mut self, ratio: f64)
  {
    self.grid.iter_mut()
      .for_each(|cell|
      {
        if self.rng.gen_bool(ratio)
        { *cell = Cell::Obstacle; }
        else
        { *cell = Cell::Node(Node::new()); }
      });
  }

  pub fn a_star_step(&mut self)
  {
  }
}
