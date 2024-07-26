
use macroquad::math::Vec2;

use crate::grid::Grid;

fn get_grid() -> Grid
{
  Grid::new()
}

#[test]
fn unvisited_neighbours()
{
  // Todo: more test cases
  let mut grid = get_grid();

  let should = vec![
    Vec2::new(4., 0.),
    Vec2::new(4., 1.),
    Vec2::new(5., 1.),
    Vec2::new(6., 1.),
    Vec2::new(6., 0.),
  ];
  let is = grid.get_unvisited_neighbours(&Vec2::new(5., 0.));

  assert_eq!(should, is);

  // println!("{:?}", is);

  // panic!()
}
