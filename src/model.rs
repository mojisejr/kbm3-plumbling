use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BillRecord {
    #[serde(rename = "ลำดับ")]
    pub order: u32,
    #[serde(rename = "เลขมิเตอร์")]
    pub meter_number: String,
    #[serde(rename = "ชื่อ")]
    pub name: String,
    #[serde(rename = "เลขอ่านครั้งก่อน")]
    pub previous_reading: u32,
    #[serde(rename = "เลขอ่านครั้งหลัง")]
    pub current_reading: u32,
    #[serde(rename = "หน่วย")]
    pub units: u32,
    #[serde(rename = "20")]
    pub maintenance_fee: u32,
    #[serde(rename = "หนวยละ")]
    pub rate_per_unit: u32,
    #[serde(rename = "จำนวน")]
    pub total_amount: u32,
}
