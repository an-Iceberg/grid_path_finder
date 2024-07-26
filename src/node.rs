use macroquad::math::Vec2;

// FIX: the interaction between the grid and cells and nodes is clunky, find a better solution
#[derive(Clone, Copy)]
pub(crate) struct Node
{
  pub(crate) is_obstacle: bool,
  pub(crate) parent: Option<Vec2>, // Note: change this to an Option<Vec2>?
  pub(crate) distance: Option<f32>, // Note: change this to an Option<f32>?
  pub(crate) heuristic: Option<f32>, // Note: change this to an Option<f32>?
  pub(crate) visited: bool,
}

// #[derive(Clone, Copy, PartialEq, Eq)]
// pub(crate) enum Direction
// { None, North, NorthWest, West, SouthWest, South, SouthEast, East, NorthEast }

impl Node
{
  pub fn new() -> Self
  {
    Node
    {
      is_obstacle: false,
      parent: None,
      distance: None,
      heuristic: None,
      visited: false
    }
  }

  pub fn clear(&mut self)
  {
    *self = Node::new();
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
