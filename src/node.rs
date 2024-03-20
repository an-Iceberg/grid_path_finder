pub(crate) enum Cell
{ Node(Node), Obstacle }

pub(crate) struct Node
{
  parent: Direction,
  distance: f32,
}

pub(crate) enum Direction
{ None, North, NorthWest, West, SouthWest, South, SouthEast, East, NorthEast }
