use std::iter;
use cgmath::{Vector3};
use egui::Context;
use egui_wgpu::ScreenDescriptor;
use wgpu::{CommandEncoder, TextureView};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;
use crate::egui::gui::EguiRenderer;
use crate::egui::gui_example::gui;
use crate::inbuilt::setup::Setup;
use crate::packages::automata_package::AutomataPackage;
use crate::packages::camera_package::{CameraPackage, OrthographicCamera};
use crate::packages::input_manager_package::InputManager;
use crate::packages::time_package::TimePackage;
use crate::pipelines::automata_compute_pipeline::AutomataComputePipeline;
use crate::pipelines::automata_pipeline::AutomataRenderPipeline;
use crate::pipelines::test_render_pipeline::TestRenderPipeline;


pub struct State<'a> {
   pub setup: Setup<'a>,
   pub egui: EguiRenderer,

   // packages
   time_package: TimePackage,
   camera_package: CameraPackage,
   input_manager: InputManager,

   automata_package: AutomataPackage,
   automata_render_pipeline: AutomataRenderPipeline,
   automata_compute_pipeline: AutomataComputePipeline,

   // pipelines
   #[allow(dead_code)]
   test_render_pipeline: TestRenderPipeline,
}

impl<'a> State<'a> {
   pub async fn new(window: &'a Window) -> State<'a> {

      // dependents
      let setup = Setup::new(window).await;
      let egui = EguiRenderer::new(&setup.device, setup.config.format, None, 1, setup.window);


      // packages
      let time_package = TimePackage::new();
      let input_manager = InputManager::new();
      let camera_package = CameraPackage::new(&setup.device, OrthographicCamera {
         eye: (0.0, 0.0, 1.0).into(),
         target: (0.0, 0.0, 0.0).into(),
         up: Vector3::unit_y(),
         aspect: setup.config.width as f32 / setup.config.height as f32,
         zoom: 1.0,
      });


      let automata_package = AutomataPackage::new(&setup, 86, 86);
      let automata_render_pipeline = AutomataRenderPipeline::new(&setup, &camera_package, &automata_package);
      let automata_compute_pipeline = AutomataComputePipeline::new(&setup, &automata_package);


      // pipelines
      let test_render_pipeline = TestRenderPipeline::new(&setup, &camera_package);


      Self {
         setup,
         egui,

         time_package,
         camera_package,
         input_manager,

         test_render_pipeline,


         automata_package,
         automata_render_pipeline,
         automata_compute_pipeline,
      }
   }

   pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
      if new_size.width > 0 && new_size.height > 0 {
         self.setup.size = new_size;
         self.setup.config.width = new_size.width;
         self.setup.config.height = new_size.height;
         self.setup.surface.configure(&self.setup.device, &self.setup.config);

         self.camera_package.camera.aspect = self.setup.config.width as f32 / self.setup.config.height as f32
      }
   }

   pub fn update_input(&mut self, event: &WindowEvent) -> bool {
      self.input_manager.process_event(event);
      false
   }

   pub fn update(&mut self) {
      self.time_package.update();
      self.camera_package.update(&mut self.setup.queue, self.time_package.delta_time as f32, &self.input_manager);

      // let mouse_world_pos = self.input_manager.pull_world_pos_2d(&self.camera_package, &self.setup);
      self.automata_package.bind_groups.ping_pong();

      self.input_manager.reset();
   }

   pub fn update_gui(&mut self, view: &TextureView, encoder: &mut CommandEncoder) {
      let screen_descriptor = ScreenDescriptor {
         size_in_pixels: [self.setup.config.width, self.setup.config.height],
         pixels_per_point: self.setup.window.scale_factor() as f32,
      };

      let run_ui = |ui: &Context| {
         gui(
            ui,
            &self.time_package,
         );
      };

      self.egui.draw(
         &self.setup.device,
         &self.setup.queue,
         encoder,
         &self.setup.window,
         &view,
         screen_descriptor,
         run_ui,
      );
   }

   pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
      let output = self.setup.surface.get_current_texture()?;
      let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
      let mut encoder = self.setup.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
         label: Some("Render Encoder"),
      });


      // compute passes
      {
         self.automata_compute_pipeline.compute_pass(&mut encoder, &self.automata_package);
      }

      // render passes
      {
         self.automata_render_pipeline.render_pass(&mut encoder, &view, &self.camera_package, &self.automata_package);
      }

      self.update_gui(&view, &mut encoder);


      self.setup.queue.submit(iter::once(encoder.finish()));
      output.present();

      Ok(())
   }
}