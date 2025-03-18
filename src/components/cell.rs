use egui::{vec2, Color32, Response, Sense, Ui, Widget};

pub fn cell(on: &mut bool) -> impl Widget + '_
{
  move |ui: &mut Ui| cell_button(ui, on)
}

fn cell_button(ui: &mut Ui, on: &mut bool) -> Response
{
  let desired_size = ui.spacing().interact_size.y * vec2(1.0, 1.0);
  let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

  if response.clicked()
  {
    *on = !*on;
    response.mark_changed();
  }

  if !ui.is_rect_visible(rect) { return response; }

  // Visuals
  let visuals = ui.style().interact_selectable(&response, *on);
  let radius = 0.5 * rect.height();

  ui.painter()
    .circle(
      rect.center(),
      0.75 * radius,
      if *on { Color32::WHITE } else { visuals.bg_fill },
      visuals.fg_stroke
    );

  return response;
}
