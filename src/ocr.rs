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

pub struct OCRProcessor;

impl OCRProcessor {
    const DEFAULT_API_KEYS: [&'static str; 7] = ["fca5393dd988957", "K84490092188957","9f368da3dd88957","1f666673b488957","ec2a41c6a188957","3c5a17b91d88957","d35acb001e88957"];
    const DEFAULT_API_URL: &'static str = "https://api.ocr.space/parse/image";

    pub fn ocr(screenshot_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let selected_api_key = OCRProcessor::DEFAULT_API_KEYS.choose(&mut rand::thread_rng()).unwrap_or(&OCRProcessor::DEFAULT_API_KEYS[0]);
        let api_key_str = *selected_api_key;

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
            ("apikey", api_key_str),
            ("base64Image", &base64_prefixed),
            ("language", "eng"),
            ("isOverlayRequired", "false"),
            ("iscreatesearchablepdf", "true"),
        ];

        let response_body = client.post(Self::DEFAULT_API_URL).form(&params).send()?.text()?;

        let response: ApiResponse = serde_json::from_str(&response_body)?;

        if let Some(parsed_result) = response.ParsedResults.get(0) {
            println!("Ocr Result:{}",parsed_result.ParsedText);
            Ok(parsed_result.ParsedText.clone())
        } else {
            Err("Failed to get parsed text from OCR".into())
        }
    }
}
