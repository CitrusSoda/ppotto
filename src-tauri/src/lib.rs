use image::ImageOutputFormat;
use std::io::Cursor;
use std::path::Path;

/// 이미지 리사이징 Tauri 커맨드
/// path: 이미지 파일 경로
/// width: 변경할 너비
/// height: 변경할 높이
#[tauri::command]
fn resize_image(path: String, width: u32, height: u32) -> Result<Vec<u8>, String> {
    let img = image::open(&Path::new(&path)).map_err(|e| format!("Failed to open image: {}", e))?;
    let resized_img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

    // 이미지를 메모리 버퍼에 기록하기 위해 Cursor를 생성합니다.
    // Cursor는 Vec<u8>와 같은 메모리 버퍼를 읽고 쓰는 데 사용됩니다.
    let mut buffer = Cursor::new(Vec::new());

    // 리사이즈된 이미지를 PNG 형식으로 버퍼에 기록합니다.
    resized_img
        .write_to(&mut buffer, ImageOutputFormat::Png)
        .map_err(|e| format!("Failed to encode image: {}", e))?;

    // 버퍼에서 내부 Vec<u8> (이미지 데이터)를 추출하여 반환합니다.
    Ok(buffer.into_inner())
}

/// 이미지 자르기 Tauri 커맨드
/// path: 이미지 파일 경로
/// x, y: 자르기 시작점 좌표
/// width, height: 자르기 영역 크기
#[tauri::command]
fn crop_image(path: String, x: u32, y: u32, width: u32, height: u32) -> Result<Vec<u8>, String> {
    // 이미지 파일 열기
    let mut img = image::open(&Path::new(&path)).map_err(|e| format!("Failed to open image: {}", e))?;

    // 이미지 자르기
    let cropped_img = img.crop_imm(x, y, width, height);

    // 잘라낸 이미지를 PNG 형식의 바이트 벡터로 인코딩
    let mut buffer = Cursor::new(Vec::new());
    cropped_img.write_to(&mut buffer, ImageOutputFormat::Png)
        .map_err(|e| format!("Failed to encode image: {}", e))?;

    // 바이트 벡터 반환
    Ok(buffer.into_inner())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![resize_image, crop_image]) // 새로운 커맨드 추가
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
