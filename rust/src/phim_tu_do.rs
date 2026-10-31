use godot::prelude::*;
use godot::classes::{Control, FileAccess};
use godot::classes::file_access::ModeFlags;
use godot::builtin::{Dictionary, GString};
use serde::{Serialize, Deserialize};
use crate::check_ten_not::chuyen_id_thanh_ten;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuKienMidi {
    pub thoi_gian: f32,
    pub so_phim: i32,
    pub la_bam: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BanGhiJson {
    pub cac_su_kien: Vec<SuKienMidi>,
}

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct PhimTuDo {
    #[base]
    base: Base<Control>,
    
    dang_ghi_am: bool,
    dang_phat_lai: bool,
    thoi_gian_hien_tai: f32,
    ban_ghi: Vec<SuKienMidi>,
    chi_so_phat: usize,
}

#[godot_api]
impl PhimTuDo {
    #[signal]
    fn hieu_lenh_phat_phim(so_phim: i32);
    #[signal]
    fn hieu_lenh_nha_phim(so_phim: i32);
    #[signal]
    fn ket_thuc_phat_lai();

    #[func]
    pub fn get_thong_tin_phim(&self, id: i32) -> Dictionary {
        let quang_8 = id / 12;
        let du = id % 12;
        
        let mut hang_trang = 0;
        let mut la_phim_den = false;
        let mut offset_den = 0.0;
        
        match du {
            0 => hang_trang = 0,
            1 => { hang_trang = 0; la_phim_den = true; offset_den = 0.65; },
            2 => hang_trang = 1,
            3 => { hang_trang = 1; la_phim_den = true; offset_den = 0.75; },
            4 => hang_trang = 2,
            5 => hang_trang = 3,
            6 => { hang_trang = 3; la_phim_den = true; offset_den = 0.6; },
            7 => hang_trang = 4,
            8 => { hang_trang = 4; la_phim_den = true; offset_den = 0.7; },
            9 => hang_trang = 5,
            10 => { hang_trang = 5; la_phim_den = true; offset_den = 0.8; },
            11 => hang_trang = 6,
            _ => {}
        }
        
        let tong_hang_trang = (quang_8 * 7 + hang_trang) as f32;
        let rong_phim_trang = 1920.0 / 49.0;
        
        let mut x_pos = tong_hang_trang * rong_phim_trang;
        let mut do_rong = rong_phim_trang - 4.0;
        let mut mau = Color::from_rgba(0.2, 0.75, 1.0, 0.85); 
        
        if la_phim_den {
            x_pos += rong_phim_trang * offset_den;
            do_rong = rong_phim_trang * 0.6;
            mau = Color::from_rgba(1.0, 0.35, 0.5, 0.85); 
        }
        
        let mut dict = Dictionary::new();
        let _ = dict.insert("toa_do_x", x_pos + 2.0);
        let _ = dict.insert("chieu_rong", do_rong);
        let _ = dict.insert("mau_sac", mau);
        let _ = dict.insert("ten_not", chuyen_id_thanh_ten(id, false));
        
        dict
    }

    #[func]
    pub fn ghi_nhan_phim_bam(&mut self, so_phim: i32) {
        if self.dang_ghi_am {
            self.ban_ghi.push(SuKienMidi {
                thoi_gian: self.thoi_gian_hien_tai,
                so_phim,
                la_bam: true,
            });
        }
    }

    #[func]
    pub fn ghi_nhan_phim_nha(&mut self, so_phim: i32) {
        if self.dang_ghi_am {
            self.ban_ghi.push(SuKienMidi {
                thoi_gian: self.thoi_gian_hien_tai,
                so_phim,
                la_bam: false,
            });
        }
    }

    #[func]
    pub fn bat_dau_ghi(&mut self) {
        self.dang_ghi_am = true;
        self.dang_phat_lai = false;
        self.thoi_gian_hien_tai = 0.0;
        self.ban_ghi.clear();
    }

    #[func]
    pub fn dung_hoat_dong(&mut self) {
        self.dang_ghi_am = false;
        self.dang_phat_lai = false;
    }

    #[func]
    pub fn phat_lai(&mut self) {
        if self.ban_ghi.is_empty() {
            self.base_mut().emit_signal("ket_thuc_phat_lai", &[]);
            return;
        }
        self.dang_ghi_am = false;
        self.dang_phat_lai = true;
        self.thoi_gian_hien_tai = 0.0;
        self.chi_so_phat = 0;
    }
    
    #[func]
    pub fn lay_trang_thai(&self) -> i32 {
        if self.dang_ghi_am { return 1; }
        if self.dang_phat_lai { return 2; }
        0
    }

    #[func]
    pub fn luu_ban_ghi(&self, duong_dan: GString) -> bool {
        let du_lieu = BanGhiJson {
            cac_su_kien: self.ban_ghi.clone(),
        };
        if let Ok(json_str) = serde_json::to_string_pretty(&du_lieu) {
            if let Some(mut file) = FileAccess::open(&duong_dan.to_string(), ModeFlags::WRITE) {
                file.store_string(&json_str);
                return true;
            }
        }
        false
    }

    #[func]
    pub fn mo_ban_ghi(&mut self, duong_dan: GString) -> bool {
        let file_text = FileAccess::get_file_as_string(&duong_dan.to_string());
        if let Ok(du_lieu) = serde_json::from_str::<BanGhiJson>(&file_text.to_string()) {
            self.ban_ghi = du_lieu.cac_su_kien;
            self.dang_ghi_am = false;
            self.dang_phat_lai = false;
            return true;
        }
        false
    }

    #[func]
    pub fn xu_ly_thoi_gian(&mut self, dt: f32) {
        if self.dang_ghi_am || self.dang_phat_lai {
            self.thoi_gian_hien_tai += dt;
        }

        if self.dang_phat_lai {
            while self.chi_so_phat < self.ban_ghi.len() {
                let su_kien = &self.ban_ghi[self.chi_so_phat];
                if su_kien.thoi_gian <= self.thoi_gian_hien_tai {
                    if su_kien.la_bam {
                        let so_phim = su_kien.so_phim;
                        self.base_mut().emit_signal("hieu_lenh_phat_phim", &[so_phim.to_variant()]);
                    } else {
                        let so_phim = su_kien.so_phim;
                        self.base_mut().emit_signal("hieu_lenh_nha_phim", &[so_phim.to_variant()]);
                    }
                    self.chi_so_phat += 1;
                } else {
                    break; 
                }
            }
            
            if self.chi_so_phat >= self.ban_ghi.len() {
                let sk_cuoi = self.ban_ghi.last().unwrap();
                if self.thoi_gian_hien_tai > sk_cuoi.thoi_gian + 1.5 {
                    self.dang_phat_lai = false;
                    self.base_mut().emit_signal("ket_thuc_phat_lai", &[]);
                }
            }
        }
    }
}