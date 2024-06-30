use cgmath::Vector2;
use wgpu::{
   BindGroup,
   BindGroupLayout,
   BindGroupLayoutDescriptor,
   BindGroupLayoutEntry,
   BindingType,
   Buffer,
   BufferBindingType,
   BufferSize,
   CommandEncoder,
   ComputePipeline,
   ComputePipelineDescriptor,
   Device,
   PipelineLayoutDescriptor,
   ShaderModuleDescriptor,
   ShaderStages,
   util::DeviceExt
};
use crate::bundles::automata::automata_package::AutomataPackage;
use crate::inbuilt::setup::Setup;

pub struct QueueComputePipeline {
   pub pipeline: ComputePipeline,
   pub buffer: Buffer,
   pub input_data: Vec<[i32; 4]>,
   pub input_array_layout: BindGroupLayout,
   pub bind_group: BindGroup,
}
impl QueueComputePipeline {
   pub fn new(device: &Device, automata_package: &AutomataPackage) -> Self {
      let cs_module = device.create_shader_module(ShaderModuleDescriptor {
         label: Some("compute_shader"),
         source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/compute/input_queue.wgsl").into()),
      });

      let middle = Vector2::new(automata_package.size.width / 2, automata_package.size.height / 2);
      let mut input_data: Vec<[i32; 4]> = vec![[0, 0, 0, 0]; 100];
      input_data[0] = [middle.x as i32, middle.y as i32, 1, 20];
      input_data[1] = [0, 0, 1, 20];

      let input_array_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
         label: None,
         entries: &[
            BindGroupLayoutEntry {
               binding: 0,
               visibility: ShaderStages::COMPUTE,
               ty: BindingType::Buffer {
                  ty: BufferBindingType::Uniform,
                  has_dynamic_offset: false,
                  min_binding_size: BufferSize::new(16 * input_data.len() as u64),
               },
               count: None,
            },
         ]
      });

      let buffer = device.create_buffer_init(
         &wgpu::util::BufferInitDescriptor {
            label: Some("Test uniform"),
            contents: bytemuck::cast_slice(&input_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
         }
      );

      let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
         label: None,
         layout: &input_array_layout,
         entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
         }],
      });


      let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
         label: Some("compute pipeline"),
         bind_group_layouts: &[
            &automata_package.bind_group_layout,
            &automata_package.bind_group_layout,
            &input_array_layout,
         ],
         push_constant_ranges: &[],
      });

      let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
         label: Some("Compute Pipeline"),
         layout: Some(&layout),
         module: &cs_module,
         entry_point: "cs_main",
      });

      Self {
         pipeline,

         bind_group,
         input_array_layout,
         input_data,
         buffer,
      }
   }

   pub fn update_queue(&mut self, setup: &Setup, data: Vec<[i32; 4]>) {
      let mut empty_data: [[i32; 4]; 100] = [[0; 4]; 100];

      for (i, entry) in data.iter().enumerate() {
         empty_data[i] = *entry;
      }

      let data = empty_data;

      setup.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&data));
   }

   pub fn compute_pass(&mut self, encoder: &mut CommandEncoder, automata_package: &AutomataPackage) {
      let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
         label: Some("Compute Pass"),
         timestamp_writes: None,
      });

      compute_pass.set_pipeline(&self.pipeline);

      // read
      compute_pass.set_bind_group(0, &automata_package.bind_groups.pull_other(), &[]);
      compute_pass.set_bind_group(1, &automata_package.bind_groups.pull_current(), &[]);

      compute_pass.set_bind_group(2, &self.bind_group, &[]);


      compute_pass.dispatch_workgroups(
         (self.input_data.len() as f32 / 8.).ceil() as u32,
         1,
         1
      );
   }
}