use godot::builtin::GString;
use godot::classes::{ Control, IControl, Label };
use godot::global::godot_print;
use godot::obj::{ Gd, WithBaseField };
use godot::{ obj::Base, prelude::{ GodotClass, godot_api } };

use crate::doc_sheet_json::doc_file_json;
use crate::khuong_nhac::KhuongNhac;
use crate::not_nhac::NotNhac;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct QuanLySheet {
    #[base]
    base: Base<Control>,
    thoi_gian: f32,
    #[export]
    da_dung: bool,

    ten_bai_hat: String,
    tac_gia: String,
    nguoi_tao_sheet: String,
    loai_vach_nhip: String,
    #[export]
    duong_dan_sheet: GString,

    #[export]
    toc_do: f32,
    bpm: f32,
    #[export]
    cho_phep_cho: bool,

    #[export]
    thoi_gian_label: Option<Gd<Label>>,

    khuong_1: Option<Gd<KhuongNhac>>,
    khuong_2: Option<Gd<KhuongNhac>>,
    thoi_gian_cho: f32,

    da_ket_thuc: bool,
    so_khuong_nhac_het_not: i32,
}

#[godot_api]
impl QuanLySheet {
    #[signal]
    fn bai_hat_ket_thuc();

    #[func]
    pub fn get_tong_so_not_sheet(&self) -> i32 {
        let mut tong_so_not = 0;
        if let Some(khuong) = self.khuong_1.as_ref() {
            tong_so_not += khuong.bind().get_tong_so_not();
        }
        if let Some(khuong) = self.khuong_2.as_ref() {
            tong_so_not += khuong.bind().get_tong_so_not();
        }
        tong_so_not
    }

    #[func]
    pub fn get_ten_bai_hat(&self) -> GString {
        self.ten_bai_hat.clone().into()
    }

    #[func]
    pub fn get_tac_gia(&self) -> GString {
        self.tac_gia.clone().into()
    }

    #[func]
    pub fn get_nguoi_tao_sheet(&self) -> GString {
        self.nguoi_tao_sheet.clone().into()
    }

    #[func]
    pub fn get_tat_ca_not_tren_sheet(&mut self) -> Vec<Gd<NotNhac>> {
        let mut ds_not = Vec::new();
        if let Some(khuong) = self.khuong_1.as_mut() {
            ds_not.extend(khuong.bind_mut().get_tat_ca_not());
        }
        if let Some(khuong) = self.khuong_2.as_mut() {
            ds_not.extend(khuong.bind_mut().get_tat_ca_not());
        }
        ds_not
    }

    #[func]
    pub fn get_thoi_gian(&self) -> f32 {
        self.thoi_gian
    }

    #[func]
    pub fn get_danh_sach_not_trong_area(&mut self) -> Vec<Gd<NotNhac>> {
        let mut danh_sach_phim_trong_area = Vec::new();
        if let Some(khuong) = self.khuong_1.as_mut() {
            danh_sach_phim_trong_area.extend(khuong.bind_mut().get_cac_not_trong_area());
        }
        if let Some(khuong) = self.khuong_2.as_mut() {
            danh_sach_phim_trong_area.extend(khuong.bind_mut().get_cac_not_trong_area());
        }
        danh_sach_phim_trong_area
    }

    #[func]
    pub fn kiem_tra_ket_thuc(&mut self) {
        self.so_khuong_nhac_het_not += 1;
        if self.so_khuong_nhac_het_not >= 2 && !self.da_ket_thuc {
            self.da_ket_thuc = true;
            self.base_mut().set_process(false);
            self.base_mut().emit_signal("bai_hat_ket_thuc", &[]);
            self.so_khuong_nhac_het_not = 0;
        }
    }

    #[func]
    pub fn khoi_tao_sheet(&mut self, duong_dan: GString) {
        self.duong_dan_sheet = duong_dan.clone().into();

        if self.duong_dan_sheet.is_empty() {
            godot_print!("Lỗi: Chưa thiết lập 'duong_dan_sheet'!");
            return;
        }
        godot_print!("Đang đọc file sheet từ: {}", self.duong_dan_sheet.to_string());
        if let Some(bai_hat) = doc_file_json(&self.duong_dan_sheet.to_string()) {
            self.ten_bai_hat = bai_hat.ten_bai_hat.into();
            self.tac_gia = bai_hat.tac_gia.into();
            self.nguoi_tao_sheet = bai_hat.nguoi_tao_sheet.into();
            self.loai_vach_nhip = bai_hat.loai_vach_nhip.clone().into();
            self.bpm = bai_hat.bpm;
            let khoang_cach_nhip = self.toc_do;

            let nhip_moi_o = match self.loai_vach_nhip.as_str() {
                "4/4" => 8,
                "3/4" => 6,
                "3/8" => 6,
                "2/4" => 4,
                "6/8" => 12,
                _ => 8,
            };

            if let Some(mut khuong) = self.khuong_1.clone() {
                let mut k_bind = khuong.bind_mut();
                k_bind.nhip_moi_o = nhip_moi_o;
                for not in bai_hat.khuong_1 {
                    k_bind.tao_not(
                        not.id_not,
                        &not.ten_not,
                        &not.mau_not,
                        not.vi_tri_so,
                        not.nhip_dich,
                        not.nhip_giu,
                        khoang_cach_nhip
                    );
                }
            }

            if let Some(mut khuong) = self.khuong_2.clone() {
                let mut k_bind = khuong.bind_mut();
                k_bind.nhip_moi_o = nhip_moi_o;
                if bai_hat.khuong_2.is_empty() {
                    k_bind.het_not_trong_khuong();
                } else {
                    for not in bai_hat.khuong_2 {
                        k_bind.tao_not(
                            not.id_not,
                            &not.ten_not,
                            &not.mau_not,
                            not.vi_tri_so,
                            not.nhip_dich,
                            not.nhip_giu,
                            khoang_cach_nhip
                        );
                    }
                }
            }
        } else {
            godot_print!("Lỗi: Không thể phân tích file JSON tại {}", duong_dan.to_string());
        }
    }
}

#[godot_api]
impl IControl for QuanLySheet {
    fn ready(&mut self) {
        self.so_khuong_nhac_het_not = 0;
        self.khuong_1 = self.base().try_get_node_as::<KhuongNhac>("K1");
        self.khuong_2 = self.base().try_get_node_as::<KhuongNhac>("K2");
        self.thoi_gian_cho = 10.0;
        self.thoi_gian = 0.0 - self.thoi_gian_cho;
    }

    fn process(&mut self, delta: f64) {
        if self.cho_phep_cho {
            let mut phai_cho = false;
            let ds_not = self.get_danh_sach_not_trong_area();

            for not_node in ds_not {
                let not_bind = not_node.bind();
                if !not_bind.get_da_duoc_danh() && not_node.get_position().x <= 117.0 {
                    phai_cho = true;
                    break;
                }
            }
            self.da_dung = phai_cho;
        }

        if self.da_dung {
            return;
        }

        self.thoi_gian += delta as f32;

        self.thoi_gian_label.as_mut().map(|label| {
            label.set_text(&format!("{:.2}", self.thoi_gian + self.thoi_gian_cho));
        });

        let khoang_cach_nhip = self.toc_do;
        let nhip_hien_tai = (self.thoi_gian * self.bpm) / 60.0;

        if let Some(khuong) = self.khuong_1.as_mut() {
            khuong.bind_mut().xu_ly_di_chuyen_not(nhip_hien_tai, khoang_cach_nhip);
        }
        if let Some(khuong) = self.khuong_2.as_mut() {
            khuong.bind_mut().xu_ly_di_chuyen_not(nhip_hien_tai, khoang_cach_nhip);
        }
    }
}
