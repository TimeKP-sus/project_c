use godot::classes::FileAccess;
use godot::global::godot_print;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BaiHatJson {
    pub bpm: f32,
    pub toc_do: f32,
    pub khuong_1: Vec<NotNhacJson>,
    pub khuong_2: Vec<NotNhacJson>,
}

#[derive(Deserialize, Debug)]
pub struct NotNhacJson {
    pub id_not: i32,
    pub ten_not: String,
    pub mau_not: String,
    pub vi_tri_so: i32,
    pub nhip_dich: f32,
    pub nhip_giu: f32,
}

// Hàm này trả về Option<BaiHatJson>, nếu lỗi sẽ trả về None
pub fn doc_file_json(duong_dan: &str) -> Option<BaiHatJson> {
    let file_text = FileAccess::get_file_as_string(duong_dan);

    if file_text.to_string().is_empty() {
        godot_print!("Không thể đọc hoặc file trống {}", duong_dan);
        return None;
    }

    match serde_json::from_str::<BaiHatJson>(&file_text.to_string()) {
        Ok(bai_hat) => Some(bai_hat),
        Err(loi) => {
            godot_print!("Lỗi cú pháp file JSON: {}", loi);
            None
        }
    }
}