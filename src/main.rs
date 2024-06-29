use wgpu_template::inbuilt::event_loop::run;


fn main() {
    pollster::block_on(run())
}