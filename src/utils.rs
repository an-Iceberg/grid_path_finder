pub(crate) fn is_point_in_circle(
  point_x: f32, point_y: f32,
  circle_x: f32, circle_y: f32,
  circle_radius: f32
) -> bool
{
  return (circle_x - point_x).powf(2_f32) + (circle_y - point_y).powf(2_f32) <= circle_radius.powf(2_f32);
}

/// `square_x` and `square_y` are the centre of the square.
pub(crate) fn is_point_in_square(
  point_x: f32, point_y: f32,
  square_x: f32, square_y: f32,
  size: f32
) -> bool
{
  if point_x < square_x - (size / 2.) - 2. ||
    point_y < square_y - (size / 2.) - 2. ||
    point_x > square_x + (size / 2.) + 2. ||
    point_y > square_y + (size / 2.) + 2.
  { return false;}

  return true;
}
