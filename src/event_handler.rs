use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, VirtualKeyCode},
};

use super::screenshot::Screenshot;
use glium::Display;

pub struct EventHandler {
    pub start_point: Option<PhysicalPosition<f64>>,
    pub end_point: Option<PhysicalPosition<f64>>,
    pub current_position: PhysicalPosition<f64>,
    pub is_dragging: bool,
    pub is_moving: bool,
    pub start_drag_point: Option<PhysicalPosition<f64>>,
    display: Display,
}

impl EventHandler {
    pub fn new(display: &Display) -> Self {
        EventHandler {
            start_point: None,
            end_point: None,
            current_position: PhysicalPosition::new(0.0, 0.0),
            is_dragging: false,
            is_moving: false,
            start_drag_point: None,
            display:display.clone(),
        }
    }

    pub fn handle_cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        self.current_position = position;
        if self.is_dragging && self.start_point.is_some() {
            self.end_point = Some(self.current_position);
        } else if self.is_moving {
            if let Some(start_drag) = self.start_drag_point {
                let dx = self.current_position.x - start_drag.x;
                let dy = self.current_position.y - start_drag.y;
                self.start_point = Some(PhysicalPosition::new(
                    self.start_point.unwrap().x + dx,
                    self.start_point.unwrap().y + dy,
                ));
                self.end_point = Some(PhysicalPosition::new(
                    self.end_point.unwrap().x + dx,
                    self.end_point.unwrap().y + dy,
                ));
            }
            self.start_drag_point = Some(self.current_position);
        }
    }

    pub fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            if state == ElementState::Pressed {
                if let (Some(start), Some(end)) = (self.start_point, self.end_point) {
                    if Self::is_point_inside_rect(
                        self.current_position,
                        PhysicalPosition::new(start.x.min(end.x), start.y.min(end.y)),
                        PhysicalPosition::new(start.x.max(end.x), start.y.max(end.y)),
                    ) {
                        self.is_moving = true;
                        self.is_dragging = false;
                        self.start_drag_point = Some(self.current_position);
                    } else {
                        self.start_point = Some(self.current_position);
                        self.is_dragging = true;
                        self.is_moving = false;
                        self.start_drag_point = None;
                    }
                } else {
                    self.start_point = Some(self.current_position);
                    self.is_dragging = true;
                    self.is_moving = false;
                    self.start_drag_point = None;
                }
            } else {
                self.is_dragging = false;
                self.is_moving = false;
                self.start_drag_point = None;
            }
        }
    }

    pub fn handle_keyboard_input(&mut self, virtual_keycode: Option<VirtualKeyCode>) -> bool {
        if let Some(VirtualKeyCode::Return) = virtual_keycode {
            if let (Some(start), Some(end)) = (self.start_point, self.end_point) {

                // 保证 start 是左上角，end 是右下角
                let top_left_x = start.x.min(end.x);
                let top_left_y = start.y.min(end.y);
                let bottom_right_x = start.x.max(end.x);
                let bottom_right_y = start.y.max(end.y);

                // 计算矩形的宽度和高度
                let rect_width = (bottom_right_x - top_left_x) as u32;
                let rect_height = (bottom_right_y - top_left_y) as u32;

                if rect_width > 0 && rect_height > 0 {
                    let window_position = self.display.gl_window().window().outer_position().unwrap();
                    let scale_factor = self.display.gl_window().window().scale_factor();
                    
                    let global_top_left_x = (top_left_x + window_position.x as f64) / scale_factor;
                    let global_top_left_y = (top_left_y + window_position.y as f64) / scale_factor;
                    let scaled_rect_width = rect_width as f64 / scale_factor;
                    let scaled_rect_height = rect_height as f64 / scale_factor;

                    // Capture the screenshot
                    match Screenshot::capture(
                        global_top_left_x,
                        global_top_left_y,
                        scaled_rect_width as u32,
                        scaled_rect_height as u32,
                    ) {
                        Ok(_) => println!("Screenshot captured successfully!"),
                        Err(e) => eprintln!("Failed to capture screenshot: {}", e),
                    }

                    return true;
                }
            }
        }
        false
    }

    fn is_point_inside_rect(
        point: PhysicalPosition<f64>,
        top_left: PhysicalPosition<f64>,
        bottom_right: PhysicalPosition<f64>,
    ) -> bool {
        point.x >= top_left.x
            && point.x <= bottom_right.x
            && point.y >= top_left.y
            && point.y <= bottom_right.y
    }
}
