use glium::{Display, Surface, Program, implement_vertex, VertexBuffer, glutin};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow},
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
                .with_transparent(true)
        .with_decorations(false)
        .with_maximized(true);

    let cb = glutin::ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0); // Red color
        }
    "#;

    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut start_point: Option<PhysicalPosition<f64>> = None;
    let mut end_point: Option<PhysicalPosition<f64>> = None;
    let mut current_position: PhysicalPosition<f64> = PhysicalPosition::new(0.0, 0.0);

    let window_dimensions = display.get_framebuffer_dimensions();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                current_position = position;
                if start_point.is_some() {
                    end_point = Some(current_position);
                }
            },
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if button == MouseButton::Left {
                    if state == ElementState::Pressed {
                        start_point = Some(current_position);
                    } else if start_point.is_some() {
                        end_point = Some(current_position);
                    }
                }
            },
            _ => (),
        }

        // Create a transparent gray background
        let mut target = display.draw();
        target.clear_color(0.5, 0.5, 0.5, 0.5);  // Half-transparent gray

        if let Some(start) = start_point {
            let end = end_point.unwrap_or(current_position);

            // Convert window coordinates to OpenGL coordinates
            let start_gl = [
                2.0 * start.x as f32 / window_dimensions.0 as f32 - 1.0,
                - (2.0 * start.y as f32 / window_dimensions.1 as f32 - 1.0),
            ];
            let end_gl = [
                2.0 * end.x as f32 / window_dimensions.0 as f32 - 1.0,
                - (2.0 * end.y as f32 / window_dimensions.1 as f32 - 1.0),
            ];

            // Draw the red rectangle with correct vertex order
            let vertices = [
                Vertex { position: [start_gl[0], start_gl[1]] },
                Vertex { position: [start_gl[0], end_gl[1]] },
                Vertex { position: [end_gl[0], start_gl[1]] },
                Vertex { position: [end_gl[0], end_gl[1]] },
            ];

            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
            let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();
            target.draw(&vertex_buffer, &indices, &program, &glium::uniform! {}, &Default::default()).unwrap();
        }

        target.finish().unwrap();
    });
}
