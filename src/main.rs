use csv_util::read_csv_file;
use pdf_util::create_pdf;
use std::error::Error;

mod csv_util;
mod drawing;
mod font_util;
mod log;
mod model;
mod pdf_util;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    log::init_logger();

    //----- INPUT DATA
    let input_path = "bills_jul_68.csv";
    let for_month = "ก.ค.";
    let bill_name = "bills/plumbing_jul_68.pdf";
    //----- INPUT DATA

    log::log_info("=== เริ่มโปรแกรมสร้างใบเสร็จ ===");

    let records = read_csv_file(input_path)?;
    log::log_info(&format!("อ่านข้อมูลจาก CSV สำเร็จ: {} รายการ", records.len()));

    create_pdf(&records, bill_name, for_month)?;
    log::log_info("สร้างไฟล์ PDF สำเร็จ");

    log::log_info("=== จบการทำงาน ===");
    Ok(())
}
