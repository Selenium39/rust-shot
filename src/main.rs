mod event_handler;
mod render;
mod screenshot;
mod vertex;

mod ocr;

mod chat;

use event_handler::EventHandler;
use glium::{glutin, Display};
use render::Render;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use glium::Surface;
use chat::ChatService;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_transparent(true)
        .with_decorations(false)
        .with_maximized(false);

    let cb = glutin::ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    let mut event_handler = EventHandler::new(&display);
    let render = Render::new(&display);

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
                    let gl_window = display.gl_window();
                    let window = gl_window.window();

                    // 获取窗口的位置和大小
                    let position = window.outer_position().unwrap();
                    let size = window.outer_size();
                    // 获取 DPI 缩放因子
                    let scale_factor = window.scale_factor();

                    // 考虑 DPI 缩放因子来调整窗口大小
                    let adjusted_size = (
                        (size.width as f64 / scale_factor) as u32,
                        (size.height as f64 / scale_factor) as u32
                    );

                    let adjusted_position = (
                        (position.x as f64 / scale_factor) as i32,
                        (position.x as f64 / scale_factor) as i32
                    );

                    window.set_visible(false);

                    *control_flow = ControlFlow::Exit;

                    render.draw_ocr_result(ocr_result, (adjusted_position.0, adjusted_position.1), (adjusted_size.0 as i32, adjusted_size.1 as i32));
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
