use std::collections::HashSet;
use cgmath::Vector2;
use winit::event::{ElementState, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::inbuilt::setup::Setup;
use crate::packages::camera_package::CameraPackage;

pub struct InputManager {
   currently_pressed: HashSet<KeyCode>,
   just_pressed: HashSet<KeyCode>,
   pub mouse_screen_pos: Vector2<f32>,
}
impl InputManager {
   pub fn new() -> Self {
      Self {
         currently_pressed: HashSet::new(),
         just_pressed: HashSet::new(),
         mouse_screen_pos: Vector2::new(0.0, 0.0),
      }
   }

   pub fn process_event(&mut self, event: &WindowEvent) {
      if let WindowEvent::KeyboardInput { event, .. } = event {
         match event.state {
            ElementState::Pressed => {
               if let PhysicalKey::Code(keycode) = event.physical_key {
                  self.currently_pressed.insert(keycode);
                  self.just_pressed.insert(keycode);
               }
            }
            ElementState::Released => {
               if let PhysicalKey::Code(keycode) = event.physical_key {
                  self.currently_pressed.remove(&keycode);
               }
            }
         }
      }

      if let WindowEvent::CursorMoved { position, ..} = event {
         self.mouse_screen_pos = Vector2::new(position.x as f32, position.y as f32)
      }

   }

   pub fn is_key_pressed(&self, key: KeyCode) -> bool {
      self.currently_pressed.contains(&key)
   }

   pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
      self.just_pressed.contains(&key)
   }

   pub fn pull_world_pos_2d(&mut self, camera_package: &CameraPackage, setup: &Setup) -> Vector2<f32> {
      let screen_size = setup.window.inner_size();

      camera_package.camera_controller.screen_to_world_pos(
         self.mouse_screen_pos,
         Vector2::new(screen_size.width as f32, screen_size.height as f32),
         &camera_package.camera,
      )
   }

   pub fn reset(&mut self) {
      self.just_pressed.clear();
   }
}