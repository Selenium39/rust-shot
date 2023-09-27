// renderer.rs

use glium::{Display, Program, Surface, VertexBuffer, Frame};
use super::vertex::Vertex; // 导入 Vertex 结构体
use winit::dpi::PhysicalPosition;

pub struct Renderer {
    display: Display,
    program: Program,
}

impl Renderer {
    pub fn new(display: &Display, vertex_shader_src: &str, fragment_shader_src: &str) -> Self {
        let program = Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();
        Renderer {
            display: display.clone(),
            program,
        }
    }

    pub fn draw_background(&self,target: &mut Frame) {
        target.clear_color(0.5, 0.5, 0.5, 0.5); // 半透明的灰色
    }

    pub fn draw_rectangle(&self, start: PhysicalPosition<f64>,end: PhysicalPosition<f64>,target: &mut Frame) {
        let window = self.display.gl_window();
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

        // Draw the rectangle with correct vertex order
        let vertices = [
            Vertex {
                position: [start_gl[0], start_gl[1]],
            },
            Vertex {
                position: [start_gl[0], end_gl[1]],
            },
            Vertex {
                position: [end_gl[0], start_gl[1]],
            },
            Vertex {
                position: [end_gl[0], end_gl[1]],
            },
        ];

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
        let vertex_buffer = VertexBuffer::new(&self.display, &vertices).unwrap();

        target
            .draw(
                &vertex_buffer,
                    &indices,
                    &self.program,
                    &glium::uniform! {},
                    &Default::default(),
            )
            .unwrap();

    }
}
