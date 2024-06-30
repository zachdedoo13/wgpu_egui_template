use wgpu::{CommandEncoder, ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, ShaderModuleDescriptor};
use crate::bundles::automata::automata_package::AutomataPackage;
use crate::inbuilt::setup::Setup;



pub enum Automata {
   GameOfLife,
   Test,
}


pub struct AutomataComputePipeline {
   pub pipeline: ComputePipeline,
}
impl AutomataComputePipeline {
   pub fn new(setup: &Setup, automata_package: &AutomataPackage, selected: Automata) -> Self {
      let shader = match selected {
         Automata::GameOfLife => include_str!("../../shaders/compute/game_of_life.wgsl"),
         Automata::Test => include_str!("../../shaders/compute/game_of_life.wgsl"),
      };

      let cs_module = setup.device.create_shader_module(ShaderModuleDescriptor {
         label: Some("compute_shader"),
         source: wgpu::ShaderSource::Wgsl(shader.into()),
      });

      let layout = setup.device.create_pipeline_layout(&PipelineLayoutDescriptor {
         label: Some("compute pipeline"),
         bind_group_layouts: &[
            &automata_package.bind_group_layout,
            &automata_package.bind_group_layout,
         ],
         push_constant_ranges: &[],
      });

      let pipeline = setup.device.create_compute_pipeline(&ComputePipelineDescriptor {
         label: Some("Compute Pipeline"),
         layout: Some(&layout),
         module: &cs_module,
         entry_point: "cs_main",
      });

      Self {
         pipeline,
      }
   }

   pub fn compute_pass(&mut self, encoder: &mut CommandEncoder, automata_package: &AutomataPackage) {
      let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
         label: Some("Compute Pass"),
         timestamp_writes: None,
      });

      compute_pass.set_pipeline(&self.pipeline);

      // read
      compute_pass.set_bind_group(0, automata_package.bind_groups.pull_current(), &[]);
      compute_pass.set_bind_group(1, automata_package.bind_groups.pull_other(), &[]);


      let texture_extent = automata_package.size;
      compute_pass.dispatch_workgroups(
         (texture_extent.width as f32 / 16.).ceil() as u32,
         (texture_extent.height as f32 / 16.).ceil() as u32,
         1
      );
   }
}