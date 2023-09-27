use reqwest;
use serde_derive::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use std::fs;
use base64;
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct ApiResponse {
    ParsedResults: Vec<ParsedResult>,
    SearchablePDFURL: Option<String>,
    OCRExitCode: Option<i32>,
}

#[derive(Deserialize, Serialize)]
struct ParsedResult {
    ParsedText: String,
}

pub struct OCRProcessor {
    api_keys: Vec<String>,
    api_url: String,
}

impl OCRProcessor {
    pub fn new() -> Self {
        const DEFAULT_API_KEYS: [&str; 7] = ["fca5393dd988957", "K84490092188957","9f368da3dd88957","1f666673b488957","ec2a41c6a188957","3c5a17b91d88957","d35acb001e88957"];
        const DEFAULT_API_URL: &str = "https://api.ocr.space/parse/image";

        OCRProcessor {
            api_keys: DEFAULT_API_KEYS.iter().map(|s| s.to_string()).collect(),
            api_url: DEFAULT_API_URL.to_string(),
        }
    }

    pub fn with_keys(api_keys: Vec<String>, api_url: &str) -> Self {
        OCRProcessor {
            api_keys,
            api_url: api_url.to_string(),
        }
    }

    pub fn process_image(&self, screenshot_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let selected_api_key = self.api_keys.choose(&mut rand::thread_rng()).unwrap_or(&self.api_keys[0]);

        // 读取图像文件并进行Base64编码
        let image_bytes = fs::read(screenshot_path)?;
        let encoded_image = base64::encode(&image_bytes);

        // 根据文件扩展名确定内容类型
        let ext = Path::new(screenshot_path).extension().and_then(std::ffi::OsStr::to_str);
        let content_type = match ext {
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            // Add more content types if needed
            _ => return Err("Unsupported file extension".into()),
        };

        let base64_prefixed = format!("data:{};base64,{}", content_type, encoded_image);

        let params = [
            ("apikey", selected_api_key.as_str()),
            ("base64Image", &base64_prefixed),
            ("language", "eng"),
            ("isOverlayRequired", "false"),
            ("iscreatesearchablepdf", "true"),
        ];

        let response_body = client.post(&self.api_url).form(&params).send()?.text()?;

        let response: ApiResponse = serde_json::from_str(&response_body)?;

        if let Some(parsed_result) = response.ParsedResults.get(0) {
            Ok(parsed_result.ParsedText.clone())
        } else {
            Err("Failed to get parsed text from OCR".into())
        }
    }
}
