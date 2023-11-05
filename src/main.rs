use winit::{event_loop::EventLoop, event::WindowEvent};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("My Window")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
    let window = window_builder.build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    match input.virtual_keycode {
                        Some(winit::event::VirtualKeyCode::Escape) => *control_flow = winit::event_loop::ControlFlow::Exit,
                        _ => (),
                    }
                },
                WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}