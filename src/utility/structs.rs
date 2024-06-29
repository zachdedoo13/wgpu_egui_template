


/// to Ping Or Pong
enum POP {
   First,
   Second,
}

pub struct PingPongData<T> {
   first: T,
   second: T,
   current: POP,
}
impl<T> PingPongData<T> {
   pub fn new(first: T, second: T) -> Self {
      Self {
         first,
         second,
         current: POP::First,
      }
   }

   pub fn pull_current(&self) -> &T {
      // send first
      match self.current {
         POP::First => { &self.first }
         POP::Second => { &self.second }
      }
   }

   pub fn pull_other(&self) -> &T {
      // send not first
      match self.current {
         POP::First => { &self.second }
         POP::Second => { & self.first }
      }
   }

   pub fn ping_pong(&mut self) {
      // swap
      self.current = match self.current {
         POP::First => { POP::Second }
         POP::Second => { POP::First }
      }
   }
}