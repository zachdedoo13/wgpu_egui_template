use std::time::{Instant};


const UPDATE_INTERVAL: f64 = 0.5;
const PAST_FPS_AMOUNT: usize = 10;

#[allow(unused_variables)]
pub struct TimePackage {
   pub start_time: Instant,
   pub fps: f64,
   pub delta_time: f64,
   pub past_fps: Vec<f64>,
   last_frame: Instant,
   last_data_dump: Instant,
   past_frames: Vec<f64>,

   pub update_interval: f64,
   pub past_fps_amount: usize,
}
impl TimePackage {
   pub fn new() -> Self {
      Self {
         start_time: Instant::now(),
         last_frame: Instant::now(),
         last_data_dump: Instant::now(),
         fps: 0.0,
         past_frames: vec![],
         past_fps: vec![],
         delta_time: 0.0,

         update_interval: UPDATE_INTERVAL,
         past_fps_amount: PAST_FPS_AMOUNT,
      }
   }

   pub fn update(&mut self) {
      if self.last_data_dump.elapsed().as_secs_f64() > self.update_interval {
         self.last_data_dump = Instant::now();
         self.calc_ave_fps();
         self.past_fps.push(self.fps);
         if self.past_fps.len() > self.past_fps_amount {
            let diff = self.past_fps.len() - self.past_fps_amount;
            for _ in 0..diff {
               self.past_fps.remove(0);
            }
         };

         // println!("{}", self.fps);
      }

      self.delta_time = self.last_frame.elapsed().as_secs_f64();
      self.past_frames.push(self.delta_time);
      self.last_frame = Instant::now();
   }

   fn calc_ave_fps(&mut self) {
      let mut total = 0.0;
      for num in &self.past_frames {
         total += num;
      }
      self.fps = 1.0 / (total / self.past_frames.len() as f64);
      self.past_frames.clear();
   }
}