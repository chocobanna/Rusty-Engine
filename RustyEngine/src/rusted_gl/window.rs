use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn create_window(width: f64, height: f64) {
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new()
        .with_title("Adjustable Window")
        .with_resizable(true)
        .with_inner_size(LogicalSize::new(width, height))
        .build(&event_loop)
        .expect("Failed to create window");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *control_flow = ControlFlow::Exit;
        }
    });
}
