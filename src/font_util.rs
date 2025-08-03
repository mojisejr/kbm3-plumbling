use crate::log;
use font_kit::font;
use printpdf::*;
use reqwest::blocking::Client;
use std::error::Error;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub fn get_fonts_dir() -> PathBuf {
    let mut fonts_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    fonts_dir.push("papa_moo_3_bills");
    fonts_dir.push("fonts");
    fs::create_dir_all(&fonts_dir).expect("Failed to create fonts directory");
    log::log_debug(&format!("Font directory: {:?}", fonts_dir));
    fonts_dir
}

pub fn download_font(font_name: &str, style: &str) -> Result<PathBuf, Box<dyn Error>> {
    let fonts_dir = get_fonts_dir();
    let font_path = fonts_dir.join(format!("{}-{}.ttf", font_name, style));

    // ถ้ามีไฟล์อยู่แล้ว ไม่ต้องดาวน์โหลดใหม่
    if font_path.exists() {
        log::log_debug(&format!("Font already exists: {:?}", font_path));
        return Ok(font_path);
    }

    // ดาวน์โหลดฟอนต์จาก Google Fonts
    log::log_info(&format!("Downloading font: {} ({})", font_name, style));
    let client = Client::new();
    let url = format!(
        "https://fonts.googleapis.com/css2?family={}:wght@400;700&display=swap",
        font_name.replace(" ", "+")
    );

    let response = client.get(&url).send()?;
    let css = response.text()?;

    // แยก URL ของฟอนต์จาก CSS
    let font_url = css
        .lines()
        .find(|line| line.contains("src: url("))
        .and_then(|line| {
            line.split("url(")
                .nth(1)?
                .split(")")
                .next()
                .map(|s| s.trim_matches(|c| c == '\'' || c == '"'))
        })
        .ok_or_else(|| "Failed to extract font URL from CSS")?;

    // ดาวน์โหลดไฟล์ฟอนต์
    log::log_debug(&format!("Downloading font from URL: {}", font_url));
    let font_data = client.get(font_url).send()?.bytes()?;
    fs::write(&font_path, font_data)?;
    log::log_info(&format!("Font downloaded successfully: {:?}", font_path));

    Ok(font_path)
}

pub fn find_thai_font(
    doc: &PdfDocumentReference,
) -> (IndirectFontRef, IndirectFontRef, IndirectFontRef) {
    // ลองดาวน์โหลดฟอนต์จาก Google Fonts ก่อน
    let font_candidates = ["Sarabun", "Noto Sans Thai", "Prompt", "Kanit"];
    log::log_info("กำลังค้นหา font ภาษาไทย จาก Google Fonts...");

    for font_name in font_candidates.iter() {
        match (
            download_font(font_name, "Regular"),
            download_font(font_name, "Bold"),
        ) {
            (Ok(regular_path), Ok(bold_path)) => {
                log::log_info(&format!("ใช้ font {} จาก Google Fonts", font_name));
                let regular_font = doc
                    .add_external_font(File::open(&regular_path).unwrap())
                    .unwrap();
                let bold_font = doc
                    .add_external_font(File::open(&bold_path).unwrap())
                    .unwrap();
                let italic_font = doc
                    .add_external_font(File::open(&regular_path).unwrap())
                    .unwrap();
                return (regular_font, bold_font, italic_font);
            }
            _ => continue,
        }
    }

    // ถ้าไม่สามารถดาวน์โหลดได้ ให้ใช้ฟอนต์ในระบบ
    log::log_info("ค้นหา font ในระบบ...");
    let font_candidates = [
        // Windows paths
        (
            "Angsana New",
            "C:/Windows/Fonts/ANGSA.TTF",
            "C:/Windows/Fonts/ANGSAB.TTF",
            "C:/Windows/Fonts/ANGSAI.TTF",
        ),
        (
            "Cordia New",
            "C:/Windows/Fonts/CORDIA.TTF",
            "C:/Windows/Fonts/CORDIAB.TTF",
            "C:/Windows/Fonts/CORDIAI.TTF",
        ),
        (
            "Tahoma",
            "C:/Windows/Fonts/TAHOMA.TTF",
            "C:/Windows/Fonts/TAHOMABD.TTF",
            "C:/Windows/Fonts/TAHOMA.TTF",
        ),
        // macOS paths
        (
            "Angsana New",
            "/System/Library/Fonts/Supplemental/Angsana.ttc",
            "/System/Library/Fonts/Supplemental/AngsanaBold.ttc",
            "/System/Library/Fonts/Supplemental/AngsanaItalic.ttc",
        ),
        (
            "Sarabun",
            "/System/Library/Fonts/Supplemental/Sarabun.ttc",
            "/System/Library/Fonts/Supplemental/SarabunBold.ttc",
            "/System/Library/Fonts/Supplemental/SarabunItalic.ttc",
        ),
        (
            "Noto Sans Thai",
            "/System/Library/Fonts/Supplemental/NotoSansThai.ttc",
            "/System/Library/Fonts/Supplemental/NotoSansThaiBold.ttc",
            "/System/Library/Fonts/Supplemental/NotoSansThaiItalic.ttc",
        ),
    ];

    for (name, regular_path, bold_path, italic_path) in font_candidates.iter() {
        if Path::new(regular_path).exists()
            && Path::new(bold_path).exists()
            && Path::new(italic_path).exists()
        {
            log::log_info(&format!("Using system font {} ({})", name, regular_path));
            let regular_font = doc
                .add_external_font(File::open(regular_path).unwrap())
                .unwrap();
            let bold_font = doc
                .add_external_font(File::open(bold_path).unwrap())
                .unwrap();
            let italic_font = doc
                .add_external_font(File::open(italic_path).unwrap())
                .unwrap();
            return (regular_font, bold_font, italic_font);
        }
    }

    // Fallback to system fonts if no Thai fonts are found
    log::log_warn("No Thai fonts found, using Helvetica as fallback");
    let regular_font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let bold_font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let italic_font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    (regular_font, bold_font, italic_font)
}
