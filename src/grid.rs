use macroquad::math::Vec2;
use rand::{rngs::ThreadRng, Rng};
use crate::{node::Node, utils::{distance, offset_vec}, DIRECTIONS, GRID_HEIGHT, GRID_WIDTH};

pub(crate) struct Grid
{
  // Note: Rc<[Node]> ?
  grid: [Node; GRID_WIDTH * GRID_HEIGHT],
  start: Option<Vec2>,
  end: Option<Vec2>,
  path_length: f32,
  unvisited_nodes: Vec<Vec2>,
  rng: ThreadRng,
  finding_path: bool,
  // todo: found_path: bool
}

// TODO: convert all Vec2 to &Vec2
impl Grid
{
  #[allow(clippy::needless_return)]
  pub fn new() -> Self
  {
    return Grid
    {
      grid: [Node::new(); GRID_WIDTH * GRID_HEIGHT],
      start: None,
      end: None,
      path_length: f32::MAX,
      unvisited_nodes: vec![],
      rng: rand::thread_rng(),
      finding_path: false,
    };
  }

  #[allow(clippy::needless_return)]
  pub fn node_at(&mut self, pos: &Vec2) -> &mut Node
  {
    return &mut self.grid[(pos.y * GRID_WIDTH as f32 + pos.x) as usize];
  }

  #[allow(clippy::needless_return)]
  pub fn set_start(&mut self, pos: &Vec2)
  {
    self.grid[Self::pos_to_idx(pos)].set_to_node();
    self.start = Some(*pos);
  }

  #[allow(clippy::needless_return)]
  pub fn set_end(&mut self, pos: &Vec2)
  {
    self.node_at(pos).set_to_node();
    self.end = Some(*pos);
  }

  #[allow(clippy::needless_return)]
  pub fn clear_start(&mut self)
  {
    if let Some(start) = self.start
    { self.grid[Self::pos_to_idx(&start)].set_to_node(); }

    self.start = None;

    self.clear_path_data();
  }

  #[allow(clippy::needless_return)]
  pub fn clear_end(&mut self)
  {
    if let Some(end) = self.end
    { self.grid[Self::pos_to_idx(&end)].set_to_node(); }

    self.end = None;

    self.clear_path_data();
  }

  #[allow(clippy::needless_return)]
  pub fn get_start(&self) -> Option<Vec2>
  {
    return self.start;
  }

  #[allow(clippy::needless_return)]
  pub fn get_end(&self) -> Option<Vec2>
  {
    return self.end;
  }

  #[allow(clippy::needless_return)]
  pub fn clear(&mut self)
  {
    self.grid = [Node::new(); GRID_WIDTH * GRID_HEIGHT];
    self.start = None;
    self.end = None;
    self.unvisited_nodes.clear();
    self.finding_path = false;
    self.path_length = f32::MAX;
  }

  #[allow(clippy::needless_return)]
  pub fn find_path(&mut self)
  {
    self.finding_path = true;
    self.grid[Self::pos_to_idx(&self.start.unwrap())].distance = Some(0.);
  }

  #[allow(clippy::needless_return)]
  pub fn get_current_node(&self) -> Option<&Vec2>
  {
    return self.unvisited_nodes.last();
  }

  #[allow(clippy::needless_return)]
  pub fn finding_path(&self) -> bool
  {
    return self.finding_path;
  }

  #[allow(clippy::needless_return)]
  pub fn get_parent(&self, at: &Vec2) -> Option<Vec2>
  {
    return self.grid[Self::pos_to_idx(at)].parent;
  }

  #[allow(clippy::needless_return)]
  pub fn get_current_path(&self) -> Option<Vec<Vec2>>
  {
    // ToDo: Some, if current_path != empty
    // If current_path is empty, then end.parent Some
    // Else -> None
    let mut current_path = vec![];

    match self.unvisited_nodes.last()
    {
      Some(current) => current_path.push(*current),
      None => return None,
    }

    loop
    {
      if let Some(last) = current_path.last()
      {
        if let Some(parent) = self.grid[Self::pos_to_idx(last)].parent
        { current_path.push(parent); }
        else { break; }
      }
    }

    return Some(current_path);
  }

  #[allow(clippy::needless_return)]
  pub fn set_random_obstacles(&mut self, ratio: f64)
  {
    self.grid.iter_mut()
      .for_each(|node|
      {
        if self.rng.gen_bool(ratio) { node.set_to_obstacle(); }
        else { node.set_to_node(); }
      });
  }

  #[allow(clippy::needless_return)]
  pub fn clear_path_data(&mut self)
  {
    self.grid.iter_mut()
      .filter(|node| !node.is_obstacle) // Note: is this filter really necessary?
      .for_each(|node| node.clear());

    self.unvisited_nodes.clear();

    if let Some(start) = self.start
    {
      self.grid[Self::pos_to_idx(&start)].distance = None;
      self.unvisited_nodes.push(start);
    }

    self.finding_path = false;
    self.path_length = f32::MAX;
    // if let Some(start) = self.start { self.node_at(&start).distance = 0.; }
  }

  #[allow(clippy::needless_return)]
  pub fn has_neighbour(&self, pos: &Vec2, direction: &Vec2) -> bool
  {
    let neighbour = *pos + *direction;

    if neighbour.x < 0. ||
      neighbour.y < 0. ||
      neighbour.x > GRID_WIDTH as f32 ||
      neighbour.y > GRID_HEIGHT as f32
    { return false; }

    return true;
  }

  #[allow(clippy::needless_return)]
  pub fn get_unvisited_neighbours(&mut self, pos: &Vec2) -> Vec<Vec2> // Vec<&mut Node>
  {
    let mut neighbours = vec![];

    DIRECTIONS.iter()
      .filter(|dir| self.has_neighbour(pos, dir))
      .map(|dir| (self.grid[Self::pos_to_idx(&(*pos + *dir))], dir))
      .filter(|(neighbour, _)| !neighbour.is_obstacle && !neighbour.visited)
      .for_each(|(_, dir)| neighbours.push(*pos + *dir));

    return neighbours;
  }

  #[allow(clippy::needless_return)]
  pub fn has_unvisited_nodes(&mut self) -> bool
  {
    return !self.unvisited_nodes.is_empty();
  }

  #[allow(clippy::needless_return)]
  fn pos_to_idx(pos: &Vec2) -> usize
  {
    return (pos.y * GRID_WIDTH as f32 + pos.x) as usize;
  }

  // FIX: don't use cells outside the grid
  // Fix: make it go in directions other than bottom left
  /// A* algorithm
  #[allow(clippy::needless_return)]
  pub fn a_star_step(&mut self)
  {
    if !self.finding_path { return; }

    let Some(end) = self.end else { return; };

    // Determine the heuristic for each node
    // Todo: only visit nodes whose distance is still unset
    self.unvisited_nodes.iter()
      // Note: filter doesn't work due to borrow checker…
      // .filter(|at| self.grid[Self::pos_to_idx(at)].heuristic.is_none())
      .for_each(|at|
      {
        // self.node_at(at).heuristic = distance(*at*12.+OFFSET, end*12.+OFFSET);
        if self.grid[Self::pos_to_idx(at)].heuristic.is_none()
        {
          self.grid[Self::pos_to_idx(at)].heuristic = Some(distance(&offset_vec(at), &offset_vec(&end)));
        }
      });

    // Sort the unvisited nodes by their heuristic (best first)
    self.unvisited_nodes.sort_by(|a, b|
    {
      let a_h = self.grid[Self::pos_to_idx(a)].heuristic;
      let b_h = self.grid[Self::pos_to_idx(b)].heuristic;
      a_h.partial_cmp(&b_h).unwrap()
    });

    // Remove visited nodes
    self.unvisited_nodes.retain(|at| !self.grid[Self::pos_to_idx(at)].visited);
    // unvisited_nodes.dedup();

    if self.unvisited_nodes.is_empty()
    {
      self.finding_path = false;
      return;
    }

    let current_coords = *self.unvisited_nodes.first().unwrap();
    self.grid[Self::pos_to_idx(&current_coords)].visited = true;

    // let current_distance = self.node_at(&current_coords).distance;
    let current_distance = match self.grid[Self::pos_to_idx(&current_coords)].distance
    {
      Some(dist) => dist,
      None => f32::MAX
    };

    // Fix: distance should be 0. without this
    // if current_coords == start { current_distance = 0.; }

    // Visiting all unvisited neighbours of the current node
    for neighbour_coordinates in self.get_unvisited_neighbours(&current_coords)
    {
      // let local_distance = current_distance + distance(&(current_coords*12.+OFFSET), &(neighbour_coordinates*12.+OFFSET));

      // Fix: set local distance to lower value
      let local_distance = if current_distance == f32::MAX { current_distance }
      else
      { current_distance + distance(&offset_vec(&current_coords), &offset_vec(&neighbour_coordinates)) };

      // If a path has been found, don't explore any paths that are longer than the found one
      if local_distance > self.path_length { continue; }

      // println!("path_length:{} local_distance:{}", self.path_length, local_distance);

      // We have reached a path. In order to find the shortest, we have to explore further
      if neighbour_coordinates == end
      { self.path_length = local_distance; }

      let neighbour = &mut self.grid[Self::pos_to_idx(&neighbour_coordinates)];

      self.unvisited_nodes.push(neighbour_coordinates);

      // Core of the algorithm: update node internals, if shorter path is possible
      if let Some(dist) = neighbour.distance { if dist < local_distance { return; } }

      neighbour.parent = Some(current_coords);
      neighbour.distance = Some(local_distance);
    }
  }
}
