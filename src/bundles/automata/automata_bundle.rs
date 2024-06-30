use cgmath::Vector2;
use wgpu::{CommandEncoder, TextureView};
use winit::keyboard::KeyCode;
use winit::keyboard::KeyCode::KeyV;
use crate::bundles::automata::automata_compute_pipeline::AutomataComputePipeline;
use crate::bundles::automata::automata_package::AutomataPackage;
use crate::bundles::automata::automata_pipeline::AutomataRenderPipeline;
use crate::bundles::automata::automata_queue_compute_pipeline::QueueComputePipeline;
use crate::inbuilt::setup::Setup;
use crate::packages::camera_package::CameraPackage;
use crate::packages::input_manager_package::InputManager;

pub struct AutomataBundle {
   package: AutomataPackage,
   render_pipeline: AutomataRenderPipeline,
   compute_pipeline: AutomataComputePipeline,
   queue_compute_pipeline: QueueComputePipeline,

   update_queued: bool,
}
impl AutomataBundle {
   pub fn new(
      setup: &Setup,
      camera_package: &CameraPackage,
      width: u32,
      height: u32,
      generate_random: bool
   ) -> Self {
      let automata_package = AutomataPackage::new(&setup, width, height, generate_random);
      let automata_render_pipeline = AutomataRenderPipeline::new(&setup, camera_package, &automata_package);
      let automata_compute_pipeline = AutomataComputePipeline::new(&setup, &automata_package);
      let queue_pipeline = QueueComputePipeline::new(&setup.device, &automata_package);

      Self {
         package: automata_package,
         render_pipeline: automata_render_pipeline,
         compute_pipeline: automata_compute_pipeline,
         queue_compute_pipeline: queue_pipeline,

         update_queued: false,
      }
   }

   pub fn update(&mut self, input_manager: &InputManager, setup: &Setup, camera_package: &CameraPackage) {
      self.package.bind_groups.ping_pong();


      if input_manager.is_key_pressed(KeyV) {
         let world_pos = input_manager.pull_world_pos_2d(camera_package, setup);
         let cube_pos_normal = Vector2::new(
            (world_pos.x + 1.0) / 2.0,
            (world_pos.y + 1.0) / 2.0,
         );
         let pix_pos = Vector2::new(
            (self.package.size.width as f32 * cube_pos_normal.x).ceil() as i32,
            (self.package.size.height as f32 * cube_pos_normal.y).ceil() as i32,
         );

         self.queue_compute_pipeline.update_queue(setup, vec![[pix_pos.x, pix_pos.y, 1, 20]]);

         self.update_queued = true;
      } else { self.update_queued = false; }

      if input_manager.is_key_just_pressed(KeyCode::Space) {
         self.reset_package(setup);
      }
   }

   fn reset_package(&mut self, setup: &Setup) {
      self.package = AutomataPackage::new(&setup, self.package.size.width, self.package.size.height, false);
      self.package.bind_groups.ping_pong(); // needed or it breaks
   }

   pub fn automata_pass(&mut self, encoder: &mut CommandEncoder, view: &TextureView, camera_package: &CameraPackage) {
      if self.update_queued { self.queue_compute_pipeline.compute_pass(encoder, &self.package); }
      self.compute_pipeline.compute_pass(encoder, &self.package);
      self.render_pipeline.render_pass(encoder, view, camera_package, &self.package)
   }
}