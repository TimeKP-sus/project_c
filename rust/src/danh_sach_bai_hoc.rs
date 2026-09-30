use godot::builtin::{ Array, Dictionary, GString };
use godot::classes::{ Control, DirAccess, FileAccess, IControl };
use godot::global::godot_print;
use godot::{ obj::{ Base }, prelude::{ GodotClass, godot_api } };
use serde::Deserialize;

use crate::choi_sheet::ChoiSheet;

#[derive(Deserialize, Debug)]
pub struct ChuongJson {
    pub ten_chuong: String,
    pub bai_hoc: Vec<BaiHocJson>,
}

#[derive(Deserialize, Debug)]
pub struct BaiHocJson {
    pub ten_bai_hoc: String,
    pub loai_bai_hoc: String,
    pub duong_dan: String,
}

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct DanhSachBaiHoc {
    #[base]
    base: Base<Control>,
}
#[godot_api]
impl DanhSachBaiHoc {
    #[func]
    /// ten_chuong, ten_file
    pub fn lay_danh_sach_chuong(&self) -> Array<Dictionary> {
        let mut danh_sach = Array::<Dictionary>::new();

        if let Some(mut dir) = DirAccess::open("res://bai_hoc/danh_sach") {
            dir.list_dir_begin();
            let mut ten_file = dir.get_next();

            while !ten_file.to_string().is_empty() {
                if !dir.current_is_dir() && ten_file.to_string().ends_with(".json") {
                    let duong_dan = format!("res://bai_hoc/danh_sach/{}", ten_file);
                    let file_text = FileAccess::get_file_as_string(&duong_dan);

                    if let Ok(data) = serde_json::from_str::<ChuongJson>(&file_text.to_string()) {

                        let mut dict = Dictionary::new();
                        dict.set("ten_file", ten_file.clone());
                        dict.set("ten_chuong", data.ten_chuong);

                        danh_sach.push(&dict);
                    }
                }
                ten_file = dir.get_next();
            }
        }

        danh_sach
    }

    #[func]
    /// ten_bai_hoc, loai_bai_hoc, duong_dan
    pub fn lay_json_bai_hoc(&self, ten_chuong: GString) -> Array<Dictionary> {
        let mut danh_sach_bai_hoc: Array<Dictionary> = Array::<Dictionary>::new();
        let duong_dan = format!("res://bai_hoc/danh_sach/{}", ten_chuong);
        godot_print!("Đang đọc file JSON: {}", duong_dan);
        let chuong_json = FileAccess::get_file_as_string(&duong_dan);

        match serde_json::from_str::<ChuongJson>(&chuong_json.to_string()) {
            Ok(du_lieu) => {
                for bai in du_lieu.bai_hoc {
                    let mut dict: Dictionary = Dictionary::new();
                    dict.set("ten_bai_hoc", bai.ten_bai_hoc);
                    dict.set("loai_bai_hoc", bai.loai_bai_hoc);
                    dict.set("duong_dan", bai.duong_dan);
                    danh_sach_bai_hoc.push(&dict);
                }
            }
            Err(e) => godot_print!("Loi json bai hoc: {}", e),
        }
        danh_sach_bai_hoc   
    }
    #[func]
    pub fn mo_bai_hoc(&self, loai_bai_hoc: GString, duong_dan: GString) {
        let duong_dan_str: String = duong_dan.to_string();
        let loai_bai_hoc_str: String = loai_bai_hoc.to_string();

        match loai_bai_hoc_str.as_str() {
            "choi_sheet" => {
            
            }
            "nghe_va_doan_not" => {
                
            }
            "tim_not" => {
                
            }
            _ => godot_print!("Loại bài học ?: {}", loai_bai_hoc_str),
        }

    }
}

#[godot_api]
impl IControl for DanhSachBaiHoc {
}
