mod event_handler;
mod render;
mod screenshot;
mod vertex;

mod ocr;

use event_handler::EventHandler;
use glium::{glutin, Display};
use render::Render;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_transparent(true)
        .with_decorations(false)
        .with_maximized(false);

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
            color = vec4(0.0, 0.0, 0.0, 0.0);
        }
    "#;

    let mut event_handler = EventHandler::new(&display);
    let render = Render::new(&display, vertex_shader_src, fragment_shader_src);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        let mut target = display.draw();


        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                event_handler.handle_cursor_moved(position);
            }

            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                event_handler.handle_mouse_input(state, button);
            }

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let Some(ocr_result) = event_handler.handle_keyboard_input(input.virtual_keycode) {
                    let x_position_for_text = 100.0; // Example value, adjust as needed.
                    let y_position_for_text = 100.0; // Example value, adjust as needed.

                    render.draw_text(&ocr_result, x_position_for_text, y_position_for_text, &mut target);
                }
            }

            _ => (),
        }

        render.draw_background(&mut target);

        if let Some(start) = event_handler.start_point {
            let end = event_handler
                .end_point
                .unwrap_or(event_handler.current_position);
            render.draw_rectangle(start, end, &mut target);
        }

        target.finish().unwrap();
    });
}
