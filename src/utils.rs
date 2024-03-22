use macroquad::math::Vec2;

// pub(crate) fn is_point_in_circle(
//   point_x: f32, point_y: f32,
//   circle_x: f32, circle_y: f32,
//   circle_radius: f32
// ) -> bool
// {
//   return (circle_x - point_x).powf(2_f32) + (circle_y - point_y).powf(2_f32) <= circle_radius.powf(2_f32);
// }

pub(crate) fn is_point_in_circle(point: Vec2, circle: Vec2, circle_radius: f32) -> bool
{
  return (circle.x - point.x).powf(2.) + (circle.y - point.y).powf(2.) <= circle_radius.powf(2.);
}

/// `square_x` and `square_y` are the centre of the square.
// pub(crate) fn is_point_in_square(
//   point_x: f32, point_y: f32,
//   square_x: f32, square_y: f32,
//   size: f32
// ) -> bool
// {
//   if point_x < square_x - (size / 2.) - 3. ||
//     point_y < square_y - (size / 2.) - 3. ||
//     point_x > square_x + (size / 2.) + 3. ||
//     point_y > square_y + (size / 2.) + 3.
//   { return false;}

//   return true;
// }

/// `square_x` and `square_y` are the centre of the square.
pub(crate) fn is_point_in_square(point: Vec2, square_center: Vec2, size: f32) -> bool
{
  if point.x < square_center.x - (size / 2.) - 3. ||
    point.y < square_center.y - (size / 2.) - 3. ||
    point.x > square_center.x + (size / 2.) + 3. ||
    point.y > square_center.y + (size / 2.) + 3.
  { return false;}

  return true;
}

// pub(crate) fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32
// {
//   return ((x2-x1).powi(2) + (y2-y1).powi(2)).sqrt();
// }

pub(crate) fn distance(v1: Vec2, v2: Vec2) -> f32
{
  return ((v2.x - v1.x).powi(2) + (v2.y-v1.y).powi(2)).sqrt();
}
