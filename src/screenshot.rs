use screenshots::Screen;

pub struct Screenshot;

impl Screenshot {
    pub fn capture(x: f64, y: f64, width: u32, height: u32) -> Result<(), String> {
        let screen = Screen::from_point(x as i32, y as i32).unwrap();
        let display_id = screen.display_info.id;
        let image_result = screen.capture_area(x as i32, y as i32, width, height);
        if let Ok(image) = image_result {
            image
                .save(format!("target/rectangle_{}.png", display_id))
                .unwrap();
            Ok(())
        } else {
            Err(format!("截屏失败: {:?}", image_result.err().unwrap()))
        }
    }
}
