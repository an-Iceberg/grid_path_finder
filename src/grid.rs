use macroquad::math::Vec2;
use rand::{rngs::ThreadRng, Rng};
use crate::{node::Node, utils::distance, DIRECTIONS, GRID_HEIGHT, GRID_WIDTH, OFFSET};

pub(crate) struct Grid
{
  grid: [Node; GRID_WIDTH * GRID_HEIGHT],
  start: Option<Vec2>,
  end: Option<Vec2>,
  path_length: f32,
  unvisited_nodes: Vec<Vec2>,
  rng: ThreadRng,
  finding_path: bool,
}

// TODO: convert all Vec2 to &Vec2
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
      unvisited_nodes: vec![],
      rng: rand::thread_rng(),
      finding_path: false,
    };
  }

  pub fn node_at(&mut self, pos: &Vec2) -> &mut Node
  {
    return &mut self.grid[(pos.y * GRID_WIDTH as f32 + pos.x) as usize];
  }

  pub fn set_start(&mut self, pos: &Vec2)
  {
    self.node_at(pos).set_to_node();
    self.start = Some(*pos);
  }

  pub fn set_end(&mut self, pos: &Vec2)
  {
    self.node_at(pos).set_to_node();
    self.end = Some(*pos);
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
    self.unvisited_nodes.clear();
  }

  pub fn find_path(&mut self)
  {
    self.finding_path = true;
  }

  pub fn get_current_node(&self) -> Option<&Vec2>
  {
    return self.unvisited_nodes.last();
  }

  pub fn finding_path(&self) -> bool
  {
    return self.finding_path;
  }

  pub fn get_parent(&self, at: &Vec2) -> Option<Vec2>
  {
    return self.grid[Self::pos_to_idx(at)].parent;
  }

  pub fn get_current_path(&self) -> Option<Vec<Vec2>>
  {
    let mut current_path = vec![];
    match self.unvisited_nodes.last()
    {
      Some(node) => current_path.push(node),
      None => ()
    }
    if current_path.is_empty() { return None; }

    // todo: get current path

    todo!()
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
      .filter(|node| !node.is_obstacle) // Note: is this filter really necessary?
      .for_each(|node| node.clear());

    self.unvisited_nodes.clear();

    if let Some(start) = self.start
    {
      self.grid[Self::pos_to_idx(&start)].distance = None;
      self.unvisited_nodes.push(start);
    }
    // if let Some(start) = self.start { self.node_at(&start).distance = 0.; }
  }

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

  pub fn get_unvisited_neighbours(&mut self, pos: &Vec2) -> Vec<Vec2> // Vec<&mut Node>
  {
    let mut neighbours = vec![];

    // DIRECTIONS.iter().for_each(|direction|
    // {
    //   if self.has_neighbour(pos, direction)
    //   {
    //     // let neighbour = self.node_at(&(*pos + *direction));
    //     let neighbour = self.grid[Self::pos_to_idx(&(*pos + *direction))];
    //     if !neighbour.is_obstacle && !neighbour.visited { neighbours.push(*pos + *direction); }
    //   }
    // });

    DIRECTIONS.iter()
      .filter(|dir| self.has_neighbour(pos, dir))
      .for_each(|direction|
      {
        let neighbour = self.grid[Self::pos_to_idx(&(*pos + *direction))];
        if !neighbour.is_obstacle && !neighbour.visited { neighbours.push(*pos + *direction); }
      });

    return neighbours;
  }

  pub fn is_unvisited_nodes_empty(&mut self) -> bool
  {
    return self.unvisited_nodes.is_empty();
  }

  fn pos_to_idx(pos: &Vec2) -> usize
  {
    return (pos.y * GRID_WIDTH as f32 + pos.x) as usize;
  }

  // FIX: don't use cells outside the grid
  /// A* algorithm
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
          self.grid[Self::pos_to_idx(at)].heuristic = Some(distance(&(*at*12.+OFFSET), &(end*12.+OFFSET)));
        }
      });

    // Sort the unvisited nodes by their heuristic (best first)
    self.unvisited_nodes.sort_by(|a, b|
    {
      let a_h = self.grid[Self::pos_to_idx(a)].heuristic;
      let b_h = self.grid[Self::pos_to_idx(b)].heuristic;
      return a_h.partial_cmp(&b_h).unwrap();
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
    // self.node_at(&current_node_coordinates).visited = true;
    self.grid[Self::pos_to_idx(&current_coords)].visited = true;
    // let current_distance = self.node_at(&current_coords).distance;
    let current_distance = match self.grid[Self::pos_to_idx(&current_coords)].distance
    {
      Some(dist) => dist,
      None => f32::MAX
    };

    // Visiting all unvisited neighbours of the current node
    for neighbour_coordinates in self.get_unvisited_neighbours(&current_coords)
    {
      let local_distance = current_distance + distance(&(current_coords*12.+OFFSET), &(neighbour_coordinates*12.+OFFSET));

      // If a path has been found, don't explore any paths that are longer than the found one
      if local_distance > self.path_length { continue; }

      if neighbour_coordinates == end
      {
        // self.path_length = self.node_at(self.end.unwrap()).distance;
        // self.node_at(neighbour_coordinates).visited = false;
        // self.node_at(&self.get_end().unwrap()).parent = current_coords;
        self.grid[Self::pos_to_idx(&end)].parent = Some(current_coords);
        self.unvisited_nodes.clear();
        self.unvisited_nodes.push(end);
        self.finding_path = false;
        return;
      }

      let mut neighbour = self.grid[Self::pos_to_idx(&neighbour_coordinates)];

      self.unvisited_nodes.push(neighbour_coordinates);

      // Core of the algorithm: update node internals, if shorter path is possible
      match neighbour.distance
      {
        Some(dist) =>
        {
          if dist > local_distance
          {
            neighbour.distance = Some(local_distance);
            neighbour.parent = Some(current_coords);
          }
        }
        None => neighbour.distance = Some(local_distance)
      }
      // if neighbour.distance > local_distance
      // {
      //   neighbour.distance = local_distance;
      //   neighbour.parent = Some(current_coords);
      // }
    }
  }
}
