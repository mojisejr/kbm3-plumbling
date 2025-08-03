use super::drawing::{draw_bill_split_line, draw_line, draw_vetical_line};
use super::font_util::find_thai_font;
use crate::log;
use crate::model::BillRecord;
use printpdf::*;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

pub fn create_pdf(
    records: &[BillRecord],
    output_path: &str,
    for_month: &str,
) -> Result<(), Box<dyn Error>> {
    log::log_info("เริ่มสร้าง PDF...");

    // สร้าง PDF document
    log::log_info("สร้างเอกสาร PDF ขนาด A5 แนวตั้ง (148mm x 210mm)");
    let (doc, page1, layer1) = PdfDocument::new(
        "ใบเสร็จ",
        Mm(148.0), // A5 width
        Mm(210.0), // A5 height
        "Layer 1",
    );

    // ตั้งค่าฟอนต์ภาษาไทย
    let (font, bold_font, italic_font) = find_thai_font(&doc);
    let font_size_normal = 8.4; // เพิ่ม 20% จาก 7.0
    let font_size_subtitle = 9.6; // เพิ่ม 20% จาก 8.0
    let font_size_title = 12.0; // เพิ่ม 20% จาก 10.0

    // สร้างใบเสร็จ 2 ใบต่อหน้า
    let chunks: Vec<_> = records.chunks(2).collect();
    log::log_info(&format!("จำนวนหน้าทั้งหมด: {}", chunks.len()));

    for (i, record_pair) in chunks.iter().enumerate() {
        // log::log_info(&format!("กำลังสร้างหน้า {}...", i + 1));

        let current_layer = if i == 0 {
            doc.get_page(page1).get_layer(layer1)
        } else {
            log::log_debug("สร้างหน้าใหม่");
            let (page, layer) = doc.add_page(Mm(148.0), Mm(210.0), format!("Page {}", i + 1));
            doc.get_page(page).get_layer(layer)
        };

        // ตำแหน่ง Y offset สำหรับใบเสร็จแต่ละใบในหน้านี้
        let y_offset_top_bill = Mm(190.0); // ลดลงจาก 195.0
        let y_offset_bottom_bill = Mm(100.0); // ลดลงจาก 105.0

        for (j, bill) in record_pair.iter().enumerate() {
            let y_offset = if j == 0 {
                y_offset_top_bill
            } else {
                y_offset_bottom_bill
            };
            log::log_debug(&format!(
                "สร้างใบเสร็จที่ {} ในหน้า {} (y_offset: {})",
                j + 1,
                i + 1,
                y_offset.0
            ));

            // เขียนข้อมูลใบเสร็จ
            log::log_debug(&format!("เขียนข้อมูล: {} - {}", bill.meter_number, bill.name));

            // หัวข้อใหญ่ - ย้ายไปตรงกลาง
            current_layer.use_text(
                "การประปาหมู่บ้าน หมู่ 3",
                font_size_title,
                Mm(50.0), // ตำแหน่ง X ตรงกลาง (148mm/2 - 24mm)
                y_offset,
                &bold_font,
            );

            // ที่อยู่ - ย้ายไปตรงกลาง
            current_layer.use_text(
                "ต.คมบาง อ.เมือง จ.จันทบุรี",
                font_size_subtitle,
                Mm(52.0),           // ตำแหน่ง X ตรงกลางเหมือนหัวข้อใหญ่
                y_offset - Mm(7.0), // ปรับระยะห่าง
                &bold_font,
            );

            // วันที่ออกบิล (ตัวอย่าง)
            current_layer.use_text(
                "วันออกบิล",
                font_size_normal,
                Mm(15.0),            // ตำแหน่ง X
                y_offset - Mm(22.0), // ปรับระยะห่าง
                &bold_font,
            );
            current_layer.use_text(
                format!(
                    "{} {} {}",
                    chrono::Local::now().format("%d").to_string(),
                    match chrono::Local::now().format("%m").to_string().as_str() {
                        "01" => "ม.ค.",
                        "02" => "ก.พ.",
                        "03" => "มี.ค.",
                        "04" => "เม.ย.",
                        "05" => "พ.ค.",
                        "06" => "มิ.ย.",
                        "07" => "ก.ค.",
                        "08" => "ส.ค.",
                        "09" => "ก.ย.",
                        "10" => "ต.ค.",
                        "11" => "พ.ย.",
                        "12" => "ธ.ค.",
                        _ => "",
                    },
                    (chrono::Local::now()
                        .format("%Y")
                        .to_string()
                        .parse::<i32>()
                        .unwrap()
                        + 543)
                        .to_string()
                ),
                font_size_normal,
                Mm(35.0),            // ตำแหน่ง X ถัดมา
                y_offset - Mm(22.0), // ลดระยะห่างลง 20%
                &italic_font,
            );

            //เลข ID user
            current_layer.use_text(
                "id.",
                font_size_normal,
                Mm(95.0),
                y_offset - Mm(10.0),
                &bold_font,
            );
            current_layer.use_text(
                &bill.order.to_string(),
                font_size_normal,
                Mm(115.0),
                y_offset - Mm(10.0),
                &italic_font,
            );

            // ประจำเดือน (ตัวอย่าง)
            current_layer.use_text(
                "ประจำเดือน",
                font_size_normal,
                Mm(95.0),            // ตำแหน่ง X ทางขวา
                y_offset - Mm(15.0), // ลดระยะห่างลง 20%
                &bold_font,
            );
            current_layer.use_text(
                for_month, // ใช้ค่าคงที่ไปก่อน
                font_size_normal,
                Mm(115.0),           // ตำแหน่ง X ถัดมา
                y_offset - Mm(15.0), // ลดระยะห่างลง 20%
                &italic_font,
            );

            // ชื่อ-นามสกุล
            current_layer.use_text(
                "ชื่อ-นามสกุล",
                font_size_normal,
                Mm(15.0),            // ตำแหน่ง X ถัดมา
                y_offset - Mm(27.0), // ลดระยะห่างลง 20%
                &bold_font,
            );
            current_layer.use_text(
                &bill.name,
                font_size_normal,
                Mm(35.0),            // ตำแหน่ง X ถัดมา
                y_offset - Mm(27.0), // ลดระยะห่างลง 20%
                &italic_font,
            );

            // หัวตาราง
            let table_header_y = y_offset - Mm(35.0); // ปรับระยะห่าง
            current_layer.use_text(
                "หมายเลขมิเตอร์",
                font_size_normal,
                Mm(15.0),
                table_header_y,
                &bold_font,
            );
            current_layer.use_text(
                "เลขอ่านครั้งหลัง",
                font_size_normal,
                Mm(45.0),
                table_header_y,
                &bold_font,
            );
            current_layer.use_text(
                "เลขอ่านครั้งก่อน",
                font_size_normal,
                Mm(70.0),
                table_header_y,
                &bold_font,
            );
            current_layer.use_text(
                "จำนวนหน่วย",
                font_size_normal,
                Mm(95.0),
                table_header_y,
                &bold_font,
            );
            current_layer.use_text(
                "จำนวนเงิน",
                font_size_normal,
                Mm(115.0),
                table_header_y,
                &bold_font,
            );

            // ข้อมูลในตาราง
            let table_data_y_start = y_offset - Mm(43.0); // ปรับระยะห่าง
            let table_data_y_line_height = Mm(6.5); // ปรับระยะห่าง

            // แถวข้อมูลมิเตอร์
            current_layer.use_text(
                &bill.meter_number,
                font_size_normal,
                Mm(15.0),
                table_data_y_start,
                &font,
            );
            current_layer.use_text(
                &bill.current_reading.to_string(),
                font_size_normal,
                Mm(45.0),
                table_data_y_start,
                &font,
            );
            current_layer.use_text(
                &bill.previous_reading.to_string(),
                font_size_normal,
                Mm(70.0),
                table_data_y_start,
                &font,
            );

            // แถวค่าบำรุง
            current_layer.use_text(
                "ค่าบำรุง",
                font_size_normal,
                Mm(95.0),
                table_data_y_start,
                &bold_font,
            );
            current_layer.use_text(
                &bill.maintenance_fee.to_string(),
                font_size_normal,
                Mm(115.0),
                table_data_y_start,
                &font,
            );

            // แถวจำนวนหน่วยและเงิน
            current_layer.use_text(
                &bill.units.to_string(),
                font_size_normal,
                Mm(95.0),
                table_data_y_start - table_data_y_line_height,
                &font,
            );
            // คำนวณค่าหน่วย (หน่วย * หน่วยละ)
            let units_cost = bill.units * bill.rate_per_unit;
            current_layer.use_text(
                &units_cost.to_string(),
                font_size_normal,
                Mm(115.0),
                table_data_y_start - table_data_y_line_height,
                &font,
            );

            // แถวรวม
            current_layer.use_text(
                "รวม",
                font_size_normal,
                Mm(95.0),
                table_data_y_start - table_data_y_line_height * 2.0,
                &bold_font,
            );
            current_layer.use_text(
                &bill.total_amount.to_string(),
                font_size_normal,
                Mm(115.0),
                table_data_y_start - table_data_y_line_height * 2.0,
                &font,
            );

            // สามารถเพิ่มการวาดเส้นเพื่อทำตารางได้ที่นี่
            //เส้นใต้ข้อมูล
            let split_line = draw_bill_split_line();
            let id_underline = draw_line(y_offset - Mm(11.0), Mm(114.0), Mm(122.0));
            let month_underline = draw_line(y_offset - Mm(16.0), Mm(114.0), Mm(122.0));
            let bill_date_underline = draw_line(y_offset - Mm(23.0), Mm(30.0), Mm(70.0));
            let name_underline = draw_line(y_offset - Mm(28.0), Mm(30.0), Mm(70.0));

            current_layer.add_line(id_underline);
            current_layer.add_line(month_underline);
            current_layer.add_line(bill_date_underline);
            current_layer.add_line(name_underline);
            current_layer.add_line(split_line);

            //เส้นตาราง
            let header_line_top = draw_line(y_offset - Mm(31.0), Mm(13.0), Mm(128.0));
            let header_line_bottom = draw_line(y_offset - Mm(36.0), Mm(13.0), Mm(128.0));
            let middle_line = draw_line(y_offset - Mm(51.0), Mm(13.0), Mm(128.0));
            let vertical_line_1 = draw_vetical_line(Mm(92.0), Mm(159.5), Mm(130.0));
            let vertical_line_2 = draw_vetical_line(Mm(92.0), Mm(69.5), Mm(40.0));

            current_layer.add_line(header_line_top);
            current_layer.add_line(header_line_bottom);
            current_layer.add_line(middle_line);
            current_layer.add_line(vertical_line_1);
            current_layer.add_line(vertical_line_2);
        }
    }

    // บันทึกไฟล์ PDF
    log::log_info(&format!("กำลังบันทึกไฟล์ PDF: {}", output_path));
    doc.save(&mut BufWriter::new(File::create(output_path)?))?;
    log::log_info("บันทึกไฟล์ PDF สำเร็จ!");
    Ok(())
}
