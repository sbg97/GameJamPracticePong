use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::Key::Named,
    keyboard::NamedKey,
    window::WindowBuilder,
};

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::KeyboardInput { ref event, .. } => match event.logical_key {
                Named(NamedKey::Escape) => elwt.exit(),
                _ => {}
            },
            _ => {}
        },
        _ => {}
    });
}
