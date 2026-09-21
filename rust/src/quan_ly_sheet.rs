use godot::classes::{ Control, IControl, Label };
use godot::global::godot_print;
use godot::obj::{ Gd, WithBaseField };
use godot::{ obj::Base, prelude::{ GodotClass, godot_api } };

use crate::check_ten_not::chuyen_id_thanh_ten;
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
    #[export]
    toc_do: f32,
    #[export]
    bpm: f32,

    #[export]
    thoi_gian_label: Option<Gd<Label>>,

    khuong_1: Option<Gd<KhuongNhac>>,
    khuong_2: Option<Gd<KhuongNhac>>,
    thoi_gian_cho: f32,
    // sheet_nhac: Json
}

#[godot_api]
impl QuanLySheet {
    // #[func]
    // pub fn nhan_tin_hieu_bam_phim(&mut self, id_not: i32) {
    //     if let Some(khuong) = self.khuong_1.as_mut() {
    //         khuong.bind_mut().xu_ly_bam_phim(id_not);
    //     }
    //     if let Some(khuong) = self.khuong_2.as_mut() {
    //         khuong.bind_mut().xu_ly_bam_phim(id_not);
    //     }
    // }

    // #[func]
    // pub fn nhan_tin_hieu_nha_phim(&mut self, id_not: i32) {
    //     // Truyền lệnh nhả phím xuống cho cả 2 khuông nhạc
    //     if let Some(khuong) = self.khuong_1.as_mut() {
    //         khuong.bind_mut().xu_ly_nha_phim(id_not);
    //     }
    //     if let Some(khuong) = self.khuong_2.as_mut() {
    //         khuong.bind_mut().xu_ly_nha_phim(id_not);
    //     }
    // }

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
        let mut danh_sach_phim_trong_area: Vec<Gd<NotNhac>> = Vec::new();
        if let Some(khuong) = self.khuong_1.as_mut() {
            let ds_phim_khuong_1 = khuong.bind_mut().get_cac_not_trong_area();
            danh_sach_phim_trong_area.extend(ds_phim_khuong_1);
        }
        if let Some(khuong) = self.khuong_2.as_mut() {
            let ds_phim_khuong_2 = khuong.bind_mut().get_cac_not_trong_area();
            danh_sach_phim_trong_area.extend(ds_phim_khuong_2);
        }
        // godot::global::godot_print!(
        //     "Danh sách nốt trong vùng: {:?}",
        //     danh_sach_phim_trong_area
        //         .iter()
        //         .map(|not| not.bind().get_id_not())
        //         .collect::<Vec<i32>>()
        // );
        danh_sach_phim_trong_area
    }
}

#[godot_api]
impl IControl for QuanLySheet {
    fn ready(&mut self) {
        self.khuong_1 = self.base().try_get_node_as::<KhuongNhac>("K1");
        self.khuong_2 = self.base().try_get_node_as::<KhuongNhac>("K2");
        // cai dat
        self.thoi_gian_cho = 10.0;
        self.thoi_gian = 0.0 - self.thoi_gian_cho;
        // ♭
        if let Some(bai_hat) = doc_file_json("res://bai_hoc/choi_sheet/test.json") {
            self.bpm = bai_hat.bpm;
            self.toc_do = bai_hat.toc_do;
            let khoang_cach_nhip: f32 = self.toc_do;

            if let Some(mut khuong) = self.khuong_1.clone() {
                let mut k_bind = khuong.bind_mut();
                for not in bai_hat.khuong_1 {
                    k_bind.tao_not(not.id_not, &not.ten_not, &not.mau_not, not.vi_tri_so, not.nhip_dich, not.nhip_giu, khoang_cach_nhip);
                }
            }

            if let Some(mut khuong) = self.khuong_2.clone() {
                let mut k_bind = khuong.bind_mut();
                for not in bai_hat.khuong_2 {
                    k_bind.tao_not(not.id_not, &not.ten_not, &not.mau_not, not.vi_tri_so, not.nhip_dich, not.nhip_giu, khoang_cach_nhip);
                }
            }
        }
    }
    fn process(&mut self, delta: f64) {
        if self.da_dung {
            return;
        }
        self.thoi_gian += delta as f32;
        // if let Some(label) = self.thoi_gian_label.as_mut() {
        //     label.set_text(&format!("{:.2}", self.thoi_gian).into());
        // }
        self.thoi_gian_label.as_mut().map(|label| {
            label.set_text(&format!("{:.2}", self.thoi_gian + self.thoi_gian_cho));
        });
        let khoang_cach_nhip: f32 = self.toc_do;
        let nhip_hien_tai: f32 = (self.thoi_gian * self.bpm) / 60.0;
        if let Some(khuong) = self.khuong_1.as_mut() {
            khuong.bind_mut().xu_ly_di_chuyen_not(nhip_hien_tai, khoang_cach_nhip);
        }

        if let Some(khuong) = self.khuong_2.as_mut() {
            khuong.bind_mut().xu_ly_di_chuyen_not(nhip_hien_tai, khoang_cach_nhip);
        }
    }
}
