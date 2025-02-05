#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate)  enum Cell
{
  Obstacle,
  Node,
}

impl Cell
{
  pub(crate) fn toggle(&mut self)
  {
    if *self == Cell::Obstacle { *self = Cell::Node }
    else { *self = Cell::Obstacle }
  }

  pub(crate) fn is_node(&self) -> bool
  {
    if *self == Cell::Obstacle { return false;}
    else { return true; }
  }
}
