use eframe::{App, CreationContext, Frame};
use egui::{Margin, Rounding, Stroke};
#[allow(unused_imports)] // TODO: remove when time comes
use egui::{pos2, vec2, Align2, CentralPanel, Color32, Context, RichText, Shadow, Shape, SidePanel, Ui, Visuals, Window};
use crate::{node::Node, AUTHORS, REPOSITORY, VERSION};

#[derive(Default)]
pub struct GridPathFinder
{
  grid: Vec<Node>,
}

impl GridPathFinder
{
  /// Called once before the first frame.
  #[allow(clippy::needless_return)]
  pub fn new(cc: &CreationContext<'_>) -> Self
  {
    // This is also where you can customize the look and feel of egui using
    // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

    cc.egui_ctx.set_visuals(Visuals
    {
      window_shadow: Shadow::NONE,
      ..Default::default()
    });

    return Default::default();
  }
}

impl App for GridPathFinder
{
  /// Called each time the UI needs repainting, which may be many times per second.
  fn update(&mut self, ctx: &Context, frame: &mut Frame)
  {
    let δ_time = frame.info().cpu_usage.unwrap_or(0.001);
    let screen_rect = ctx.input(|i: &egui::InputState| i.screen_rect());
    #[allow(unused_variables)] // TODO: remove when time comes
    let (screen_width, screen_height) = (screen_rect.width(), screen_rect.height());

    // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
    // For inspiration and more examples, go to https://emilk.github.io/egui

    SidePanel::right("my_right_panel")
      .resizable(false)
      .exact_width(170.)
    .show(ctx, |ui|
    {
      ui.heading("Grid Path Finder");
      ui.separator();
      ui.add_space(20.);

      ui.heading("Controls");
      ui.add_space(20.);

      ui.heading("Stats");
      ui.monospace(RichText::new(format!("fps:{:>7.2}", 1./δ_time)));
      ui.add_space(20.);

      credits(ui);
    });

    CentralPanel::default().show(ctx, |ui|
    {
      // Canvas to draw on
      egui::Frame::canvas(ui.style())
        .fill(Color32::from_rgb(110, 80, 30))
        // .rounding(Rounding{nw: 20., ne: 20., sw: 20., se: 20.})
      .show(ui, |ui|
      {
        ui.allocate_space(ui.available_size());

        let mut shapes = vec![];

        shapes.push(Shape::circle_filled(pos2(100., 200.), 20., Color32::from_rgb(128, 0, 255)));
        shapes.push(Shape::circle_stroke(pos2(200., 100.), 5., Stroke::new(1., Color32::WHITE)));

        ui.painter().extend(shapes);

        ui.ctx().request_repaint();
      });

      // Window::new("Balls")
      //   .movable(false)
      //   .resizable(false)
      //   .anchor(Align2::RIGHT_TOP, vec2(-10., 10.))
      //   .max_size(vec2(160., 0.))
      //   .show(ctx, |ui|
      //   {
      //     ui.heading("Controls");

      //     ui.separator();

      //     ui.heading("Stats:");
      //     ui.monospace(RichText::new(format!("fps:{:>7.2}", 1./δ_time)));
      //     // ui.monospace(RichText::new(format!("δ time: {}", δ_time)));

      //     credits(ui);
      //   });
    });
  }
}

fn credits(ui: &mut Ui)
{
  ui.heading("Credits");

  // Me
  ui.horizontal(|ui|
  {
    ui.monospace(RichText::new(format!("v{}", VERSION.unwrap_or("unknown"))));
    // ui.label(format!("v{}", VERSION.unwrap_or("unknown")));
    ui.separator();
    ui.spacing_mut().item_spacing.x = 0.0;
    ui.label("Made by ");
    ui.hyperlink_to("Sandra", "https://github.com/an-Iceberg")
      .on_hover_text(AUTHORS.unwrap());
  });

  ui.hyperlink_to("GitHub repository link ⇗", REPOSITORY.unwrap())
    .on_hover_text(REPOSITORY.unwrap());

  // egui & eframe
  ui.horizontal(|ui|
  {
    let egui_link = "https://github.com/emilk/egui#readme";
    let eframe_link = "https://github.com/emilk/eframe_template#readme";

    ui.spacing_mut().item_spacing.x = 0.0;
    ui.label("Powered by ");
    ui.hyperlink_to("egui", egui_link).on_hover_text(egui_link);
    ui.label(" & ");
    ui.hyperlink_to("eframe", eframe_link,).on_hover_text(eframe_link);
  });
}
