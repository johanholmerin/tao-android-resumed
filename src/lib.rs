use tao::{event_loop::EventLoop, window::WindowBuilder};

#[allow(unused)]
pub fn run() {
    let event_loop = EventLoop::new();
    let mut window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        println!("event: {event:#?}");
    });
}

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
fn main() {
    run();
}
