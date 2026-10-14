use std::clone;

use godot::builtin::GString;
use godot::classes::{ AudioStreamPlayer2D, Control, IControl, Label };
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

    // Lưu lại thời điểm kết thúc luyện tập (đã được trừ)
    thoi_gian_ket_thuc_luyen_tap: Option<f32>, 

    da_ket_thuc: bool,
    so_khuong_nhac_het_not: i32,

    am_thanh_nhip: Option<Gd<AudioStreamPlayer2D>>,
    nhip_nguyen_truoc: i32,
    cho_phep_am_thanh_nhip: bool,

    // ===== HAI BIẾN MỚI THÊM VÀO ĐỂ QUẢN LÝ MIỀN NỐT =====

    mien_not_k1: i32,
    mien_not_k2: i32,

    // ===== THÊM 2 BIẾN LABEL ĐỂ HIỂN THỊ LÊN MÀN HÌNH =====
    #[export]
    mien_k1_label: Option<Gd<Label>>,
    #[export]
    mien_k2_label: Option<Gd<Label>>,
}

#[godot_api]
impl QuanLySheet {
    #[signal]
    fn bai_hat_ket_thuc();

    #[func]
    pub fn bat_tat_am_thanh_nhip(&mut self, bat: bool) {
        self.cho_phep_am_thanh_nhip = bat;
        if !bat {
            if let Some(am_thanh) = self.am_thanh_nhip.as_mut() {
                am_thanh.stop();
            }
        }
    }

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
    pub fn khoi_tao_sheet(
        &mut self, 
        duong_dan: GString, 
        khuong_chon: GString, 
        phan_thoi_gian: GString
    ) {
        self.duong_dan_sheet = duong_dan.clone().into();

        if self.duong_dan_sheet.is_empty() {
            godot_print!("Lỗi: Chưa thiết lập 'duong_dan_sheet'!");
            return;
        }
        
        let mut khuong_str = khuong_chon.to_string();
        if khuong_str.is_empty() {
            khuong_str = String::from("12");
        }

        godot_print!("Đang đọc file sheet từ: {}", self.duong_dan_sheet.to_string());
        if let Some(bai_hat) = doc_file_json(&self.duong_dan_sheet.to_string()) {
            self.ten_bai_hat = bai_hat.ten_bai_hat.into();
            self.tac_gia = bai_hat.tac_gia.into();
            self.nguoi_tao_sheet = bai_hat.nguoi_tao_sheet.into();
            self.loai_vach_nhip = bai_hat.loai_vach_nhip.clone().into();
            self.bpm = bai_hat.bpm;
            self.mien_not_k1 = bai_hat.mien_not_bat_dau_k1;
            self.mien_not_k2 = bai_hat.mien_not_bat_dau_k2;
            // godot_print!("mien_not_k1: {}, mien_not_k2: {}", self.mien_not_k1, self.mien_not_k2);
            self.cap_nhat_hien_thi_mien(self.mien_not_k1, self.mien_not_k2);
            let khoang_cach_nhip = self.toc_do;

            let nhip_moi_o = match self.loai_vach_nhip.as_str() {
                "4/4" => 8,
                "3/4" => 6,
                "3/8" => 6,
                "2/4" => 4,
                "6/8" => 12,
                _ => 8,
            };
            
            // ================= TÍNH TOÁN DỊCH NHỊP LUYỆN TẬP =================
            let mut nhip_bat_dau: f32 = 0.0;
            let mut nhip_ket_thuc: f32 = f32::MAX;
            self.thoi_gian_ket_thuc_luyen_tap = None;
            
            let chuoi_tg: String = phan_thoi_gian.to_string();
            let dang_luyen_tap: bool = !chuoi_tg.is_empty();

            if dang_luyen_tap {
                let cac_phan: Vec<&str> = chuoi_tg.split('-').collect();
                if cac_phan.len() == 2 {
                    if let Ok(bat_dau) = cac_phan[0].trim().parse::<f32>() {
                        nhip_bat_dau = bat_dau;
                    }
                    if let Ok(ket_thuc) = cac_phan[1].trim().parse::<f32>() {
                        nhip_ket_thuc = ket_thuc;
                        self.thoi_gian_ket_thuc_luyen_tap = Some(ket_thuc - nhip_bat_dau);
                    }
                }
            }

            let loc_va_dich_not = |nhip_goc: f32| -> Option<f32> {
                if !dang_luyen_tap {
                    return Some(nhip_goc); // Không luyện tập, giữ nguyên
                }
                
                if nhip_goc >= nhip_bat_dau && nhip_goc <= nhip_ket_thuc {
                    Some(nhip_goc - nhip_bat_dau) 
                } else {
                    None
                }
            };
            // =================================================================
            if khuong_str.contains('1') {
                if let Some(mut khuong) = self.khuong_1.clone() {
                    let mut k_bind = khuong.bind_mut();
                    k_bind.nhip_moi_o = nhip_moi_o;
                    
                    // Khởi tạo miền theo dõi cho khuông 1
                    let mut mien_hien_tai_k1 = self.mien_not_k1;

                    for not in bai_hat.khuong_1 {
                        if let Some(nhip_dich_moi) = loc_va_dich_not(not.nhip_dich) {
                            k_bind.tao_not(
                                not.id_not, 
                                not.mau_not.clone(), 
                                nhip_dich_moi, 
                                not.nhip_giu, 
                                not.chuyen_not, 
                                mien_hien_tai_k1, // Gửi miền hiện tại để tính tọa độ
                                khoang_cach_nhip
                            );
                            
                            // Nếu đây là nốt chuyển, cộng dồn vào miền hiện tại cho nốt sau
                            if not.id_not == 100 {
                                mien_hien_tai_k1 += not.chuyen_not;
                            }
                        }
                    }
                }
            } else if let Some(mut khuong) = self.khuong_1.clone() {
                khuong.bind_mut().het_not_trong_khuong();
            }

            if khuong_str.contains('2') {
                if let Some(mut khuong) = self.khuong_2.clone() {
                    let mut k_bind = khuong.bind_mut();
                    k_bind.nhip_moi_o = nhip_moi_o;
                    if bai_hat.khuong_2.is_empty() {
                        k_bind.het_not_trong_khuong();
                    } else {
                        // Khởi tạo miền theo dõi cho khuông 2
                        let mut mien_hien_tai_k2 = self.mien_not_k2;

                        for not in bai_hat.khuong_2 {
                            if let Some(nhip_dich_moi) = loc_va_dich_not(not.nhip_dich) {
                                k_bind.tao_not(
                                    not.id_not, 
                                    not.mau_not.clone(), 
                                    nhip_dich_moi, 
                                    not.nhip_giu, 
                                    not.chuyen_not, 
                                    mien_hien_tai_k2, // Gửi miền hiện tại của K2
                                    khoang_cach_nhip
                                );
                                
                                // Cập nhật miền nếu có nốt chuyển ở K2
                                if not.id_not == 100 {
                                    mien_hien_tai_k2 += not.chuyen_not;
                                }
                            }
                        }
                    }
                }
            } else if let Some(mut khuong) = self.khuong_2.clone() {
                khuong.bind_mut().het_not_trong_khuong();
            }

            let nhip_khoi_diem: f32 = (self.thoi_gian * self.bpm) / 60.0;
            self.nhip_nguyen_truoc = nhip_khoi_diem.floor() as i32;
        } else {
            godot_print!("Lỗi: Không thể phân tích file JSON tại {}", duong_dan.to_string());
        }
    }
    // Hàm nhận tín hiệu từ Khuông 1
    #[func]
    pub fn thay_doi_mien_k1(&mut self, gia_tri_chuyen: i32) {
        self.mien_not_k1 += gia_tri_chuyen;
        self.cap_nhat_hien_thi_mien(self.mien_not_k1, self.mien_not_k2);
    }

    // Hàm nhận tín hiệu từ Khuông 2
    #[func]
    pub fn thay_doi_mien_k2(&mut self, gia_tri_chuyen: i32) {
        self.mien_not_k2 += gia_tri_chuyen;
        self.cap_nhat_hien_thi_mien(self.mien_not_k1, self.mien_not_k2);
    }
    pub fn chuyen_doi_ten_mien(so: i32) -> String {
        let dau = chuyen_id_thanh_ten(so, false);
        let cuoi = chuyen_id_thanh_ten(so + 17, false);
        format!("{} - {}", dau, cuoi)
    }
    pub fn cap_nhat_hien_thi_mien(&mut self, mien_1: i32, mien_2: i32) {
        if let Some(mut label) = self.mien_k1_label.clone() {
            label.set_text(&Self::chuyen_doi_ten_mien(mien_1));
        }
        if let Some(mut label) = self.mien_k2_label.clone() {
            label.set_text(&Self::chuyen_doi_ten_mien(mien_2));
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
        let bpm_hien_tai: f32 = if self.bpm > 0.0 { self.bpm } else { 80.0 };
        let nhip_khoi_diem: f32 = (self.thoi_gian * bpm_hien_tai) / 60.0;
        self.nhip_nguyen_truoc = nhip_khoi_diem.floor() as i32;

        self.am_thanh_nhip = self.base().try_get_node_as::<AudioStreamPlayer2D>("nhip_trong");
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
        let nhip_hien_tai: f32 = (self.thoi_gian * self.bpm) / 60.0;
        
        // ================= XỬ LÝ PHÁT ÂM THANH NHỊP =================
        let nhip_nguyen: i32 = nhip_hien_tai.floor() as i32;
        
        if nhip_nguyen > self.nhip_nguyen_truoc {
            // CHỈ PHÁT ÂM THANH KHI BIẾN NÀY LÀ TRUE
            if self.cho_phep_am_thanh_nhip {
                if let Some(am_thanh) = self.am_thanh_nhip.as_mut() {
                    let nhip_moi_o = match self.loai_vach_nhip.as_str() {
                        "4/4" => 8,
                        "3/4" => 6,
                        "3/8" => 6,
                        "2/4" => 4,
                        "6/8" => 12,
                        _ => 8, // Mặc định
                    };
                    // Xử lý bù trừ số âm: rem_euclid giúp % số âm vẫn ra phách mạnh chuẩn xác.
                    if nhip_nguyen.rem_euclid(nhip_moi_o) == 0 {
                        am_thanh.set_pitch_scale(1.5);
                    } else {
                        am_thanh.set_pitch_scale(1.0);
                    }
                    
                    am_thanh.play();
                }
            }
            self.nhip_nguyen_truoc = nhip_nguyen;
        }
        
        // KIỂM TRA HẾT THỜI GIAN LUYỆN TẬP
        if let Some(ket_thuc_da_dich) = self.thoi_gian_ket_thuc_luyen_tap {
            if nhip_hien_tai > ket_thuc_da_dich + 1.5 { 
                self.da_ket_thuc = true;
                self.base_mut().set_process(false);
                self.base_mut().emit_signal("bai_hat_ket_thuc", &[]);
                return;
            }
        }

        // Khôi phục lại giao diện hiển thị 10 giây chờ
        self.thoi_gian_label.as_mut().map(|label| {
            label.set_text(&format!("{:.2}", self.thoi_gian + self.thoi_gian_cho));
        });

        let khoang_cach_nhip = self.toc_do;

        if let Some(khuong) = self.khuong_1.as_mut() {
            khuong.bind_mut().xu_ly_di_chuyen_not(nhip_hien_tai, khoang_cach_nhip);
        }
        if let Some(khuong) = self.khuong_2.as_mut() {
            khuong.bind_mut().xu_ly_di_chuyen_not(nhip_hien_tai, khoang_cach_nhip);
        }
    }
}