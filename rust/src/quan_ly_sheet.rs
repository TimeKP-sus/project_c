use godot::builtin::GString;
use godot::classes::{ AudioStreamPlayer2D, Control, IControl, Label };
use godot::global::godot_print;
use godot::obj::{ Gd, WithBaseField };
use godot::{ obj::Base, prelude::{ GodotClass, godot_api } };
use crate::check_ten_not::chuyen_id_thanh_ten;
use crate::doc_sheet_json::{ doc_file_json, tien_xu_ly_khuong };
use crate::du_lieu_nguoi_dung::CaiDatGameJson;
use crate::khuong_nhac::KhuongNhac;
use crate::not_nhac::NotNhac;

const TOC_DO_GOC: f32 = 100.0;

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
    do_tre_phim: f32,
    #[export]
    toc_do: f32,
    pub bpm: f32,
    #[export]
    cho_phep_cho: bool,
    #[export]
    hien_thi_ten_khac: bool,
    dang_luyen_tap_mode: bool,

    #[export]
    thoi_gian_label: Option<Gd<Label>>,
    #[export]
    hien_thi_thoi_gian: bool,

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

    mien_not_k1: i32,
    mien_not_k2: i32,

    #[export]
    mien_k1_label: Option<Gd<Label>>,
    #[export]
    mien_k2_label: Option<Gd<Label>>,
    #[export]
    nhip_k1_label: Option<Gd<Label>>,
    #[export]
    nhip_k2_label: Option<Gd<Label>>,
}

#[godot_api]
impl QuanLySheet {
    #[signal]
    fn bai_hat_ket_thuc();

    #[func]
    pub fn bat_tat_cho_phep_xoa_not(&mut self, bat: bool) {
        if let Some(khuong) = self.khuong_1.as_mut() {
            khuong.bind_mut().set_cho_phep_xoa_not(bat);
        }
        if let Some(khuong) = self.khuong_2.as_mut() {
            khuong.bind_mut().set_cho_phep_xoa_not(bat);
        }
    }
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
    pub fn set_che_do_luyen_tap(&mut self, la_luyen_tap: bool) {
        self.dang_luyen_tap_mode = la_luyen_tap;
        if !la_luyen_tap {
            self.bat_tat_hien_thi_ten_khac(false);
        }
    }

    #[func]
    pub fn bat_tat_hien_thi_ten_khac(&mut self, bat: bool) {
        // Chỉ cho phép bật khi đang ở chế độ tập luyện
        let cho_phep = bat && self.dang_luyen_tap_mode;
        self.hien_thi_ten_khac = cho_phep;

        if let Some(khuong) = self.khuong_1.as_mut() {
            khuong.bind_mut().dat_hien_thi_ten_khac(cho_phep);
        }
        if let Some(khuong) = self.khuong_2.as_mut() {
            khuong.bind_mut().dat_hien_thi_ten_khac(cho_phep);
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
        let mut ds_not: Vec<Gd<NotNhac>> = Vec::new();
        if let Some(khuong) = self.khuong_1.as_mut() {
            ds_not.extend(khuong.bind_mut().get_tat_ca_not());
        }
        if let Some(khuong) = self.khuong_2.as_mut() {
            ds_not.extend(khuong.bind_mut().get_tat_ca_not());
        }
        ds_not
    }
    // pub fn get_toc_do(&self) -> f32 {
    //     self.toc_do
    // }

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
    pub fn cat_nhat_nhip_label(&mut self, nhip_k: String) {
        let nhip_k: &str = match nhip_k.as_str() {
            "4/4" => "4\n4",
            "3/4" => "3\n4",
            "3/8" => "3\n8",
            "2/4" => "2\n4",
            "6/8" => "6\n8",
            _ => "4\n4",
        };
        if let Some(label) = self.nhip_k1_label.as_mut() {
            label.set_text(&format!("{}", nhip_k));
        }
        if let Some(label) = self.nhip_k2_label.as_mut() {
            label.set_text(&format!("{}", nhip_k));
        }
    }
    #[func]
    pub fn set_do_tre_phim_ms(&mut self, do_tre_ms: f32) {
        self.do_tre_phim = do_tre_ms / 1000.0;
    }
    #[func]
    pub fn khoi_tao_sheet(
        &mut self,
        duong_dan: GString,
        khuong_chon: GString,
        phan_thoi_gian: GString
    ) {
        self.duong_dan_sheet = duong_dan.clone().into();
        let cai_dat: CaiDatGameJson = CaiDatGameJson::doc_cai_dat();
        self.toc_do = TOC_DO_GOC * cai_dat.toc_do_choi;
        self.do_tre_phim = cai_dat.do_tre_phim_ms / 1000.0; // Đổi ms sang giây
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
            self.cat_nhat_nhip_label(self.loai_vach_nhip.clone().as_str().into());
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

            let mut nhip_bat_dau: f32 = 0.0;
            let mut nhip_ket_thuc: f32 = f32::MAX;
            self.thoi_gian_ket_thuc_luyen_tap = None;

            let chuoi_tg: String = phan_thoi_gian.to_string();
            let dang_luyen_tap = !chuoi_tg.is_empty() || self.cho_phep_cho || self.dang_luyen_tap_mode;
            self.dang_luyen_tap_mode = dang_luyen_tap;
            let cho_hien_ten_khac: bool = self.hien_thi_ten_khac && self.dang_luyen_tap_mode;

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

            let (mien_k1_moi, ds_not_k1) = tien_xu_ly_khuong(
                bai_hat.khuong_1,
                bai_hat.mien_not_bat_dau_k1,
                dang_luyen_tap,
                nhip_bat_dau,
                nhip_ket_thuc
            );
            let (mien_k2_moi, ds_not_k2) = tien_xu_ly_khuong(
                bai_hat.khuong_2,
                bai_hat.mien_not_bat_dau_k2,
                dang_luyen_tap,
                nhip_bat_dau,
                nhip_ket_thuc
            );

            self.mien_not_k1 = mien_k1_moi;
            self.mien_not_k2 = mien_k2_moi;
            self.cap_nhat_hien_thi_mien(self.mien_not_k1, self.mien_not_k2);

            // Sinh nốt cho Khuông 1
            if khuong_str.contains('1') {
                if let Some(mut khuong) = self.khuong_1.clone() {
                    let mut k_bind = khuong.bind_mut();
                    k_bind.nhip_moi_o = nhip_moi_o;
                    if ds_not_k1.is_empty() {
                        k_bind.het_not_trong_khuong();
                    } else {
                        let mut mien_hien_tai_k1 = self.mien_not_k1;
                        for i in 0..ds_not_k1.len() {
                            let not = &ds_not_k1[i];

                            // Gọi hàm mới: Vừa áp dụng Dấu chỉnh tay, vừa Tự động đoán nếu để trống
                            let dung_giang = crate::doc_sheet_json::xac_dinh_giang_tai_vi_tri(
                                &ds_not_k1,
                                i
                            );

                            k_bind.tao_not(
                                not.id_not,
                                not.mau_not.clone(),
                                not.nhip_dich,
                                not.nhip_giu,
                                not.chuyen_not,
                                mien_hien_tai_k1,
                                khoang_cach_nhip,
                                dung_giang,
                                not.ten_khac.clone(),
                                cho_hien_ten_khac,
                            );

                            if not.id_not == 100 {
                                mien_hien_tai_k1 += not.chuyen_not;
                            }
                        }
                    }
                }
            } else if let Some(mut khuong) = self.khuong_1.clone() {
                khuong.bind_mut().het_not_trong_khuong();
            }

            // Sinh nốt cho Khuông 2
            if khuong_str.contains('2') {
                if let Some(mut khuong) = self.khuong_2.clone() {
                    let mut k_bind = khuong.bind_mut();
                    k_bind.nhip_moi_o = nhip_moi_o;
                    if ds_not_k2.is_empty() {
                        k_bind.het_not_trong_khuong();
                    } else {
                        let mut mien_hien_tai_k2: i32 = self.mien_not_k2;
                        for i in 0..ds_not_k2.len() {
                            let not = &ds_not_k2[i];

                            // Gọi hàm mới cho khuông 2
                            let dung_giang = crate::doc_sheet_json::xac_dinh_giang_tai_vi_tri(
                                &ds_not_k2,
                                i
                            );

                            k_bind.tao_not(
                                not.id_not,
                                not.mau_not.clone(),
                                not.nhip_dich,
                                not.nhip_giu,
                                not.chuyen_not,
                                mien_hien_tai_k2,
                                khoang_cach_nhip,
                                dung_giang,
                                not.ten_khac.clone(),
                                cho_hien_ten_khac,
                            );

                            if not.id_not == 100 {
                                mien_hien_tai_k2 += not.chuyen_not;
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
        let dau: &str = chuyen_id_thanh_ten(so, false);
        let hang_dau: i32 = crate::check_ten_not::id_sang_hang_phim_trang(so, false);
        let id_cuoi: i32 = crate::check_ten_not::hang_phim_trang_sang_id(hang_dau + 16);
        let cuoi: &str = chuyen_id_thanh_ten(id_cuoi, false);
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
    #[func]
    pub fn dat_mien_cho_tao_sheet(&mut self, mien_1: i32, mien_2: i32) {
        self.mien_not_k1 = mien_1;
        self.mien_not_k2 = mien_2;
        self.cap_nhat_hien_thi_mien(self.mien_not_k1, self.mien_not_k2);
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
            let mut phai_cho: bool = false;
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
        if self.hien_thi_thoi_gian {
            self.thoi_gian_label.as_mut().map(|label| {
                label.set_text(&format!("{:.2}", self.thoi_gian + self.thoi_gian_cho));
            });
        }

        let khoang_cach_nhip: f32 = self.toc_do;
        let thoi_gian_hieu_dung: f32 = self.thoi_gian + self.do_tre_phim;
        let nhip_cho_khuong: f32 = (thoi_gian_hieu_dung * self.bpm) / 60.0;

        if let Some(khuong) = self.khuong_1.as_mut() {
            khuong.bind_mut().xu_ly_di_chuyen_not(nhip_cho_khuong, khoang_cach_nhip);
        }
        if let Some(khuong) = self.khuong_2.as_mut() {
            khuong.bind_mut().xu_ly_di_chuyen_not(nhip_cho_khuong, khoang_cach_nhip);
        }
    }
}
