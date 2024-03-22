use macroquad::math::{Vec2, Vec2Swizzles};
use rand::{rngs::ThreadRng, Rng};

use crate::{node::Node, utils::distance, DIRECTIONS, GRID_HEIGHT, GRID_WIDTH, OFFSET};

pub(crate) struct Grid
{
  grid: [Node; GRID_WIDTH * GRID_HEIGHT],
  start: Option<Vec2>,
  end: Option<Vec2>,
  path_length: f32,
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
      path_length: f32::MAX,
      rng: rand::thread_rng()
    };
  }

  pub fn node_at(&mut self, pos: Vec2) -> &mut Node
  {
    return &mut self.grid[(pos.y * GRID_WIDTH as f32 + pos.x) as usize];
  }

  pub fn set_start(&mut self, pos: Vec2)
  {
    self.node_at(pos).set_to_node();
    self.start = Some(pos);
  }

  pub fn set_end(&mut self, pos: Vec2)
  {
    self.node_at(pos).set_to_node();
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
    self.grid.iter_mut()
      .for_each(|node|
      {
        if self.rng.gen_bool(ratio) { node.set_to_obstacle(); }
        else { node.set_to_node(); }
      });
  }

  pub fn clear_path_data(&mut self)
  {
    self.grid.iter_mut()
      .filter(|node| !node.is_obstacle)
      .for_each(|node| node.clear());

    if let Some(start) = self.start { self.node_at(start).distance = 0.; }
  }

  pub fn has_neighbour(&self, pos: Vec2, direction: Vec2) -> bool
  {
    let neighbour = pos + direction;

    if neighbour.x < 0. ||
      neighbour.y < 0. ||
      neighbour.x > GRID_WIDTH as f32 ||
      neighbour.y > GRID_HEIGHT as f32
    { return false; }

    print!(" {:?} ", neighbour);
    return true;
  }

  pub fn get_unvisited_neighbours(&mut self, pos: Vec2) -> Vec<Vec2> // Vec<&mut Node>
  {
    let mut neighbours = vec![];

    DIRECTIONS.iter().for_each(|direction|
    {
      if self.has_neighbour(pos, *direction)
      {
        let neighbour = self.node_at(pos + *direction);
        if !neighbour.is_obstacle && !neighbour.visited { neighbours.push(pos + *direction); }
      }
    });

    return neighbours;
  }

  /// A* algorithm
  pub fn a_star_step(&mut self, unvisited_nodes: &mut Vec<Vec2>, finding_path: &mut bool)
  {
    // println!("unvisited_nodes: {:?}", unvisited_nodes);
    // Determine the heuristic for each node
    unvisited_nodes.iter().for_each(|at|
    {
      let end = self.get_end().unwrap();
      self.node_at(*at).heuristic = distance(*at*12.+OFFSET, end*12.+OFFSET);
    });

    // Sort the unvisited nodes by their heuristic (best first)
    unvisited_nodes.sort_by(|a, b|
    {
      let a_h = self.node_at(*a).heuristic;
      let b_h = self.node_at(*b).heuristic;
      return a_h.partial_cmp(&b_h).unwrap();
    });

    // Remove visited nodes
    unvisited_nodes.retain(|at| !self.node_at(*at).visited);
    // unvisited_nodes.dedup();

    if unvisited_nodes.is_empty() { return; }

    let current_node_coordinates = *unvisited_nodes.first().unwrap();
    self.node_at(current_node_coordinates).visited = true;
    let current_distance = self.node_at(current_node_coordinates).distance;

    // println!("current_node_coordinates: {:?}", current_node_coordinates);

    // Visiting all unvisited neighbours of the current node
    for neighbour_coordinates in self.get_unvisited_neighbours(current_node_coordinates)
    {
      let local_distance = current_distance + distance(current_node_coordinates*12.+OFFSET, neighbour_coordinates*12.+OFFSET);

      if local_distance > self.path_length { break; }

      if neighbour_coordinates == self.end.unwrap()
      {
        // self.path_length = self.node_at(self.end.unwrap()).distance;
        // self.node_at(neighbour_coordinates).visited = false;
        self.node_at(self.get_end().unwrap()).parent = current_node_coordinates;
        unvisited_nodes.clear();
        *finding_path = false;
        return;
      }

      let neighbour = self.node_at(neighbour_coordinates);

      unvisited_nodes.push(neighbour_coordinates);

      // Core of the algorithm: update node internals, if shorter path is possible
      if neighbour.distance > local_distance
      {
        neighbour.distance = local_distance;
        neighbour.parent = current_node_coordinates;
      }
    }
  }
}
