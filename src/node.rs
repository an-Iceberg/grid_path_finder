#[derive(Clone, Copy)]
pub(crate) enum Cell
{ Node(Node), Obstacle }

impl Cell
{
  pub fn is_obstacle(&self) -> bool
  {
    return matches!(*self, Cell::Obstacle);
  }

  pub fn is_node(&self) -> bool
  {
    return matches!(*self, Cell::Node(_));
  }
}

#[derive(Clone, Copy)]
pub(crate) struct Node
{
  pub(crate) parent: Direction,
  pub(crate) distance: f32,
  pub(crate) heuristic: f32,
  pub(crate) visited: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction
{ None, North, NorthWest, West, SouthWest, South, SouthEast, East, NorthEast }

impl Node
{
  pub fn new() -> Self
  {
    return Node { parent: Direction::None, distance: 0., heuristic: 0., visited: false };
  }
}
