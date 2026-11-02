use godot::builtin::{Array, Dictionary, GString};
use godot::classes::{Control, DirAccess, FileAccess, IControl};
use godot::global::godot_print;
use godot::{
    obj::Base,
    prelude::{GodotClass, godot_api},
};
use serde::Deserialize;

use crate::du_lieu_lich_su::DuLieuLichSu;

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
    #[serde(default)]
    pub nguoi_tao: String,
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
        let mut ds_ten_file: Vec<String> = Vec::new();

        if let Some(mut dir) = DirAccess::open("res://bai_hoc/danh_sach") {
            dir.list_dir_begin();
            let mut ten_file: GString = dir.get_next();

            while !ten_file.to_string().is_empty() {
                let ten_str: String = ten_file.to_string();
                if !dir.current_is_dir() && ten_str.ends_with(".json") {
                    ds_ten_file.push(ten_str);
                }
                ten_file = dir.get_next();
            }
        }

        // Sắp xếp tên file tăng dần để các chương luôn hiển thị đúng thứ tự
        ds_ten_file.sort();

        for ten_file in ds_ten_file {
            let duong_dan: String = format!("res://bai_hoc/danh_sach/{}", ten_file);
            let file_text: GString = FileAccess::get_file_as_string(&duong_dan);

            if let Ok(data) = serde_json::from_str::<ChuongJson>(&file_text.to_string()) {
                let mut dict = Dictionary::new();
                dict.set("ten_file", GString::from(&ten_file));
                dict.set("ten_chuong", data.ten_chuong);
                danh_sach.push(&dict);
            }
        }

        danh_sach
    }
    #[func]
    /// ten_bai_hoc, loai_bai_hoc, duong_dan, nguoi_tao, diem_cao_nhat, ti_le_hoan_thanh, so_lan_choi, hang_tot_nhat
    pub fn lay_json_bai_hoc(&self, ten_chuong: GString) -> Array<Dictionary> {
        let mut danh_sach_bai_hoc: Array<Dictionary> = Array::<Dictionary>::new();
        let duong_dan = format!("res://bai_hoc/danh_sach/{}", ten_chuong);
        godot_print!("Đang đọc file JSON: {}", duong_dan);
        let chuong_json: GString = FileAccess::get_file_as_string(&duong_dan);
        let lich_su: DuLieuLichSu = DuLieuLichSu::doc_lich_su();

        match serde_json::from_str::<ChuongJson>(&chuong_json.to_string()) {
            Ok(du_lieu) => {
                for bai in du_lieu.bai_hoc {
                    let mut dict: Dictionary = Dictionary::new();
                    let nguoi_tao: String = if bai.nguoi_tao.is_empty() {
                        String::from("TimeKP")
                    } else {
                        bai.nguoi_tao
                    };

                    let (diem, ti_le, so_lan, da_xong, hang) =
                        if let Some(kq) = lich_su.lay_ket_qua(&bai.loai_bai_hoc, &bai.duong_dan) {
                            (
                                kq.diem_cao_nhat,
                                kq.ti_le_hoan_thanh,
                                kq.so_lan_choi,
                                kq.da_hoan_thanh,
                                kq.hang_tot_nhat.clone(),
                            )
                        } else {
                            (0, 0.0_f32, 0, false, String::from("-"))
                        };

                    dict.set("ten_bai_hoc", bai.ten_bai_hoc);
                    dict.set("loai_bai_hoc", bai.loai_bai_hoc);
                    dict.set("duong_dan", bai.duong_dan);
                    dict.set("nguoi_tao", nguoi_tao);
                    dict.set("diem_cao_nhat", diem);
                    dict.set("ti_le_hoan_thanh", ti_le);
                    dict.set("so_lan_choi", so_lan);
                    dict.set("da_hoan_thanh", da_xong);
                    dict.set("hang_tot_nhat", GString::from(&hang));
                    danh_sach_bai_hoc.push(&dict);
                }
            }
            Err(e) => godot_print!("Lỗi json bài học: {}", e),
        }
        danh_sach_bai_hoc
    }

    #[func]
    pub fn lay_thong_tin_lich_su(
        &self,
        loai_bai_hoc: GString,
        duong_dan_bai: GString,
    ) -> Dictionary {
        let lich_su: DuLieuLichSu = DuLieuLichSu::doc_lich_su();
        let mut dict: Dictionary = Dictionary::new();
        if let Some(kq) = lich_su.lay_ket_qua(&loai_bai_hoc.to_string(), &duong_dan_bai.to_string())
        {
            dict.set("diem_cao_nhat", kq.diem_cao_nhat);
            dict.set("ti_le_hoan_thanh", kq.ti_le_hoan_thanh);
            dict.set("so_lan_choi", kq.so_lan_choi);
            dict.set("da_hoan_thanh", kq.da_hoan_thanh);
            dict.set("hang_tot_nhat", GString::from(&kq.hang_tot_nhat));
        } else {
            dict.set("diem_cao_nhat", 0);
            dict.set("ti_le_hoan_thanh", 0.0_f32);
            dict.set("so_lan_choi", 0);
            dict.set("da_hoan_thanh", false);
            dict.set("hang_tot_nhat", GString::from("-"));
        }
        dict
    }
}

#[godot_api]
impl IControl for DanhSachBaiHoc {}
