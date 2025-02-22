use std::time::{Instant};
use cgmath::Vector2;
use wgpu::{CommandEncoder, TextureView};
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::keyboard::KeyCode::{KeyB};
use crate::bundles::automata::automata_compute_pipeline::{Automata, AutomataComputePipeline};
use crate::bundles::automata::automata_package::AutomataPackage;
use crate::bundles::automata::automata_pipeline::AutomataRenderPipeline;
use crate::bundles::automata::automata_queue_compute_pipeline::QueueComputePipeline;
use crate::inbuilt::setup::Setup;
use crate::packages::camera_package::CameraPackage;
use crate::packages::input_manager_package::InputManager;
use crate::packages::time_package::{TimePackage, Timer};


pub struct AutomataBundle {
   package: AutomataPackage,
   render_pipeline: AutomataRenderPipeline,
   compute_pipeline: AutomataComputePipeline,
   queue_compute_pipeline: QueueComputePipeline,

   pub target_size: Vector2<u32>,
   pub running: bool,
   pub generate_random: bool,
   pub update_rate: f64,
   pub active_automata: Automata,

   pub limit_compute_fps: bool,
   time_since_last_compute_pass: Instant,

   update_queued: bool,

   pub brush_type: i32,
   pub brush_size: i32,
}
impl AutomataBundle {
   pub fn new(
      setup: &Setup,
      camera_package: &CameraPackage,
   ) -> Self {
      let target_size = Vector2::new(56, 56);
      let generate_random = true;
      let update_rate = 60.0;
      let active_automata = Automata::GameOfLife;

      let brush_type = 1;
      let brush_size = 5;

      let automata_package = AutomataPackage::new(&setup, target_size.x, target_size.y, generate_random);
      let automata_render_pipeline = AutomataRenderPipeline::new(&setup, camera_package, &automata_package);
      let automata_compute_pipeline = AutomataComputePipeline::new(&setup, &automata_package, &active_automata);
      let queue_pipeline = QueueComputePipeline::new(&setup.device, &automata_package);

      Self {
         package: automata_package,
         render_pipeline: automata_render_pipeline,
         compute_pipeline: automata_compute_pipeline,
         queue_compute_pipeline: queue_pipeline,

         target_size,
         generate_random,
         update_rate,
         active_automata,
         brush_type,
         brush_size,

         limit_compute_fps: true,
         time_since_last_compute_pass: Instant::now(),

         update_queued: false,
         running: true
      }
   }

   pub fn update(&mut self, input_manager: &InputManager, setup: &Setup, camera_package: &CameraPackage) {
      if input_manager.is_mouse_key_just_pressed(MouseButton::Left) {
         let pix_pos = self.get_pix_pos(input_manager, camera_package, setup);

         self.queue_compute_pipeline.update_queue(setup, vec![[pix_pos.x, pix_pos.y, self.brush_type, self.brush_size]]);

         self.update_queued = true;
      }
      else if input_manager.is_mouse_key_just_pressed(MouseButton::Right) {
         let pix_pos = self.get_pix_pos(input_manager, camera_package, setup);

         self.queue_compute_pipeline.update_queue(setup, vec![[pix_pos.x, pix_pos.y, -self.brush_type, self.brush_size]]);

         self.update_queued = true;
      }
      else { self.update_queued = false; }

      if input_manager.is_key_pressed(KeyB) {
         self.running = !self.running;
      }

      if input_manager.is_key_just_pressed(KeyCode::Space) {
         self.reset_package(setup);
      }
   }

   fn get_pix_pos(&self, input_manager: &InputManager, camera_package: &CameraPackage, setup: &Setup) -> Vector2<i32> {
      let world_pos = input_manager.pull_world_pos_2d(camera_package, setup);
      let cube_pos_normal = Vector2::new(
         (world_pos.x + 1.0) / 2.0,
         (world_pos.y + 1.0) / 2.0,
      );

      if cube_pos_normal.x > 1.0 || cube_pos_normal.x < 0.0 {return Vector2::new(i32::MAX, i32::MAX)}
      else if cube_pos_normal.y > 1.0 || cube_pos_normal.y < 0.0 {return Vector2::new(i32::MAX, i32::MAX)}

      let pix_pos = Vector2::new(
         (self.package.size.width as f32 * cube_pos_normal.x).ceil() as i32 -1,
         (self.package.size.height as f32 * cube_pos_normal.y).ceil() as i32 -1,
      );

      pix_pos
   }

   pub fn reset_package(&mut self, setup: &Setup) {
      self.package = AutomataPackage::new(&setup, self.target_size.x, self.target_size.y, self.generate_random);
      self.package.bind_groups.ping_pong(); // needed or it breaks
   }

   pub fn reset_compute(&mut self, setup: &Setup) {
      self.compute_pipeline = AutomataComputePipeline::new(&setup, &self.package, &self.active_automata);
   }

   pub fn automata_pass(
      &mut self, encoder: &mut CommandEncoder,
      view: &TextureView,
      camera_package: &CameraPackage,
      time_package: &mut TimePackage,
   ) {
      if self.update_queued { self.queue_compute_pipeline.compute_pass(encoder, &self.package); }

      self.render_pipeline.render_pass(encoder, view, camera_package, &self.package);

      if self.limit_compute_fps {
         if self.update_rate > 0.0 {
            let target = 1.0 / self.update_rate;
            let diff = target - time_package.delta_time;

            if self.time_since_last_compute_pass.elapsed().as_secs_f64() > diff {
               if self.running {
                  self.compute_pipeline.compute_pass(encoder, &self.package);
                  self.time_since_last_compute_pass = Instant::now();
                  self.package.bind_groups.ping_pong();
               }
            }
         }
      }
      else {
         self.compute_pipeline.compute_pass(encoder, &self.package);
         self.time_since_last_compute_pass = Instant::now();
         self.package.bind_groups.ping_pong();
      }


   }
}