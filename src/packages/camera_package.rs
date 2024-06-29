use cgmath::{SquareMatrix, Transform, Vector2};
use wgpu::{Device, Queue};
use wgpu::util::DeviceExt;
use winit::keyboard::KeyCode::{KeyA, KeyD, KeyS, KeyW, KeyX, KeyZ};
use crate::packages::input_manager_package::InputManager;

pub struct OrthographicCamera {
   pub eye: cgmath::Point3<f32>,
   pub target: cgmath::Point3<f32>,
   pub up: cgmath::Vector3<f32>,
   pub aspect: f32,
   pub zoom: f32,
}

impl OrthographicCamera {
   pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
      let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

      let height = 2.0 / self.zoom;
      let width = self.aspect * height;

      let left = -width / 2.0;
      let right = width / 2.0;
      let bottom = -height / 2.0;
      let top = height / 2.0;

      let proj = cgmath::ortho(left, right, bottom, top, -1.0, 1.0);
      proj * view
   }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
   view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
   pub fn new() -> Self {
      use cgmath::SquareMatrix;
      Self {
         view_proj: cgmath::Matrix4::identity().into(),
      }
   }

   pub fn update_view_proj(&mut self, camera: &OrthographicCamera) {
      self.view_proj = (camera.build_view_projection_matrix()).into();
   }
}

pub struct OrthographicCameraController {
   speed: f32,
}

impl OrthographicCameraController {
   pub fn new(speed: f32) -> Self {
      Self {
         speed,
      }
   }

   pub fn update_camera(&self, camera: &mut OrthographicCamera, delta_time: f32, input_manager: &InputManager) {
      if input_manager.is_key_pressed(KeyZ) { camera.zoom *=  self.speed + self.speed * delta_time }
      if input_manager.is_key_pressed(KeyX) { camera.zoom *= self.speed - self.speed * delta_time }

      if input_manager.is_key_pressed(KeyW) { camera.eye.y += self.speed * delta_time; camera.target.y += self.speed * delta_time }
      if input_manager.is_key_pressed(KeyS) { camera.eye.y -= self.speed * delta_time; camera.target.y -= self.speed * delta_time }

      if input_manager.is_key_pressed(KeyD) { camera.eye.x += self.speed * delta_time; camera.target.x += self.speed * delta_time }
      if input_manager.is_key_pressed(KeyA) { camera.eye.x -= self.speed * delta_time; camera.target.x -= self.speed * delta_time }
   }

   pub fn screen_to_world_pos(
      &self,
      screen_pos: Vector2<f32>,
      window_size: Vector2<f32>,
      camera: &OrthographicCamera,
   ) -> Vector2<f32> {
      let screen_pos = Vector2::new(
         2.0 * screen_pos.x / window_size.x - 1.0,
         1.0 - 2.0 * screen_pos.y / window_size.y,
      );

      let inv_view_proj = camera.build_view_projection_matrix().invert().unwrap();


      let world_pos = inv_view_proj.transform_point(cgmath::Point3::new(screen_pos.x, screen_pos.y, 0.0));


      Vector2::new(world_pos.x, world_pos.y)
   }
}


pub struct CameraPackage {
   pub(crate) camera: OrthographicCamera,
   camera_uniform: CameraUniform,
   camera_buffer: wgpu::Buffer,
   pub(crate) camera_bind_group_layout: wgpu::BindGroupLayout,
   pub(crate) camera_bind_group: wgpu::BindGroup,
   pub(crate) camera_controller: OrthographicCameraController,
}

impl CameraPackage {
   pub fn new(device: &Device, camera: OrthographicCamera) -> Self {
      let mut camera_uniform = CameraUniform::new();
      camera_uniform.update_view_proj(&camera);

      let camera_buffer = device.create_buffer_init(
         &wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
         }
      );

      let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
         entries: &[
            wgpu::BindGroupLayoutEntry {
               binding: 0,
               visibility: wgpu::ShaderStages::VERTEX,
               ty: wgpu::BindingType::Buffer {
                  ty: wgpu::BufferBindingType::Uniform,
                  has_dynamic_offset: false,
                  min_binding_size: None,
               },
               count: None,
            }
         ],
         label: Some("camera_bind_group_layout"),
      });

      let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
         layout: &camera_bind_group_layout,
         entries: &[
            wgpu::BindGroupEntry {
               binding: 0,
               resource: camera_buffer.as_entire_binding(),
            }
         ],
         label: Some("camera_bind_group"),
      });

      let camera_controller = OrthographicCameraController::new(1.0);

      Self {
         camera,
         camera_uniform,
         camera_buffer,
         camera_bind_group_layout,
         camera_bind_group,
         camera_controller,
      }
   }

   pub fn update(&mut self, queue: &mut Queue, delta_time: f32, input_manager: &InputManager) {
      self.camera_controller.update_camera(&mut self.camera, delta_time, input_manager);
      self.camera_uniform.update_view_proj(&self.camera);
      queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
   }
}