use egui::{Align2, Context, Ui};
use crate::packages::time_package::TimePackage;

pub fn gui(
   ui: &Context,
   time_package: &TimePackage,
) {

   let code = | ui: &mut Ui |
   {
      ui.add(egui::Label::new(format!("FPS: {}", time_package.fps)));

      let mut test = 23.0;
      ui.add(egui::Slider::new(&mut test, 0.1..=1.0).text("test"));

      ui.end_row();
   };

    egui::Window::new("template thinggy")
        .default_open(true)
        .max_width(1000.0)
        .max_height(800.0)
        .default_width(800.0)
        .resizable(true)
        .anchor(Align2::LEFT_TOP, [0.0, 0.0])
        .show(&ui, code);
}