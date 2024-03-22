use macroquad::math::Vec2;
use rand::{rngs::ThreadRng, Rng};

use crate::{node::{Direction, Node}, utils::distance, DIRECTIONS, GRID_HEIGHT, GRID_WIDTH};

pub(crate) struct Grid
{
  grid: [Node; GRID_WIDTH * GRID_HEIGHT],
  start: Option<Vec2>,
  end: Option<Vec2>,
  rng: ThreadRng,
}

impl Grid
{
  pub fn new() -> Self
  {
    return Grid
    {
      grid: [Node::new(); GRID_WIDTH * GRID_HEIGHT],
      start: None,
      end: None,
      rng: rand::thread_rng()
    };
  }

  pub fn node_at(&mut self, pos: Vec2) -> &mut Node
  {
    return &mut self.grid[(pos.y * GRID_WIDTH as f32 + pos.x) as usize];
  }

  // pub fn set_to_node(&self, x: usize, y: usize)
  // {
  //   self.node_at(x, y).set_to_node()
  // }

  // pub fn set_to_obstacle(&self, x: usize, y: usize)
  // {
  //   self.node_at(x, y).set_to_obstacle();
  // }

  pub fn set_start(&mut self, mut pos: Vec2)
  {
    self.node_at(pos).set_to_node();
    pos += Vec2::new(1., 1.);
    self.start = Some(pos);
  }

  pub fn set_end(&mut self, mut pos: Vec2)
  {
    self.node_at(pos).set_to_node();
    pos += Vec2::new(1., 1.);
    self.end = Some(pos);
  }

  pub fn clear_start(&mut self)
  {
    self.start = None;
  }

  pub fn clear_end(&mut self)
  {
    self.end = None;
  }

  pub fn get_start(&self) -> Option<Vec2>
  {
    return self.start;
  }

  pub fn get_end(&self) -> Option<Vec2>
  {
    return self.end;
  }

  pub fn clear(&mut self)
  {
    self.grid = [Node::new(); GRID_WIDTH * GRID_HEIGHT];
    self.start = None;
    self.end = None;
  }

  pub fn set_random_obstacles(&mut self, ratio: f64)
  {
    // self.grid.iter_mut()
    //   .for_each(|cell|
    //   {
    //     if self.rng.gen_bool(ratio)
    //     { *cell = Cell::Obstacle; }
    //     else
    //     { *cell = Cell::Node(Node::new()); }
    //   });

    self.grid.iter_mut()
      .for_each(|node|
      {
        if self.rng.gen_bool(ratio)
        { node.set_to_obstacle(); }
        else
        { node.set_to_node(); }
      });
  }

  pub fn clear_path_data(&mut self)
  {
    // self.grid.iter_mut()
    //   .filter(|cell| cell.is_node())
    //   .map(|cell| cell.unwrap())
    //   .for_each(|mut node| {
    //     node.visited = false;
    //     node.heuristic = f32::MAX;
    //     node.distance = f32::MAX;
    //   });

    self.grid.iter_mut()
      .filter(|node| !node.is_obstacle)
      .for_each(|node| node.clear());

    if let Some(start) = self.start
    { self.node_at(start).distance = 0.; }
  }

  pub fn has_neighbour(&self, pos: Vec2, direction: Vec2) -> bool
  {
    match direction
    {
      Vec2{ x: 1., y: 1. } => return true,
    };
  }

  // TODO: refactor this
  pub fn get_unvisited_neighbours(&mut self, pos: Vec2) -> Vec<Vec2> // Vec<&mut Node>
  {
    let mut neighbours = vec![];

    // let directions = vec![
    //   Vec2::new(-1., 0.),
    //   Vec2::new(-1., 1.),
    //   Vec2::new(0., 1.),
    //   Vec2::new(1., 1.),
    //   Vec2::new(1., 0.),
    //   Vec2::new(1., -1.),
    //   Vec2::new(0., -1.),
    //   Vec2::new(-1., -1.),
    // ];

    DIRECTIONS.iter().for_each(|direction|
    {
      if self.has_neighbour(pos, *direction)
      {
        let neighbour = self.node_at(pos);
        if !neighbour.is_obstacle && !neighbour.visited { neighbours.push(pos + *direction); }
      }
    });

    return neighbours;
  }

  pub fn a_star_step(&mut self, unvisited_nodes: &mut Vec<(usize, usize)>)
  {
    // Determine the heuristic for each node
    unvisited_nodes.iter().for_each(|at|
    {
      let end = self.get_end().unwrap();
      self.node_at(at.0, at.1).heuristic = distance(
        (at.0 * 12) as f32, (at.1 * 12) as f32,
        (end.0 + 12) as f32, (end.1 * 12) as f32);
    });

    // Sort the unvisited nodes by their heuristic (best first)
    unvisited_nodes.sort_by(|a, b|
    {
      let a_h = self.node_at(a.0, a.1).heuristic;
      let b_h = self.node_at(b.0, b.1).heuristic;
      return a_h.partial_cmp(&b_h).unwrap();
    });

    // Remove visited nodes
    unvisited_nodes.retain(|at| !self.node_at(at.0, at.1).visited);

    if unvisited_nodes.is_empty() { return; }

    let current_node_coords = unvisited_nodes.swap_remove(0);
    let mut current_node = *self.node_at(current_node_coords.0, current_node_coords.1);

    current_node.visited = true;

    for neighbour_coordinates in self.get_unvisited_neighbours(current_node_coords.0, current_node_coords.1)
    {
      let neighbour = self.node_at(neighbour_coordinates.0, neighbour_coordinates.1);

      if neighbour.distance > current_node.distance + distance(
        (current_node_coords.0 * 12) as f32,
        (current_node_coords.1 * 12) as f32,
        (neighbour_coordinates.0 * 12) as f32,
        (neighbour_coordinates.1 * 12) as f32)
      {
        neighbour.distance = current_node.distance + distance(
        (current_node_coords.0 * 12) as f32,
        (current_node_coords.1 * 12) as f32,
        (neighbour_coordinates.0 * 12) as f32,
        (neighbour_coordinates.1 * 12) as f32);

        neighbour.parent = match (neighbour_coordinates.0 as i16 - current_node_coords.0 as i16, neighbour_coordinates.1 as i16 - current_node_coords.1 as i16)
        {
          (1, 1) => Direction::SouthEast,
          (-1, -1) => Direction::NorthWest,
          (1, -1) => Direction::SouthWest,
          (-1, 1) => Direction::NorthEast,
          (1, 0) => Direction::South,
          (-1, 0) => Direction::North,
          (0, 1) => Direction::East,
          (0, -1) => Direction::West,
          (_, _) => Direction::None
        }
      }
    }

    // TODO: iterate thru each of the node's neighbours
  }
}
