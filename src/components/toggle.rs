use egui::{lerp, pos2, vec2, Response, Sense, StrokeKind, Ui, Widget};

pub fn toggle(on: &mut bool) -> impl Widget + '_
{
  move |ui: &mut Ui| toggle_button(ui, on)
}

fn toggle_button(ui: &mut Ui, on: &mut bool) -> Response
{
  let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
  let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

  if response.clicked()
  {
    *on = !*on;
    response.mark_changed();
  }

  if !ui.is_rect_visible(rect) { return response; }

  // Purely visuals
  let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
  let visuals = ui.style().interact_selectable(&response, *on);
  let rect = rect.expand(visuals.expansion);
  let radius = 0.5 * rect.height();

  ui.painter()
    .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke, StrokeKind::Inside);

  let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
  let center = pos2(circle_x, rect.center().y);

  ui.painter()
    .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);

  return response;
}

// Todo: create your own node/cell component
// Todo: create grid component
