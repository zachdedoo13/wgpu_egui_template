use egui::{Align2, Context, Ui};
use crate::bundles::automata::automata_bundle::AutomataBundle;
use crate::bundles::automata::automata_compute_pipeline::Automata;
use crate::inbuilt::setup::Setup;
use crate::packages::time_package::TimePackage;

pub fn gui(
   ui: &Context,
   time_package: &TimePackage,
   automata_bundle: &mut AutomataBundle,
   setup: &Setup,
) {

   let code = | ui: &mut Ui |
   {
      ui.add(egui::Label::new(format!("FPS: {}", time_package.fps as i32)));

      ui.add(egui::Slider::new(&mut automata_bundle.target_size.x, 1..=8192).text("size"));
      automata_bundle.target_size.y = automata_bundle.target_size.x;

      ui.add(egui::Slider::new(&mut automata_bundle.update_rate, 5.0..=144.0).text("update rate ~fps"));

      if ui.add(egui::Button::new("reset")).clicked() {
         automata_bundle.reset_package(setup);
      }

      ui.add(egui::Checkbox::new(&mut automata_bundle.generate_random, "generate random"));
      ui.add(egui::Checkbox::new(&mut automata_bundle.running, "running"));


      ui.add_space(20.0);
      ui.add(egui::Label::new(format!("Active: {}", match automata_bundle.active_automata {
         Automata::GameOfLife => {"Game Of Life"}
         Automata::SmoothLife => {"Smooth Life"}
      })));

      if ui.add(egui::Button::new("Game Of Life")).clicked() {
         automata_bundle.active_automata = Automata::GameOfLife;
         automata_bundle.reset_compute(&setup);
      }
      if ui.add(egui::Button::new("Smooth Life")).clicked() {
         automata_bundle.active_automata = Automata::SmoothLife;
         automata_bundle.reset_compute(&setup);
      }


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