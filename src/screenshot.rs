use screenshots::Screen;

pub struct Screenshot;

impl Screenshot {
    pub fn capture(x: f64, y: f64, width: u32, height: u32) -> Result<String, String> {
        let screen = Screen::from_point(x as i32, y as i32).unwrap();
        let display_id = screen.display_info.id;
        let image_result = screen.capture_area(x as i32, y as i32, width, height);

        match image_result {
            Ok(image) => {
                let path = format!("target/rectangle_{}.png", display_id);
                image.save(&path).unwrap();
                Ok(path)
            },
            Err(_) => {
                Err(format!("截屏失败: {:?}", image_result.err().unwrap()))
            }
        }
    }
}
