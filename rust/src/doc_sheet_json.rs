use godot::classes::FileAccess;
use godot::global::godot_print;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BaiHatJson {
    pub ten_bai_hat: String,
    pub tac_gia: String,
    pub nguoi_tao_sheet: String,
    pub loai_vach_nhip: String,
    pub bpm: f32,
    pub mien_not_bat_dau_k1: i32, // Khuông 1
    pub mien_not_bat_dau_k2: i32, // Khuông 2
    pub khuong_1: Vec<NotNhacJson>,
    pub khuong_2: Vec<NotNhacJson>,
}

#[derive(Deserialize, Debug)]
pub struct NotNhacJson {
    pub id_not: i32,
    pub mau_not: String,
    pub nhip_dich: f32,
    #[serde(default)]
    pub nhip_giu: f32,
    #[serde(default)]
    pub chuyen_not: i32, // Sẽ mang giá trị 0 nếu không được ghi trong JSON
}

pub fn doc_file_json(duong_dan: &str) -> Option<BaiHatJson> {
    let file_text = FileAccess::get_file_as_string(duong_dan);
    if file_text.to_string().is_empty() { return None; }
    match serde_json::from_str::<BaiHatJson>(&file_text.to_string()) {
        Ok(bai_hat) => Some(bai_hat),
        Err(loi) => { godot_print!("Lỗi JSON: {}", loi); None }
    }
}


#[derive(Deserialize, Debug)]
pub struct LuyenTapJson {
    pub ten_bai_hoc: String,
    pub duong_dan_sheet: String,
    pub khuong: String,
    pub cho_phep_cho: bool,
    pub phan_thoi_gian: String,
    // Note: cac_ghi_chu can be implemented later with a HashMap if needed
}

pub fn doc_file_luyen_tap(duong_dan: &str) -> Option<LuyenTapJson> {
    let file_text = FileAccess::get_file_as_string(duong_dan);

    if file_text.to_string().is_empty() {
        godot_print!("Không thể đọc hoặc file trống {}", duong_dan);
        return None;
    }

    match serde_json::from_str::<LuyenTapJson>(&file_text.to_string()) {
        Ok(bai_hoc) => Some(bai_hoc),
        Err(loi) => {
            godot_print!("Lỗi cú pháp file JSON luyện tập: {}", loi);
            None
        }
    }
}