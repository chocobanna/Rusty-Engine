use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

/// Creates a window with the specified dimensions and draws a line.
pub fn draw_line(width: u32, height: u32) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Line Drawing Window")
        .with_inner_size(LogicalSize::new(width, height))
        .with_resizable(true)
        .build(&event_loop)
        .expect("Failed to create window");

    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(width, height, surface_texture)
        .expect("Failed to create pixel buffer");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::RedrawRequested(_) => {
                // Use get_frame_mut() to get a mutable reference
                let frame = pixels.get_frame_mut();

                // Clear the frame (black background)
                for pixel in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[0, 0, 0, 255]);
                }

                // Draw a red line from (10,10) to (width-10, height-10)
                draw_line_on_frame(
                    frame,
                    width,
                    height,
                    10,
                    10,
                    width as i32 - 10,
                    height as i32 - 10,
                    [255, 0, 0, 255],
                );

                pixels.render().expect("Failed to render frame");
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}

/// Draws a line on the frame buffer using Bresenham's algorithm.
fn draw_line_on_frame(
    frame: &mut [u8],
    frame_width: u32,
    frame_height: u32,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: [u8; 4],
) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && x < frame_width as i32 && y >= 0 && y < frame_height as i32 {
            let idx = (y as u32 * frame_width + x as u32) as usize * 4;
            if idx + 3 < frame.len() {
                frame[idx..idx+4].copy_from_slice(&color);
            }
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
