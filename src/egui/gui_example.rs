use egui::{Align2, Context, };

pub fn gui(
   ui: &Context,
) {
    egui::Window::new("template thinggy")
        .default_open(true)
        .max_width(1000.0)
        .max_height(800.0)
        .default_width(800.0)
        .resizable(true)
        .anchor(Align2::LEFT_TOP, [0.0, 0.0])
        .show(&ui, |ui| {

           let mut test = 23.0;
           ui.add(egui::Slider::new(&mut test, 0.1..=1.0).text("test"));


           ui.end_row();
        });
}


fn round_to_x_decimals(num: f32, decimals: u32) -> f32 {
   let multiplier = 10f32.powi(decimals as i32);
   (num * multiplier).round() / multiplier
}