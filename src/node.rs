use macroquad::math::Vec2;

// FIX: the interaction between the grid and cells and nodes is clunky, find a better solution
#[derive(Clone, Copy)]
pub(crate) struct Node
{
  pub(crate) is_obstacle: bool,
  pub(crate) parent: Vec2, // TODO: replace all directions and (usize, usize) with Vec2
  pub(crate) distance: f32,
  pub(crate) heuristic: f32,
  pub(crate) visited: bool,
}

// #[derive(Clone, Copy, PartialEq, Eq)]
// pub(crate) enum Direction
// { None, North, NorthWest, West, SouthWest, South, SouthEast, East, NorthEast }

impl Node
{
  pub fn new() -> Self
  {
    return Node
    {
      is_obstacle: false,
      parent: Vec2::new(0., 0.),
      distance: f32::MAX,
      heuristic: f32::MAX,
      visited: false
    };
  }

  pub fn clear(&mut self)
  {
    self.is_obstacle = false;
    self.parent = Vec2::new(0., 0.);
    self.distance = f32::MAX;
    self.heuristic = f32::MAX;
    self.visited = false;
  }

  pub fn set_to_node(&mut self)
  {
    self.clear();
    self.is_obstacle = false;
  }

  pub fn set_to_obstacle(&mut self)
  {
    self.clear();
    self.is_obstacle = true;
  }
}
