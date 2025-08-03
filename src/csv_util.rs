use crate::log;
use crate::model::BillRecord;
use std::error::Error;

pub fn read_csv_file(file_path: &str) -> Result<Vec<BillRecord>, Box<dyn Error>> {
    log::log_info(&format!("กำลังอ่านไฟล์ CSV: {}", file_path));
    let mut reader = csv::Reader::from_path(file_path)?;
    let records: Vec<BillRecord> = reader.deserialize().collect::<Result<_, _>>()?;
    log::log_info(&format!("อ่านข้อมูลสำเร็จ จำนวน {} รายการ", records.len()));

    for (i, record) in records.iter().enumerate() {
        log::log_debug(&format!(
            "รายการที่ {}: {} - {}",
            i + 1,
            record.meter_number,
            record.name
        ));
    }
    Ok(records)
}
