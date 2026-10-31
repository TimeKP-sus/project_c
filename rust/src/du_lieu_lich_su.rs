use godot::builtin::{Dictionary, GString};
use godot::classes::{file_access::ModeFlags, DirAccess, FileAccess, Object};
use godot::global::godot_print;
use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const DUONG_DAN_LICH_SU: &str = "res://du_lieu_nguoi_dung/lich_su.json";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KetQuaBaiHocJson {
    pub loai_che_do: String,
    pub duong_dan_bai: String,
    pub diem_cao_nhat: i32,
    pub ti_le_hoan_thanh: f32,
    pub so_lan_choi: i32,
    pub da_hoan_thanh: bool,
    #[serde(default)]
    pub hang_tot_nhat: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DuLieuLichSu {
    #[serde(default)]
    pub danh_sach_ket_qua: HashMap<String, KetQuaBaiHocJson>,
}

impl DuLieuLichSu {
    /// Chuẩn hóa tên chế độ (gộp "tap_luyen_sheet" và "luyen_tap_sheet" làm một)
    pub fn chuan_hoa_che_do(loai_che_do: &str) -> String {
        match loai_che_do.trim() {
            "tap_luyen_sheet" | "luyen_tap_sheet" | "luyen_tap" => String::from("luyen_tap_sheet"),
            other => other.to_string(),
        }
    }

    /// Tạo khóa duy nhất kết hợp giữa chế độ chơi và đường dẫn bài học
    pub fn tao_khoa(loai_che_do: &str, duong_dan_bai: &str) -> String {
        let che_do = Self::chuan_hoa_che_do(loai_che_do);
        let path = duong_dan_bai.trim().replace('\\', "/");
        format!("{}|{}", che_do, path)
    }

    pub fn doc_lich_su() -> Self {
        let txt = FileAccess::get_file_as_string(DUONG_DAN_LICH_SU).to_string();
        if !txt.trim().is_empty() {
            if let Ok(data) = serde_json::from_str::<DuLieuLichSu>(&txt) {
                return data;
            }
        }
        Self::default()
    }

    pub fn luu_lich_su(&self) -> bool {
        let _ = DirAccess::make_dir_recursive_absolute("res://du_lieu_nguoi_dung");
        if let Ok(noi_dung) = serde_json::to_string_pretty(self) {
            if let Some(mut file) = FileAccess::open(DUONG_DAN_LICH_SU, ModeFlags::WRITE) {
                file.store_string(&noi_dung);
                return true;
            }
        }
        godot_print!("Lỗi: Không thể ghi file lịch sử tại {}", DUONG_DAN_LICH_SU);
        false
    }

    pub fn cap_nhat_ket_qua(
        &mut self,
        loai_che_do: &str,
        duong_dan_bai: &str,
        diem_moi: i32,
        ti_le_moi: f32,
        hang_moi: &str,
    ) -> KetQuaBaiHocJson {
        let che_do_chuan = Self::chuan_hoa_che_do(loai_che_do);
        let path_chuan = duong_dan_bai.trim().replace('\\', "/");
        let khoa = Self::tao_khoa(&che_do_chuan, &path_chuan);
        let ti_le_chuan = ti_le_moi.clamp(0.0, 100.0);

        let muc = self
            .danh_sach_ket_qua
            .entry(khoa)
            .or_insert_with(|| KetQuaBaiHocJson {
                loai_che_do: che_do_chuan.clone(),
                duong_dan_bai: path_chuan.clone(),
                ..Default::default()
            });

        muc.so_lan_choi += 1;
        if diem_moi > muc.diem_cao_nhat {
            muc.diem_cao_nhat = diem_moi;
        }
        if ti_le_chuan >= muc.ti_le_hoan_thanh {
            muc.ti_le_hoan_thanh = ti_le_chuan;
            if !hang_moi.is_empty() {
                muc.hang_tot_nhat = hang_moi.to_string();
            }
        } else if muc.hang_tot_nhat.is_empty() && !hang_moi.is_empty() {
            muc.hang_tot_nhat = hang_moi.to_string();
        }

        if ti_le_chuan >= 50.0 || diem_moi > 0 {
            muc.da_hoan_thanh = true;
        }

        let kq = muc.clone();
        self.luu_lich_su();
        kq
    }

    pub fn lay_ket_qua(&self, loai_che_do: &str, duong_dan_bai: &str) -> Option<&KetQuaBaiHocJson> {
        let khoa: String = Self::tao_khoa(loai_che_do, duong_dan_bai);
        self.danh_sach_ket_qua.get(&khoa)
    }

    pub fn tinh_hang(ti_le: f32) -> &'static str {
        if ti_le >= 95.0 {
            "S"
        } else if ti_le >= 85.0 {
            "A"
        } else if ti_le >= 70.0 {
            "B"
        } else if ti_le >= 50.0 {
            "C"
        } else {
            "D"
        }
    }
}

#[derive(GodotClass)]
#[class(init, base = Object)]
pub struct QuanLyLichSu {
    #[base]
    base: Base<Object>,
}

#[godot_api]
impl QuanLyLichSu {
    #[func]
    pub fn luu_ket_qua_bai_hoc(
        &self,
        loai_che_do: GString,
        duong_dan_bai: GString,
        diem: i32,
        ti_le_hoan_thanh: f32,
        hang: GString,
    ) -> Dictionary {
        let mut ls = DuLieuLichSu::doc_lich_su();
        let kq = ls.cap_nhat_ket_qua(
            &loai_che_do.to_string(),
            &duong_dan_bai.to_string(),
            diem,
            ti_le_hoan_thanh,
            &hang.to_string(),
        );

        let mut dict = Dictionary::new();
        dict.set("diem_cao_nhat", kq.diem_cao_nhat);
        dict.set("ti_le_hoan_thanh", kq.ti_le_hoan_thanh);
        dict.set("so_lan_choi", kq.so_lan_choi);
        dict.set("da_hoan_thanh", kq.da_hoan_thanh);
        dict.set("hang_tot_nhat", GString::from(&kq.hang_tot_nhat));
        dict
    }

    #[func]
    pub fn lay_ket_qua_bai_hoc(&self, loai_che_do: GString, duong_dan_bai: GString) -> Dictionary {
        let ls: DuLieuLichSu = DuLieuLichSu::doc_lich_su();
        let mut dict: Dictionary = Dictionary::new();
        if let Some(kq) = ls.lay_ket_qua(&loai_che_do.to_string(), &duong_dan_bai.to_string()) {
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