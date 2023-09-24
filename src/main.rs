use glium::{Display, Surface, Program, implement_vertex, VertexBuffer, glutin};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow},
};
use winit::event::VirtualKeyCode;
use screenshots::Screen;
use std::time::Instant;

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
            color = vec4(1.0, 0.0, 0.0, 1.0); // Red color
        }
    "#;

    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut start_point: Option<PhysicalPosition<f64>> = None;
    let mut end_point: Option<PhysicalPosition<f64>> = None;
    let mut current_position: PhysicalPosition<f64> = PhysicalPosition::new(0.0, 0.0);



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
            }
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
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: keyboard_input, ..
                },
                ..
            } => {
                if let Some(VirtualKeyCode::Return) = keyboard_input.virtual_keycode {  // 检查是否是 Enter 键
                    if keyboard_input.state == ElementState::Pressed && start_point.is_some() && end_point.is_some() {
                        let start_time = Instant::now();  // 开始计时
                        // 这里是截屏的代码
                        let start = start_point.unwrap();
                        let end = end_point.unwrap();

                        // 保证 start 是左上角，end 是右下角
                        let top_left_x = start.x.min(end.x);
                        let top_left_y = start.y.min(end.y);
                        let bottom_right_x = start.x.max(end.x);
                        let bottom_right_y = start.y.max(end.y);

                        // 计算矩形的宽度和高度
                        let rect_width = (bottom_right_x - top_left_x) as u32;
                        let rect_height = (bottom_right_y - top_left_y) as u32;

                        if rect_width > 0 && rect_height > 0 {

                            // 获取窗口在桌面上的位置和放缩比
                            let window_position = display.gl_window().window().outer_position().unwrap();
                            let scale_factor = display.gl_window().window().scale_factor();

                            let global_top_left_x = (top_left_x + window_position.x as f64) / scale_factor;
                            let global_top_left_y = (top_left_y + window_position.y as f64) / scale_factor;
                            let scaled_rect_width = rect_width as f64 / scale_factor;
                            let scaled_rect_height = rect_height as f64 / scale_factor;

                            let screen = Screen::from_point(global_top_left_x as i32, global_top_left_y as i32).unwrap();
                            let image_result = screen.capture_area(global_top_left_x as i32, global_top_left_y as i32, scaled_rect_width as u32, scaled_rect_height as u32);
                            if let Ok(image) = image_result {
                                image.save(format!("target/rectangle_{}.png", screen.display_info.id)).unwrap();
                            } else {
                                eprintln!("截屏失败: {:?}", image_result.err().unwrap());
                            }
                        }
                        let duration = start_time.elapsed();  // 计算耗时
                        println!("截屏耗时: {:?}", duration);  // 打印耗时
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
            _ => (),
        }

        // Create a transparent gray background
        let mut target = display.draw();
        target.clear_color(0.5, 0.5, 0.5, 0.5);  // Half-transparent gray

        if let Some(start) = start_point {
            let end = end_point.unwrap_or(current_position);

            let window = display.gl_window();
            let size = window.window().inner_size();
            let width = size.width as f32;
            let height = size.height as f32;


            // Convert window coordinates to OpenGL coordinates
            let start_gl = [
                2.0 * start.x as f32 / width - 1.0,
                1.0 - 2.0 * start.y as f32 / height,
            ];
            let end_gl = [
                2.0 * end.x as f32 / width - 1.0,
                1.0 - 2.0 * end.y as f32 / height,
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
