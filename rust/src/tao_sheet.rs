use godot::builtin::{ Callable, GString, PackedStringArray };
use godot::classes::{
    Button,
    CheckButton,
    Control,
    FileDialog,
    HSlider,
    IControl,
    Label,
    LineEdit,
    OptionButton,
    Panel,
    SpinBox,
    VBoxContainer,
};
use godot::global::godot_print;
use godot::obj::{ Base, Gd, WithBaseField };
use godot::prelude::{ godot_api, GodotClass };

use crate::ban_phim_midi::BanPhimMidi;
use crate::check_ten_not::chuyen_id_thanh_ten;
use crate::doc_sheet_json::{
    self,
    BaiHatJson,
    NotNhacJson,
    doc_file_json,
    luu_file_json,
    sap_xep_khuong_giu_nguyen,
    tien_xu_ly_khuong,
};
use crate::khuong_nhac::KhuongNhac;
use crate::midi_read::MIDIRead;
use crate::quan_ly_sheet::QuanLySheet;
use crate::quan_ly_tao_sheet::{QuanLyTaoSheet, ThongTinNotChon};

const NHIP_CHO_DAU: f32 = -2.0;
const THOI_GIAN_GOM_HOP_AM: f32 = 0.1;

#[derive(Clone, Debug)]
pub struct PhimDangGiu {
    pub so_phim: i32,
    pub khuong_idx: i32,
    pub nhip_dat: f32,
    pub thoi_gian_giu: f32,
}

/// Lưu trạng thái phím đang bấm trong chế độ "Ghi lại hành trình nốt"
#[derive(Clone, Debug)]
pub struct PhimGhiHanhTrinh {
    pub so_phim: i32,
    pub khuong_idx: i32,
    pub mau_not: String,
    pub nhip_bat_dau: f32,
    pub thoi_gian_giu: f32,
}

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct TaoSheet {
    #[base]
    base: Base<Control>,

    // Node hệ thống
    ban_phim_midi: Option<Gd<BanPhimMidi>>,
    quan_ly_sheet: Option<Gd<QuanLySheet>>,
    khuong_1: Option<Gd<KhuongNhac>>,
    khuong_2: Option<Gd<KhuongNhac>>,
    quan_ly_tao_sheet: Option<Gd<QuanLyTaoSheet>>,

    // Các Node UI ở Thanh Thông Tin (Trên)
    ten_input: Option<Gd<LineEdit>>,
    tac_gia_input: Option<Gd<LineEdit>>,
    nguoi_tao_input: Option<Gd<LineEdit>>,
    loai_nhip_opt: Option<Gd<OptionButton>>,
    bpm_spin: Option<Gd<SpinBox>>,
    mien_k1_spin: Option<Gd<SpinBox>>,
    mien_k2_spin: Option<Gd<SpinBox>>,

    // Các Node UI ở Thanh Công Cụ Soạn (Dưới)
    // Các Node UI ở Thanh Công Cụ Soạn (Dưới)
    nhip_label: Option<Gd<Label>>,
    thanh_tua: Option<Gd<HSlider>>,
    khuong_chon_opt: Option<Gd<OptionButton>>,
    buoc_nhip_opt: Option<Gd<OptionButton>>,
    mau_not_opt: Option<Gd<OptionButton>>,
    tu_dong_tien_check: Option<Gd<CheckButton>>,
    hien_ten_khac_check: Option<Gd<CheckButton>>,
    nghe_thu_btn: Option<Gd<Button>>,
    ghi_hanh_trinh_btn: Option<Gd<Button>>,

    // Các Node UI ở Panel Sửa Nốt / Chuyển Miền (Inspector Panel)
    panel_sua_not: Option<Gd<Panel>>,
    tieu_de_sua_label: Option<Gd<Label>>,
    hop_thuoc_tinh_not: Option<Gd<VBoxContainer>>,
    nhan_sua_id_not: Option<Gd<Label>>,
    sua_id_not_spin: Option<Gd<SpinBox>>,
    sua_mau_opt: Option<Gd<OptionButton>>,
    sua_dau_hoa_opt: Option<Gd<OptionButton>>,
    sua_ten_khac_input: Option<Gd<LineEdit>>,
    sua_nhip_giu_spin: Option<Gd<SpinBox>>,
    nhan_sua_nhip_dich: Option<Gd<Label>>,
    sua_nhip_dich_spin: Option<Gd<SpinBox>>,

    hop_thuoc_tinh_chuyen_mien: Option<Gd<VBoxContainer>>,
    sua_chuyen_not_spin: Option<Gd<SpinBox>>,
    nhan_sua_nhip_chuyen: Option<Gd<Label>>,
    sua_nhip_chuyen_spin: Option<Gd<SpinBox>>,

    // Hộp thoại File
    hop_thoai_luu: Option<Gd<FileDialog>>,
    hop_thoai_mo: Option<Gd<FileDialog>>,

    // Dữ liệu nốt của 2 khuông (bao gồm cả nốt thường và vạch ID 100)
    ds_not_k1: Vec<NotNhacJson>,
    ds_not_k2: Vec<NotNhacJson>,

    nhip_hien_tai: f32,
    dang_nghe_thu: bool,

    // Biến cho chế độ "Ghi lại hành trình nốt" (Live Recording không delay)
    dang_ghi_hanh_trinh: bool,
    phim_dang_giu_khi_ghi: Vec<PhimGhiHanhTrinh>,
    bo_nho_tam_ghi_k1: Vec<NotNhacJson>,
    bo_nho_tam_ghi_k2: Vec<NotNhacJson>,

    // Biến gom hợp âm & đo thời gian giữ phím (chế độ nhập từng bước)
    dang_gom_hop_am: bool,
    thoi_gian_gom_hop_am: f32,
    nhip_dat_hop_am: f32,
    nhip_giu_max_trong_cum: f32,
    cac_phim_dang_giu: Vec<PhimDangGiu>,
}

#[godot_api]
impl TaoSheet {
    #[func]
    pub fn khi_bat_tat_ten_khac(&mut self, bat: bool) {
        if let Some(mut k1) = self.khuong_1.clone() {
            k1.bind_mut().dat_hien_thi_ten_khac(bat);
        }
        if let Some(mut k2) = self.khuong_2.clone() {
            k2.bind_mut().dat_hien_thi_ten_khac(bat);
        }
    }
    fn lay_buoc_nhip(&self) -> f32 {
        let idx = self.buoc_nhip_opt
            .as_ref()
            .map(|o| o.get_selected())
            .unwrap_or(2);
        match idx {
            0 => 0.25,
            1 => 0.5,
            2 => 1.0,
            3 => 2.0,
            _ => 1.0,
        }
    }

    fn lay_loai_vach_nhip(&self) -> String {
        let idx = self.loai_nhip_opt
            .as_ref()
            .map(|o| o.get_selected())
            .unwrap_or(0);
        match idx {
            0 => "4/4".to_string(),
            1 => "3/4".to_string(),
            2 => "3/8".to_string(),
            3 => "2/4".to_string(),
            4 => "6/8".to_string(),
            _ => "4/4".to_string(),
        }
    }

    fn tinh_nhip_giu_lam_tron(&self, thoi_gian_giu: f32) -> f32 {
        if thoi_gian_giu < 0.25 {
            return 0.0;
        }

        let bpm = self.bpm_spin
            .as_ref()
            .map(|s| s.get_value() as f32)
            .unwrap_or(120.0);
        let loai_nhip = self.lay_loai_vach_nhip();
        let nhip_tho = (thoi_gian_giu * bpm) / 60.0;

        let buoc_lam_tron = match loai_nhip.as_str() {
            "4/4" | "2/4" | "3/4" => {
                if nhip_tho < 1.0 { 0.25 } else { 0.5 }
            }
            "3/8" | "6/8" => 0.5,
            _ => 0.5,
        };

        let nhip_lam_tron = (nhip_tho / buoc_lam_tron).round() * buoc_lam_tron;
        if nhip_lam_tron < buoc_lam_tron {
            0.0
        } else {
            nhip_lam_tron
        }
    }

    fn ket_thuc_gom_hop_am(&mut self) {
        self.dang_gom_hop_am = false;
        self.thoi_gian_gom_hop_am = 0.0;

        let tu_dong_tien = self.tu_dong_tien_check
            .as_ref()
            .map(|c| c.is_pressed())
            .unwrap_or(false);

        if tu_dong_tien {
            let buoc = if self.nhip_giu_max_trong_cum > 0.0 {
                self.nhip_giu_max_trong_cum
            } else {
                self.lay_buoc_nhip()
            };
            self.dat_nhip_hien_tai(self.nhip_dat_hop_am + buoc);
        }

        self.nhip_giu_max_trong_cum = 0.0;
    }

    /// Bật / Tắt chế độ "Ghi lại hành trình nốt" (Chỉ lưu và vẽ lại sau khi đánh xong)
    #[func]
    pub fn bat_tat_ghi_hanh_trinh(&mut self) {
        if !self.dang_ghi_hanh_trinh {
            // BẮT ĐẦU GHI HÀNH TRÌNH
            self.dang_ghi_hanh_trinh = true;
            self.dang_nghe_thu = false;
            self.dang_gom_hop_am = false;
            self.cac_phim_dang_giu.clear();
            self.phim_dang_giu_khi_ghi.clear();
            self.bo_nho_tam_ghi_k1.clear();
            self.bo_nho_tam_ghi_k2.clear();

            if let Some(mut btn_nghe) = self.nghe_thu_btn.clone() {
                btn_nghe.set_text("▶ Nghe thử");
            }
            if let Some(mut btn_ghi) = self.ghi_hanh_trinh_btn.clone() {
                btn_ghi.set_text("⏹ Lưu hành trình");
            }

            // Nếu đang ở đầu bài (<= 0.0), lùi về -2.0 nhịp để người dùng chuẩn bị bắt nhịp
            if self.nhip_hien_tai <= 0.0 && self.nhip_hien_tai > NHIP_CHO_DAU {
                self.dat_nhip_hien_tai(NHIP_CHO_DAU);
            }
            godot_print!("Bắt đầu ghi hành trình nốt (Zero-Delay)...");
        } else {
            // KẾT THÚC GHI -> CHỐT VÀ LƯU TOÀN BỘ NỐT VỪA ĐÁNH LÊN SHEET
            self.dang_ghi_hanh_trinh = false;
            if let Some(mut btn_ghi) = self.ghi_hanh_trinh_btn.clone() {
                btn_ghi.set_text("⏺ Ghi hành trình");
            }

            // Chốt nốt những phím người dùng vẫn còn đang giữ tay khi bấm dừng
            let cac_phim_con_giu: Vec<PhimGhiHanhTrinh> = self.phim_dang_giu_khi_ghi
                .drain(..)
                .collect();
            for phim in cac_phim_con_giu {
                let nhip_giu_moi = self.tinh_nhip_giu_lam_tron(phim.thoi_gian_giu);
                let not_moi: NotNhacJson = NotNhacJson {
                    id_not: phim.so_phim,
                    mau_not: phim.mau_not,
                    nhip_dich: phim.nhip_bat_dau,
                    nhip_giu: nhip_giu_moi,
                    chuyen_not: 0,
                    dau_hoa: String::new(),
                    ten_khac: String::new(),
                };
                if phim.khuong_idx == 0 {
                    self.bo_nho_tam_ghi_k1.push(not_moi);
                } else {
                    self.bo_nho_tam_ghi_k2.push(not_moi);
                }
            }

            let co_not_k1 = !self.bo_nho_tam_ghi_k1.is_empty();
            let co_not_k2 = !self.bo_nho_tam_ghi_k2.is_empty();

            // Gộp các nốt từ bộ nhớ tạm vào danh sách chính thức
            for not_moi in self.bo_nho_tam_ghi_k1.drain(..) {
                if
                    let Some(pos) = self.ds_not_k1
                        .iter()
                        .position(|n| {
                            n.id_not == not_moi.id_not &&
                                (n.nhip_dich - not_moi.nhip_dich).abs() < 0.01
                        })
                {
                    self.ds_not_k1[pos] = not_moi;
                } else {
                    self.ds_not_k1.push(not_moi);
                }
            }

            for not_moi in self.bo_nho_tam_ghi_k2.drain(..) {
                if
                    let Some(pos) = self.ds_not_k2
                        .iter()
                        .position(|n| {
                            n.id_not == not_moi.id_not &&
                                (n.nhip_dich - not_moi.nhip_dich).abs() < 0.01
                        })
                {
                    self.ds_not_k2[pos] = not_moi;
                } else {
                    self.ds_not_k2.push(not_moi);
                }
            }

            // Vẽ lại và tính chuyển miền 1 lần duy nhất sau khi đã đánh xong
            if co_not_k1 && co_not_k2 {
                self.cap_nhat_khuong_nhac(0, true);
            } else if co_not_k1 {
                self.cap_nhat_khuong_nhac(1, true);
            } else if co_not_k2 {
                self.cap_nhat_khuong_nhac(2, true);
            }
            godot_print!("Đã lưu hành trình nốt vào Sheet!");
        }
    }

    #[func]
    pub fn khi_bam_phim_midi(&mut self, so_phim: i32) {
        if self.dang_nghe_thu {
            return;
        }

        let khuong_idx = self.khuong_chon_opt
            .as_ref()
            .map(|o| o.get_selected())
            .unwrap_or(0);
        let mau_idx = self.mau_not_opt
            .as_ref()
            .map(|o| o.get_selected())
            .unwrap_or(0);
        let mau_not = format!("{}", mau_idx + 1);

        // 1. NẾU ĐANG GHI HÀNH TRÌNH: Chỉ lưu vào RAM, KHÔNG tạo Node để tránh delay 100%
        if self.dang_ghi_hanh_trinh {
            // Làm tròn thời điểm bấm về lưới 0.25 nhịp
            let nhip_bam = (self.nhip_hien_tai.max(0.0) * 4.0).round() / 4.0;
            self.phim_dang_giu_khi_ghi.retain(|p| p.so_phim != so_phim);
            self.phim_dang_giu_khi_ghi.push(PhimGhiHanhTrinh {
                so_phim,
                khuong_idx,
                mau_not,
                nhip_bat_dau: nhip_bam,
                thoi_gian_giu: 0.0,
            });
            return;
        }

        // 2. CHẾ ĐỘ NHẬP TỪNG BƯỚC BÌNH THƯỜNG
        let cho_phep_nhap_midi = self.tu_dong_tien_check
            .as_ref()
            .map(|c| c.is_pressed())
            .unwrap_or(true);
        if !cho_phep_nhap_midi {
            return;
        }

        if self.nhip_hien_tai < 0.0 {
            self.dat_nhip_hien_tai(0.0);
        }

        if !self.dang_gom_hop_am {
            self.dang_gom_hop_am = true;
            self.thoi_gian_gom_hop_am = 0.0;
            self.nhip_dat_hop_am = self.nhip_hien_tai;
            self.nhip_giu_max_trong_cum = 0.0;
        } else if self.thoi_gian_gom_hop_am > THOI_GIAN_GOM_HOP_AM {
            self.ket_thuc_gom_hop_am();
            self.dang_gom_hop_am = true;
            self.thoi_gian_gom_hop_am = 0.0;
            self.nhip_dat_hop_am = self.nhip_hien_tai;
        }
        let nhip_dat = self.nhip_dat_hop_am;

        let ds_muc_tieu = if khuong_idx == 0 { &mut self.ds_not_k1 } else { &mut self.ds_not_k2 };

        if
            let Some(pos) = ds_muc_tieu
                .iter()
                .position(|n| n.id_not == so_phim && (n.nhip_dich - nhip_dat).abs() < 0.01)
        {
            ds_muc_tieu.remove(pos);
            self.cac_phim_dang_giu.retain(|p| p.so_phim != so_phim);
        } else {
            ds_muc_tieu.push(NotNhacJson {
                id_not: so_phim,
                mau_not,
                nhip_dich: nhip_dat,
                nhip_giu: 0.0,
                chuyen_not: 0,
                dau_hoa: String::new(),
                ten_khac: String::new(),
            });
            self.cac_phim_dang_giu.retain(|p| p.so_phim != so_phim);
            self.cac_phim_dang_giu.push(PhimDangGiu {
                so_phim,
                khuong_idx,
                nhip_dat,
                thoi_gian_giu: 0.0,
            });
        }
        // Dọn dẹp vạch "auto" trước khi tính toán lại toàn bộ khuông
        let ds_muc_tieu = if khuong_idx == 0 { &mut self.ds_not_k1 } else { &mut self.ds_not_k2 };
        ds_muc_tieu.retain(|n| !(n.id_not == 100 && n.mau_not == "auto"));

        let khuong_so = if khuong_idx == 0 { 1 } else { 2 };
        // Gọi cập nhật và tính lại miền (true)
        self.cap_nhat_khuong_nhac(khuong_so, true);
    }

    #[func]
    pub fn khi_nha_phim_midi(&mut self, so_phim: i32) {
        if self.dang_nghe_thu {
            return;
        }

        // 1. NẾU ĐANG GHI HÀNH TRÌNH: Chốt độ dài nốt vào bộ nhớ tạm, KHÔNG vẽ lại Sheet ngay
        if self.dang_ghi_hanh_trinh {
            if let Some(pos) = self.phim_dang_giu_khi_ghi.iter().position(|p| p.so_phim == so_phim) {
                let phim = self.phim_dang_giu_khi_ghi.remove(pos);
                let nhip_giu_moi = self.tinh_nhip_giu_lam_tron(phim.thoi_gian_giu);
                let not_moi = NotNhacJson {
                    id_not: phim.so_phim,
                    mau_not: phim.mau_not,
                    nhip_dich: phim.nhip_bat_dau,
                    nhip_giu: nhip_giu_moi,
                    chuyen_not: 0,
                    dau_hoa: String::new(),
                    ten_khac: String::new(),
                };
                if phim.khuong_idx == 0 {
                    self.bo_nho_tam_ghi_k1.push(not_moi);
                } else {
                    self.bo_nho_tam_ghi_k2.push(not_moi);
                }
            }
            return;
        }

        // 2. CHẾ ĐỘ NHẬP TỪNG BƯỚC BÌNH THƯỜNG
        if let Some(pos) = self.cac_phim_dang_giu.iter().position(|p| p.so_phim == so_phim) {
            let phim_giu = self.cac_phim_dang_giu.remove(pos);
            let nhip_giu_moi = self.tinh_nhip_giu_lam_tron(phim_giu.thoi_gian_giu);

            let ds_muc_tieu = if phim_giu.khuong_idx == 0 {
                &mut self.ds_not_k1
            } else {
                &mut self.ds_not_k2
            };

            if
                let Some(not) = ds_muc_tieu
                    .iter_mut()
                    .find(|n| {
                        n.id_not == phim_giu.so_phim &&
                            (n.nhip_dich - phim_giu.nhip_dat).abs() < 0.01
                    })
            {
                not.nhip_giu = nhip_giu_moi;
                self.nhip_giu_max_trong_cum = self.nhip_giu_max_trong_cum.max(nhip_giu_moi);
            }

            let khuong_so = if phim_giu.khuong_idx == 0 { 1 } else { 2 };

            // SỬA Ở ĐÂY: Đổi `false` thành `true` để ép tính lại chuyển miền!
            self.cap_nhat_khuong_nhac(khuong_so, true);
        }
        if
            self.dang_gom_hop_am &&
            self.thoi_gian_gom_hop_am >= THOI_GIAN_GOM_HOP_AM &&
            self.cac_phim_dang_giu.is_empty()
        {
            self.ket_thuc_gom_hop_am();
        }
    }

    /// Thêm hoặc cập nhật ký tự chuyển miền (ID 100) cho vùng đang chọn hoặc tại nhịp hiện tại
    #[func]
    pub fn them_ky_tu_chuyen_mien(&mut self) {
        let ds_chon = if let Some(mut ql) = self.quan_ly_tao_sheet.clone() {
            ql.bind_mut().lay_thong_tin_da_chon()
        } else {
            Vec::new()
        };

        // Nếu người dùng đang chọn/khoanh vùng các nốt trên khuông:
        // Đặt vạch chuyển miền ngay tại nhịp đầu tiên của vùng được chọn!
        let (khuong_idx, nhip_dat, goi_y_id_tb) = if !ds_chon.is_empty() {
            let la_k1 = ds_chon[0].la_khuong_1;
            let min_nhip = ds_chon
                .iter()
                .map(|c| c.nhip_dich)
                .fold(f32::MAX, f32::min)
                .max(0.0);

            let cac_id: Vec<i32> = ds_chon
                .iter()
                .filter(|c| c.id_not != 100)
                .map(|c| c.id_not)
                .collect();
            let id_tb = if !cac_id.is_empty() {
                let min_id = *cac_id.iter().min().unwrap();
                let max_id = *cac_id.iter().max().unwrap();
                Some((min_id + max_id) / 2)
            } else {
                None
            };
            (if la_k1 { 0 } else { 1 }, min_nhip, id_tb)
        } else {
            let k_idx = self.khuong_chon_opt
                .as_ref()
                .map(|o| o.get_selected())
                .unwrap_or(0);
            (k_idx, self.nhip_hien_tai.max(0.0), None)
        };

        // Tính miền hiện tại ngay trước nhip_dat
        let mien_goc = if khuong_idx == 0 {
            self.mien_k1_spin
                .as_ref()
                .map(|s| s.get_value() as i32)
                .unwrap_or(36)
        } else {
            self.mien_k2_spin
                .as_ref()
                .map(|s| s.get_value() as i32)
                .unwrap_or(24)
        };

        let ds_muc_tieu = if khuong_idx == 0 { &mut self.ds_not_k1 } else { &mut self.ds_not_k2 };

        let mut mien_truoc_do = mien_goc;
        for not in ds_muc_tieu.iter() {
            if not.id_not == 100 && not.nhip_dich < nhip_dat - 0.01 {
                mien_truoc_do = (mien_truoc_do + not.chuyen_not).clamp(0, 67);
            }
        }

        // Tính giá trị dịch miền phù hợp cho vùng chọn (hoặc +4 nếu không có gợi ý)
        let mut chuyen_not_moi = if let Some(id_tb) = goi_y_id_tb {
            let mien_mong_muon = (id_tb - 8).clamp(0, 67);
            let chenh_lech = mien_mong_muon - mien_truoc_do;
            if chenh_lech != 0 {
                chenh_lech
            } else {
                4
            }
        } else {
            4
        };

        // Nếu tại nhịp này đã có sẵn vạch ID 100 (kể cả vạch auto) -> Cập nhật nó thành "thu_cong"
        if
            let Some(vach_cu) = ds_muc_tieu
                .iter_mut()
                .find(|n| n.id_not == 100 && (n.nhip_dich - nhip_dat).abs() < 0.01)
        {
            if goi_y_id_tb.is_none() {
                // Nếu bấm lần nữa vào cùng nhịp thì cộng thêm 4 bậc (hoặc đảo chiều)
                chuyen_not_moi = if vach_cu.chuyen_not >= 12 { -4 } else { vach_cu.chuyen_not + 4 };
            }
            vach_cu.chuyen_not = chuyen_not_moi;
            vach_cu.mau_not = String::from("thu_cong");
        } else {
            ds_muc_tieu.push(NotNhacJson {
                id_not: 100,
                mau_not: String::from("thu_cong"),
                nhip_dich: nhip_dat,
                nhip_giu: 0.0,
                chuyen_not: chuyen_not_moi,
                dau_hoa: String::new(),
                ten_khac: String::new(),
            });
        }

        let khuong_so = if khuong_idx == 0 { 1 } else { 2 };
        // Cập nhật lại khuông nhạc ngay lập tức để toàn bộ nốt dịch theo vạch chuyển miền mới
        self.cap_nhat_khuong_nhac(khuong_so, false);
    }

    /// Khi thay đổi giá trị trên SpinBox ID nốt (hiển thị tên nhạc lý C4, D#4... khi chọn 1 nốt)
    /// Khi thay đổi giá trị trên SpinBox ID nốt (hiển thị tên nhạc lý C4, D#4... khi chọn 1 nốt)
    #[func]
    pub fn khi_doi_spin_id_not(&self, gia_tri: f64) {
        if let Some(mut nhan) = self.nhan_sua_id_not.clone() {
            let text_ht = nhan.get_text().to_string();
            if !text_ht.contains('±') {
                let id: i32 = (gia_tri as i32).clamp(0, 83);
                let ten: &str = chuyen_id_thanh_ten(id, false);
                nhan.set_text(&format!("ID nốt ({}):", ten));
            }
        }
    }

    /// Khi danh sách chọn trong QuanLyTaoSheet thay đổi -> Cập nhật và hiện/ẩn PanelSuaNot
    #[func]
    pub fn khi_thay_doi_chon_not(&mut self) {
        let Some(mut ql) = self.quan_ly_tao_sheet.clone() else {
            return;
        };
        let ds_chon = ql.bind_mut().lay_thong_tin_da_chon();

        let Some(mut panel) = self.panel_sua_not.clone() else {
            return;
        };

        if ds_chon.is_empty() {
            panel.set_visible(false);
            return;
        }

        panel.set_visible(true);
        let so_luong = ds_chon.len();
        let la_chuyen_mien = ds_chon[0].id_not == 100;

        if let Some(mut hop_not) = self.hop_thuoc_tinh_not.clone() {
            hop_not.set_visible(!la_chuyen_mien);
        }
        if let Some(mut hop_cm) = self.hop_thuoc_tinh_chuyen_mien.clone() {
            hop_cm.set_visible(la_chuyen_mien);
        }

        if la_chuyen_mien {
            if let Some(mut lbl) = self.tieu_de_sua_label.clone() {
                lbl.set_text(&format!("Sửa Chuyển Miền ({} vạch)", so_luong));
            }
            if let Some(mut spin) = self.sua_chuyen_not_spin.clone() {
                spin.set_value_no_signal(ds_chon[0].chuyen_not as f64);
            }
            if let Some(mut nhan) = self.nhan_sua_nhip_chuyen.clone() {
                if so_luong == 1 {
                    nhan.set_text("Nhịp đích:");
                } else {
                    nhan.set_text("Dịch nhịp (±):");
                }
            }
            if let Some(mut spin) = self.sua_nhip_chuyen_spin.clone() {
                if so_luong == 1 {
                    spin.set_min(0.0);
                    spin.set_value_no_signal(ds_chon[0].nhip_dich as f64);
                } else {
                    spin.set_min(-100.0);
                    spin.set_value_no_signal(0.0);
                }
            }
        } else {
            if let Some(mut lbl) = self.tieu_de_sua_label.clone() {
                lbl.set_text(&format!("Đang chọn: {} nốt nhạc", so_luong));
            }

            // 1. Thiết lập ô Đổi ID Nốt (Chỉnh trực tiếp ID khi chọn 1 nốt, hoặc Dịch cao độ ± khi chọn nhiều nốt)
            if let Some(mut nhan_id) = self.nhan_sua_id_not.clone() {
                if so_luong == 1 {
                    let ten = chuyen_id_thanh_ten(ds_chon[0].id_not, false);
                    nhan_id.set_text(&format!("ID nốt ({}):", ten));
                } else {
                    nhan_id.set_text("Dịch ID nốt (±):");
                }
            }
            // Thay thế đoạn thiết lập spin_id trong hàm khi_thay_doi_chon_not bằng đoạn này:
            if let Some(mut spin_id) = self.sua_id_not_spin.clone() {
                // Tạm khóa phát tín hiệu khi thay đổi min/max để tránh gọi ngược lại TaoSheet
                spin_id.set_block_signals(true);

                if so_luong == 1 {
                    spin_id.set_min(0.0);
                    spin_id.set_max(83.0);
                    spin_id.set_value(ds_chon[0].id_not as f64);
                } else {
                    spin_id.set_min(-48.0);
                    spin_id.set_max(48.0);
                    spin_id.set_value(0.0);
                }

                spin_id.set_block_signals(false);
            }

            // 2. Thiết lập ô Màu nốt
            let mau_dau = &ds_chon[0].mau_not;
            let cung_mau = ds_chon.iter().all(|n| &n.mau_not == mau_dau);

            if let Some(mut opt) = self.sua_mau_opt.clone() {
                if cung_mau {
                    let idx = mau_dau.parse::<i32>().unwrap_or(0).clamp(0, 5);
                    opt.select(idx);
                } else {
                    opt.select(0); // 0 = "-- Giữ nguyên --"
                }
            }
            // Thiết lập ô Dấu hóa (0 = Tự động, 1 = Thăng #, 2 = Giáng ♭)
            if let Some(mut opt_dh) = self.sua_dau_hoa_opt.clone() {
                let co_phim_den = ds_chon
                    .iter()
                    .any(|c| crate::check_ten_not::co_thang_giang(c.id_not));
                opt_dh.set_disabled(!co_phim_den);

                if !co_phim_den {
                    opt_dh.select(0);
                } else {
                    let mut cung_dau_hoa = true;
                    let mut dh_dau = String::new();
                    for (idx, chon) in ds_chon.iter().enumerate() {
                        let ds = if chon.la_khuong_1 { &self.ds_not_k1 } else { &self.ds_not_k2 };
                        let dh = ds
                            .iter()
                            .find(
                                |n|
                                    n.id_not == chon.id_not &&
                                    (n.nhip_dich - chon.nhip_dich).abs() < 0.01
                            )
                            .map(|n| n.dau_hoa.clone())
                            .unwrap_or_default();
                        if idx == 0 {
                            dh_dau = dh;
                        } else if dh != dh_dau {
                            cung_dau_hoa = false;
                            break;
                        }
                    }

                    if cung_dau_hoa {
                        match dh_dau.as_str() {
                            "thang" => opt_dh.select(1),
                            "giang" => opt_dh.select(2),
                            _ => opt_dh.select(0), // Tự động
                        }
                    } else {
                        opt_dh.select(0);
                    }
                }
            }

            // 3. Thiết lập ô Nhịp giữ (Nếu chọn nhiều nốt có nhịp giữ khác nhau, để -1.0 để giữ nguyên trừ khi người dùng sửa)
            let giu_dau = ds_chon[0].nhip_giu;
            let cung_nhip_giu = ds_chon.iter().all(|n| (n.nhip_giu - giu_dau).abs() < 0.01);
            if let Some(mut spin) = self.sua_nhip_giu_spin.clone() {
                if so_luong == 1 || cung_nhip_giu {
                    spin.set_min(0.0);
                    spin.set_value_no_signal(giu_dau as f64);
                } else {
                    spin.set_min(-0.25);
                    spin.set_value_no_signal(-0.25); // -0.25 = Giữ nguyên độ dài riêng từng nốt
                }
            }

            // 4. Thiết lập ô Nhịp đích / Dịch nhịp
            if let Some(mut nhan) = self.nhan_sua_nhip_dich.clone() {
                if so_luong == 1 {
                    nhan.set_text("Nhịp đích:");
                } else {
                    nhan.set_text("Dịch nhịp (±):");
                }
            }
            if let Some(mut spin) = self.sua_nhip_dich_spin.clone() {
                if so_luong == 1 {
                    spin.set_min(0.0);
                    spin.set_value_no_signal(ds_chon[0].nhip_dich as f64);
                } else {
                    spin.set_min(-100.0);
                    spin.set_value_no_signal(0.0);
                }
            }
            // 5. Thiết lập ô Tên khác (chỉ hiện tên khi chọn đúng 1 nốt)
            if let Some(mut input) = self.sua_ten_khac_input.clone() {
                if so_luong == 1 {
                    let chon = &ds_chon[0];
                    input.set_text(&chon.ten_khac);
                    input.set_placeholder("VD: Đô, 1, Ngón 1...");
                } else {
                    input.set_text("");
                    input.set_placeholder("-- Giữ nguyên --");
                }
            }
        }
    }

    /// Khi bấm nút "Lưu thay đổi" trên PanelSuaNot -> Áp dụng cho tất cả các nốt/vạch đang chọn
    #[func]
    pub fn luu_thay_doi_not_chon(&mut self) {
        let Some(mut ql) = self.quan_ly_tao_sheet.clone() else {
            return;
        };
        let ds_chon: Vec<ThongTinNotChon> = ql.bind_mut().lay_thong_tin_da_chon();
        if ds_chon.is_empty() {
            return;
        }

        let so_luong = ds_chon.len();
        let la_chuyen_mien = ds_chon[0].id_not == 100;

        let mut idx_k1: Vec<usize> = Vec::new();
        let mut idx_k2: Vec<usize> = Vec::new();

        let dau_hoa_chon_idx = self.sua_dau_hoa_opt
            .as_ref()
            .map(|o| o.get_selected())
            .unwrap_or(0);

        for chon in &ds_chon {
            if chon.la_khuong_1 {
                if
                    let Some((i, _)) = self.ds_not_k1
                        .iter()
                        .enumerate()
                        .find(|(i, n)| {
                            !idx_k1.contains(i) &&
                                n.id_not == chon.id_not &&
                                (n.nhip_dich - chon.nhip_dich).abs() < 0.01
                        })
                {
                    idx_k1.push(i);
                }
            } else if
                let Some((i, _)) = self.ds_not_k2
                    .iter()
                    .enumerate()
                    .find(|(i, n)| {
                        !idx_k2.contains(i) &&
                            n.id_not == chon.id_not &&
                            (n.nhip_dich - chon.nhip_dich).abs() < 0.01
                    })
            {
                idx_k2.push(i);
            }
        }

        if la_chuyen_mien {
            let chuyen_not_moi = self.sua_chuyen_not_spin
                .as_ref()
                .map(|s| s.get_value() as i32)
                .unwrap_or(0);
            let gia_tri_nhip = self.sua_nhip_chuyen_spin
                .as_ref()
                .map(|s| s.get_value() as f32)
                .unwrap_or(0.0);

            for i in idx_k1 {
                self.ds_not_k1[i].chuyen_not = chuyen_not_moi;
                self.ds_not_k1[i].mau_not = String::from("thu_cong");
                if so_luong == 1 {
                    self.ds_not_k1[i].nhip_dich = gia_tri_nhip.max(0.0);
                } else {
                    self.ds_not_k1[i].nhip_dich = (self.ds_not_k1[i].nhip_dich + gia_tri_nhip).max(
                        0.0
                    );
                }
                if crate::check_ten_not::co_thang_giang(self.ds_not_k1[i].id_not) {
                    self.ds_not_k1[i].dau_hoa = match dau_hoa_chon_idx {
                        1 => "thang".to_string(),
                        2 => "giang".to_string(),
                        _ => String::new(), // 0 = Tự động theo nhạc lý
                    };
                }
            }
            for i in idx_k2 {
                self.ds_not_k2[i].chuyen_not = chuyen_not_moi;
                self.ds_not_k2[i].mau_not = String::from("thu_cong");
                if so_luong == 1 {
                    self.ds_not_k2[i].nhip_dich = gia_tri_nhip.max(0.0);
                } else {
                    self.ds_not_k2[i].nhip_dich = (self.ds_not_k2[i].nhip_dich + gia_tri_nhip).max(
                        0.0
                    );
                }
                if crate::check_ten_not::co_thang_giang(self.ds_not_k1[i].id_not) {
                    self.ds_not_k1[i].dau_hoa = match dau_hoa_chon_idx {
                        1 => "thang".to_string(),
                        2 => "giang".to_string(),
                        _ => String::new(), // 0 = Tự động theo nhạc lý
                    };
                }
            }
            // Vẽ lại cả 2 khuông để các nốt phía sau dịch chuyển theo vạch chuyển miền vừa sửa
            self.cap_nhat_khuong_nhac(0, false);
        } else {
            let gia_tri_id = self.sua_id_not_spin
                .as_ref()
                .map(|s| s.get_value() as i32)
                .unwrap_or(0);
            let mau_chon_idx = self.sua_mau_opt
                .as_ref()
                .map(|o| o.get_selected())
                .unwrap_or(0);
            let dau_hoa_chon_idx = self.sua_dau_hoa_opt
                .as_ref()
                .map(|o| o.get_selected())
                .unwrap_or(0);
            let nhip_giu_moi = self.sua_nhip_giu_spin
                .as_ref()
                .map(|s| s.get_value() as f32)
                .unwrap_or(-0.25);
            let gia_tri_nhip = self.sua_nhip_dich_spin
                .as_ref()
                .map(|s| s.get_value() as f32)
                .unwrap_or(0.0);
            let ten_khac_moi = self.sua_ten_khac_input
                .as_ref()
                .map(|i| i.get_text().to_string().trim().to_string())
                .unwrap_or_default();
            let giu_nguyen_ten_khac = self.sua_ten_khac_input
                .as_ref()
                .map(
                    |i|
                        i.get_placeholder().to_string() == "-- Giữ nguyên --" &&
                        ten_khac_moi.is_empty()
                )
                .unwrap_or(false);

            let mut co_doi_id_hoac_nhip = false;

            for i in idx_k1 {
                // Đổi ID
                if so_luong == 1 {
                    if self.ds_not_k1[i].id_not != gia_tri_id.clamp(0, 83) {
                        co_doi_id_hoac_nhip = true;
                    }
                    self.ds_not_k1[i].id_not = gia_tri_id.clamp(0, 83);
                } else if gia_tri_id != 0 {
                    co_doi_id_hoac_nhip = true;
                    self.ds_not_k1[i].id_not = (self.ds_not_k1[i].id_not + gia_tri_id).clamp(0, 83);
                }
                if !giu_nguyen_ten_khac {
                    self.ds_not_k1[i].ten_khac = ten_khac_moi.clone();
                }

                // Đổi Màu
                if (1..=5).contains(&mau_chon_idx) {
                    self.ds_not_k1[i].mau_not = format!("{}", mau_chon_idx);
                }

                // ĐỔI DẤU HÓA (Đã sửa lỗi logic)
                if crate::check_ten_not::co_thang_giang(self.ds_not_k1[i].id_not) {
                    self.ds_not_k1[i].dau_hoa = match dau_hoa_chon_idx {
                        1 => "thang".to_string(),
                        2 => "giang".to_string(),
                        _ => String::new(), // 0 = Trả về mặc định tự động
                    };
                    // Báo hiệu cần vẽ lại khuông để hình ảnh nốt cập nhật
                    co_doi_id_hoac_nhip = true;
                }

                // Đổi Nhịp giữ
                if nhip_giu_moi >= 0.0 {
                    self.ds_not_k1[i].nhip_giu = nhip_giu_moi;
                }

                // Đổi Nhịp đích
                if so_luong == 1 {
                    if (self.ds_not_k1[i].nhip_dich - gia_tri_nhip.max(0.0)).abs() > 0.01 {
                        co_doi_id_hoac_nhip = true;
                    }
                    self.ds_not_k1[i].nhip_dich = gia_tri_nhip.max(0.0);
                } else if gia_tri_nhip.abs() > 0.001 {
                    co_doi_id_hoac_nhip = true;
                    self.ds_not_k1[i].nhip_dich = (self.ds_not_k1[i].nhip_dich + gia_tri_nhip).max(
                        0.0
                    );
                }
            }

            for i in idx_k2 {
                // Đổi ID
                if so_luong == 1 {
                    if self.ds_not_k2[i].id_not != gia_tri_id.clamp(0, 83) {
                        co_doi_id_hoac_nhip = true;
                    }
                    self.ds_not_k2[i].id_not = gia_tri_id.clamp(0, 83);
                } else if gia_tri_id != 0 {
                    co_doi_id_hoac_nhip = true;
                    self.ds_not_k2[i].id_not = (self.ds_not_k2[i].id_not + gia_tri_id).clamp(0, 83);
                }

                if !giu_nguyen_ten_khac {
                    self.ds_not_k2[i].ten_khac = ten_khac_moi.clone();
                }

                // Đổi Màu
                if (1..=5).contains(&mau_chon_idx) {
                    self.ds_not_k2[i].mau_not = format!("{}", mau_chon_idx);
                }

                // ĐỔI DẤU HÓA (Đã sửa lỗi logic)
                if crate::check_ten_not::co_thang_giang(self.ds_not_k2[i].id_not) {
                    self.ds_not_k2[i].dau_hoa = match dau_hoa_chon_idx {
                        1 => "thang".to_string(),
                        2 => "giang".to_string(),
                        _ => String::new(),
                    };
                    co_doi_id_hoac_nhip = true;
                }

                // Đổi Nhịp giữ
                if nhip_giu_moi >= 0.0 {
                    self.ds_not_k2[i].nhip_giu = nhip_giu_moi;
                }

                // Đổi Nhịp đích
                if so_luong == 1 {
                    if (self.ds_not_k2[i].nhip_dich - gia_tri_nhip.max(0.0)).abs() > 0.01 {
                        co_doi_id_hoac_nhip = true;
                    }
                    self.ds_not_k2[i].nhip_dich = gia_tri_nhip.max(0.0);
                } else if gia_tri_nhip.abs() > 0.001 {
                    co_doi_id_hoac_nhip = true;
                    self.ds_not_k2[i].nhip_dich = (self.ds_not_k2[i].nhip_dich + gia_tri_nhip).max(
                        0.0
                    );
                }
            }

            // Vẽ lại khuông nhạc ngay lập tức để thấy thay đổi
            self.cap_nhat_khuong_nhac(0, co_doi_id_hoac_nhip);
        }
    }

    #[func]
    pub fn bo_chon_tat_ca(&mut self) {
        if let Some(mut ql) = self.quan_ly_tao_sheet.clone() {
            ql.bind_mut().lam_trong_mang_da_chon();
        }
    }

    #[func]
    pub fn ve_lai_sheet(&mut self) {
        self.cap_nhat_khuong_nhac(0, false);
    }

    pub fn ve_lai_sheet_tuy_chon(&mut self, tu_dong_chen_100: bool) {
        self.cap_nhat_khuong_nhac(0, tu_dong_chen_100);
    }

    #[func]
    pub fn xu_ly_not_khi_doi_mien(&mut self, khuong_so: i32) {
        if khuong_so == 1 || khuong_so == 0 {
            self.ds_not_k1.retain(|n| n.id_not != 100);
        }
        if khuong_so == 2 || khuong_so == 0 {
            self.ds_not_k2.retain(|n| n.id_not != 100);
        }

        self.cap_nhat_khuong_nhac(khuong_so, true);
    }

    pub fn cap_nhat_khuong_nhac(&mut self, khuong_so: i32, tinh_lai_chuyen_mien: bool) {
        
        if let Some(mut ql) = self.quan_ly_tao_sheet.clone() {
            ql.bind_mut().lam_trong_mang_da_chon_noi_bo();
        }
        if let Some(mut panel) = self.panel_sua_not.clone() {
            panel.set_visible(false);
        }

        let mien_k1 = self.mien_k1_spin
            .as_ref()
            .map(|s| s.get_value() as i32)
            .unwrap_or(36);
        let mien_k2 = self.mien_k2_spin
            .as_ref()
            .map(|s| s.get_value() as i32)
            .unwrap_or(24);
        let loai_vach = self.lay_loai_vach_nhip();
        let nhip_moi_o = match loai_vach.as_str() {
            "4/4" => 8,
            "3/4" => 6,
            "3/8" => 6,
            "2/4" => 4,
            "6/8" => 12,
            _ => 8,
        };

        if let Some(mut qls) = self.quan_ly_sheet.clone() {
            qls.bind_mut().cat_nhat_nhip_label(loai_vach);
        }

        let toc_do = self.quan_ly_sheet
            .as_ref()
            .map(|q| q.bind().get_toc_do())
            .unwrap_or(80.0);

        if khuong_so == 1 || khuong_so == 0 {
            let (mien_k1_moi, ds_k1_da_xu_ly) = if tinh_lai_chuyen_mien {
                tien_xu_ly_khuong(self.ds_not_k1.clone(), mien_k1, false, 0.0, f32::MAX)
            } else {
                sap_xep_khuong_giu_nguyen(self.ds_not_k1.clone(), mien_k1)
            };
            self.ds_not_k1 = ds_k1_da_xu_ly.clone();

            if mien_k1_moi != mien_k1 {
                if let Some(mut spin) = self.mien_k1_spin.clone() {
                    spin.set_value_no_signal(mien_k1_moi as f64);
                }
            }

            if let Some(mut k1) = self.khuong_1.clone() {
                let mut k_bind = k1.bind_mut();
                k_bind.xoa_tat_ca_not();
                k_bind.nhip_moi_o = nhip_moi_o;
                let mut mien_ht = mien_k1_moi;
                for i in 0..ds_k1_da_xu_ly.len() {
                    let not: &NotNhacJson = &ds_k1_da_xu_ly[i];
                    let dung_giang: bool = doc_sheet_json::xac_dinh_giang_tai_vi_tri(
                        &ds_k1_da_xu_ly,
                        i
                    );
                    k_bind.tao_not(
                        not.id_not,
                        not.mau_not.clone(),
                        not.nhip_dich,
                        not.nhip_giu,
                        not.chuyen_not,
                        mien_ht,
                        toc_do,
                        dung_giang,
                        not.ten_khac.clone(),
                        true
                    );
                    if not.id_not == 100 {
                        mien_ht = (mien_ht + not.chuyen_not).clamp(0, 67);
                    }
                }
                k_bind.cap_nhat_vi_tri_soan(self.nhip_hien_tai, toc_do);
            }
        }

        if khuong_so == 2 || khuong_so == 0 {
            let (mien_k2_moi, ds_k2_da_xu_ly) = if tinh_lai_chuyen_mien {
                tien_xu_ly_khuong(self.ds_not_k2.clone(), mien_k2, false, 0.0, f32::MAX)
            } else {
                sap_xep_khuong_giu_nguyen(self.ds_not_k2.clone(), mien_k2)
            };
            self.ds_not_k2 = ds_k2_da_xu_ly.clone();

            if mien_k2_moi != mien_k2 {
                if let Some(mut spin) = self.mien_k2_spin.clone() {
                    spin.set_value_no_signal(mien_k2_moi as f64);
                }
            }

            if let Some(mut k2) = self.khuong_2.clone() {
                let mut k_bind: godot::prelude::GdMut<'_, KhuongNhac> = k2.bind_mut();
                k_bind.xoa_tat_ca_not();
                k_bind.nhip_moi_o = nhip_moi_o;
                let mut mien_ht = mien_k2_moi;
                for i in 0..ds_k2_da_xu_ly.len() {
                    let not = &ds_k2_da_xu_ly[i];
                    let dung_giang = doc_sheet_json::xac_dinh_giang_tai_vi_tri(&ds_k2_da_xu_ly, i);
                    k_bind.tao_not(
                        not.id_not,
                        not.mau_not.clone(),
                        not.nhip_dich,
                        not.nhip_giu,
                        not.chuyen_not,
                        mien_ht,
                        toc_do,
                        dung_giang,
                        not.ten_khac.clone(),
                        true
                    );
                    if not.id_not == 100 {
                        mien_ht = (mien_ht + not.chuyen_not).clamp(0, 67);
                    }
                }
                k_bind.cap_nhat_vi_tri_soan(self.nhip_hien_tai, toc_do);
            }
        }

        let max_tua = self.tinh_max_thanh_tua().max(self.nhip_hien_tai + 2.0);
        if let Some(mut slider) = self.thanh_tua.clone() {
            slider.set_max(max_tua as f64);
        }
        self.cap_nhat_mien_theo_vi_tri_nhip();
    }

    #[func]
    pub fn khi_doi_mien_k1(&mut self, _gia_tri: f64) {
        self.xu_ly_not_khi_doi_mien(1);
    }

    #[func]
    pub fn khi_doi_mien_k2(&mut self, _gia_tri: f64) {
        self.xu_ly_not_khi_doi_mien(2);
    }

    #[func]
    pub fn khi_doi_loai_nhip(&mut self, _index: i64) {
        self.cap_nhat_khuong_nhac(0, false);
    }

    fn dat_nhip_hien_tai(&mut self, nhip_moi: f32) {
        let mut max_tua: f32 = self.thanh_tua
            .as_ref()
            .map(|s| s.get_max() as f32)
            .unwrap_or(2.0);

        // Khi đang ghi hành trình hoặc bấm Tiến nhịp vượt quá max_tua hiện tại -> Tự động mở rộng thanh trượt
        if nhip_moi > max_tua {
            max_tua = nhip_moi + 4.0;
            if let Some(mut slider) = self.thanh_tua.clone() {
                slider.set_max(max_tua as f64);
            }
        }

        self.nhip_hien_tai = nhip_moi.clamp(NHIP_CHO_DAU, max_tua);

        if let Some(mut label) = self.nhip_label.clone() {
            label.set_text(&format!("Nhịp: {:.2}", self.nhip_hien_tai));
        }

        if let Some(mut slider) = self.thanh_tua.clone() {
            slider.set_value_no_signal(self.nhip_hien_tai as f64);
        }

        let toc_do: f32 = self.quan_ly_sheet
            .as_ref()
            .map(|q| q.bind().get_toc_do())
            .unwrap_or(80.0);
        if let Some(mut k1) = self.khuong_1.clone() {
            k1.bind_mut().cap_nhat_vi_tri_soan(self.nhip_hien_tai, toc_do);
        }
        if let Some(mut k2) = self.khuong_2.clone() {
            k2.bind_mut().cap_nhat_vi_tri_soan(self.nhip_hien_tai, toc_do);
        }
        self.cap_nhat_mien_theo_vi_tri_nhip();
    }

    fn tinh_max_thanh_tua(&self) -> f32 {
        let max_nhip_ket_thuc: f32 = self.ds_not_k1
            .iter()
            .chain(self.ds_not_k2.iter())
            .map(|n| n.nhip_dich + n.nhip_giu)
            .fold(0.0_f32, f32::max);

        max_nhip_ket_thuc + 2.0
    }

    #[func]
    pub fn khi_tua_nhip(&mut self, gia_tri: f64) {
        self.dat_nhip_hien_tai(gia_tri as f32);
    }

    #[func]
    pub fn lui_nhip(&mut self) {
        let buoc = self.lay_buoc_nhip();
        self.dat_nhip_hien_tai(self.nhip_hien_tai - buoc);
    }

    #[func]
    pub fn tien_nhip(&mut self) {
        let buoc = self.lay_buoc_nhip();
        let nhip_dich_moi = self.nhip_hien_tai + buoc;
        if let Some(mut slider) = self.thanh_tua.clone() {
            if (nhip_dich_moi as f64) > slider.get_max() {
                slider.set_max((nhip_dich_moi + 2.0) as f64);
            }
        }
        self.dat_nhip_hien_tai(nhip_dich_moi);
    }

    /// Xóa các nốt hoặc vạch chuyển miền đã được chọn
    #[func]
    pub fn xoa_not_tai_nhip(&mut self) {
        let Some(mut ql_tao) = self.quan_ly_tao_sheet.clone() else {
            return;
        };

        let ds_can_xoa = ql_tao.bind_mut().lay_va_xoa_not_da_chon();
        if ds_can_xoa.is_empty() {
            return;
        }

        for (la_khuong_1, id_xoa, nhip_xoa) in ds_can_xoa {
            if la_khuong_1 {
                if
                    let Some(pos) = self.ds_not_k1
                        .iter()
                        .position(|n| n.id_not == id_xoa && (n.nhip_dich - nhip_xoa).abs() < 0.01)
                {
                    self.ds_not_k1.remove(pos);
                }
            } else if
                let Some(pos) = self.ds_not_k2
                    .iter()
                    .position(|n| n.id_not == id_xoa && (n.nhip_dich - nhip_xoa).abs() < 0.01)
            {
                self.ds_not_k2.remove(pos);
            }
        }
        self.ve_lai_sheet_tuy_chon(false);
    }

    #[func]
    pub fn bat_tat_nghe_thu(&mut self) {
        if self.dang_ghi_hanh_trinh {
            self.bat_tat_ghi_hanh_trinh();
        }

        self.dang_nghe_thu = !self.dang_nghe_thu;

        if self.dang_nghe_thu && self.nhip_hien_tai <= 0.0 && self.nhip_hien_tai > NHIP_CHO_DAU {
            self.dat_nhip_hien_tai(NHIP_CHO_DAU);
        }

        if let Some(mut btn) = self.nghe_thu_btn.clone() {
            if self.dang_nghe_thu {
                btn.set_text("⏸ Tạm dừng");
            } else {
                btn.set_text("▶ Nghe thử");
            }
        }
    }

    #[func]
    pub fn mo_hop_thoai_luu(&mut self) {
        if let Some(mut dialog) = self.hop_thoai_luu.clone() {
            dialog.popup_centered();
        }
    }

    #[func]
    pub fn mo_hop_thoai_mo(&mut self) {
        if let Some(mut dialog) = self.hop_thoai_mo.clone() {
            dialog.popup_centered();
        }
    }

    #[func]
    pub fn thuc_hien_luu_file(&mut self, duong_dan: GString) {
        let ten_bai_hat = self.ten_input
            .as_ref()
            .map(|i| i.get_text().to_string())
            .unwrap_or_default();
        let tac_gia = self.tac_gia_input
            .as_ref()
            .map(|i| i.get_text().to_string())
            .unwrap_or_default();
        let nguoi_tao_sheet = self.nguoi_tao_input
            .as_ref()
            .map(|i| i.get_text().to_string())
            .unwrap_or_default();
        let loai_vach_nhip = self.lay_loai_vach_nhip();
        let bpm = self.bpm_spin
            .as_ref()
            .map(|s| s.get_value() as f32)
            .unwrap_or(120.0);
        let mien_k1 = self.mien_k1_spin
            .as_ref()
            .map(|s| s.get_value() as i32)
            .unwrap_or(36);
        let mien_k2 = self.mien_k2_spin
            .as_ref()
            .map(|s| s.get_value() as i32)
            .unwrap_or(24);

        let (mien_not_bat_dau_k1, khuong_1) = sap_xep_khuong_giu_nguyen(
            self.ds_not_k1.clone(),
            mien_k1
        );
        let (mien_not_bat_dau_k2, khuong_2) = sap_xep_khuong_giu_nguyen(
            self.ds_not_k2.clone(),
            mien_k2
        );

        let bai_hat = BaiHatJson {
            ten_bai_hat,
            tac_gia,
            nguoi_tao_sheet,
            loai_vach_nhip,
            bpm,
            mien_not_bat_dau_k1,
            mien_not_bat_dau_k2,
            khuong_1,
            khuong_2,
        };

        luu_file_json(&duong_dan.to_string(), &bai_hat);
    }

    #[func]
    pub fn thuc_hien_mo_file(&mut self, duong_dan: GString) {
        if let Some(bai_hat) = doc_file_json(&duong_dan.to_string()) {
            if let Some(mut input) = self.ten_input.clone() {
                input.set_text(&bai_hat.ten_bai_hat);
            }
            if let Some(mut input) = self.tac_gia_input.clone() {
                input.set_text(&bai_hat.tac_gia);
            }
            if let Some(mut input) = self.nguoi_tao_input.clone() {
                input.set_text(&bai_hat.nguoi_tao_sheet);
            }

            if let Some(mut spin) = self.bpm_spin.clone() {
                spin.set_value_no_signal(bai_hat.bpm as f64);
            }
            if let Some(mut spin) = self.mien_k1_spin.clone() {
                spin.set_value_no_signal(bai_hat.mien_not_bat_dau_k1 as f64);
            }
            if let Some(mut spin) = self.mien_k2_spin.clone() {
                spin.set_value_no_signal(bai_hat.mien_not_bat_dau_k2 as f64);
            }

            if let Some(mut opt) = self.loai_nhip_opt.clone() {
                let idx = match bai_hat.loai_vach_nhip.as_str() {
                    "4/4" => 0,
                    "3/4" => 1,
                    "3/8" => 2,
                    "2/4" => 3,
                    "6/8" => 4,
                    _ => 0,
                };
                opt.select(idx);
            }

            self.ds_not_k1 = bai_hat.khuong_1;
            self.ds_not_k2 = bai_hat.khuong_2;

            self.dat_nhip_hien_tai(0.0);
            self.ve_lai_sheet_tuy_chon(true);
        }
    }

    #[func]
    pub fn mo_hop_thoai_nhap_midi(&mut self) {
        if let Some(mut dialog) = self.base().try_get_node_as::<FileDialog>("HopThoaiNhapMid") {
            dialog.popup_centered();
        }
    }

    #[func]
    pub fn thuc_hien_nhap_midi(&mut self, duong_dan: GString) {
        let mut midi: MIDIRead = MIDIRead::new(&duong_dan.to_string(), "", "", "Tác giả");

        if midi.khoi_tao() {
            if let Some(mut input) = self.ten_input.clone() {
                input.set_text(&midi.ten_bai_hat);
            }
            if let Some(mut input) = self.tac_gia_input.clone() {
                input.set_text(&midi.ten_tac_gia);
            }
            if let Some(mut input) = self.nguoi_tao_input.clone() {
                input.set_text(&midi.ten_nguoi_tao);
            }
            if let Some(mut spin) = self.bpm_spin.clone() {
                spin.set_value_no_signal(midi.bpm as f64);
            }
            if let Some(mut spin) = self.mien_k1_spin.clone() {
                spin.set_value_no_signal(midi.mien_bat_dau_k1 as f64);
            }
            if let Some(mut spin) = self.mien_k2_spin.clone() {
                spin.set_value_no_signal(midi.mien_bat_dau_k2 as f64);
            }
            if let Some(mut opt) = self.loai_nhip_opt.clone() {
                let idx = match midi.loai_vach_nhip.as_str() {
                    "4/4" => 0,
                    "3/4" => 1,
                    "3/8" => 2,
                    "2/4" => 3,
                    "6/8" => 4,
                    _ => 0,
                };
                opt.select(idx);
            }

            self.ds_not_k1 = midi.khuong_1;
            self.ds_not_k2 = midi.khuong_2;
            self.dat_nhip_hien_tai(0.0);
            self.ve_lai_sheet_tuy_chon(false);
            godot_print!("Nhập file MIDI thành công: {}", midi.ten_bai_hat);
        }
    }

    pub fn cap_nhat_mien_theo_vi_tri_nhip(&mut self) {
        let mien_k1_goc = self.mien_k1_spin
            .as_ref()
            .map(|s| s.get_value() as i32)
            .unwrap_or(36);
        let mien_k2_goc = self.mien_k2_spin
            .as_ref()
            .map(|s| s.get_value() as i32)
            .unwrap_or(24);

        let mut mien_k1_ht = mien_k1_goc;
        for not in &self.ds_not_k1 {
            if not.id_not == 100 && not.nhip_dich <= self.nhip_hien_tai + 0.01 {
                mien_k1_ht = (mien_k1_ht + not.chuyen_not).clamp(0, 67);
            }
        }

        let mut mien_k2_ht = mien_k2_goc;
        for not in &self.ds_not_k2 {
            if not.id_not == 100 && not.nhip_dich <= self.nhip_hien_tai + 0.01 {
                mien_k2_ht = (mien_k2_ht + not.chuyen_not).clamp(0, 67);
            }
        }

        if let Some(mut qls) = self.quan_ly_sheet.clone() {
            qls.bind_mut().dat_mien_cho_tao_sheet(mien_k1_ht, mien_k2_ht);
        }
    }

    #[func]
    pub fn khi_keo_tha_file(&mut self, danh_sach_file: PackedStringArray) {
        if danh_sach_file.is_empty() {
            return;
        }

        if let Some(duong_dan) = danh_sach_file.get(0) {
            let duong_dan_str = duong_dan.to_string().to_lowercase();

            if duong_dan_str.ends_with(".mid") || duong_dan_str.ends_with(".midi") {
                godot_print!("Đã kéo thả file MIDI: {}", duong_dan);
                self.thuc_hien_nhap_midi(duong_dan);
            } else if duong_dan_str.ends_with(".json") {
                godot_print!("Đã kéo thả file JSON: {}", duong_dan);
                self.thuc_hien_mo_file(duong_dan);
            } else {
                godot_print!("Định dạng file không hỗ trợ! Chỉ nhận .mid hoặc .json");
            }
        }
    }
}

#[godot_api]
impl IControl for TaoSheet {
    fn enter_tree(&mut self) {
        // Bật hiển thị Collision Shape khi vào trang TaoSheet
        if let Some(mut tree) = self.base().get_tree() {
            tree.set_debug_collisions_hint(true);
        }
    }

    fn exit_tree(&mut self) {
        // Tắt hiển thị Collision Shape khi thoát khỏi trang TaoSheet
        if let Some(mut tree) = self.base().get_tree() {
            tree.set_debug_collisions_hint(false);
        }
    }
    fn ready(&mut self) {
        self.ban_phim_midi = self.base().try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano");
        self.quan_ly_sheet = self.base().try_get_node_as::<QuanLySheet>("QuanLySheet");
        self.sua_ten_khac_input = self
            .base()
            .try_get_node_as::<LineEdit>(
                "PanelSuaNot/VBox/HopThuocTinhNot/HangTenKhac/SuaTenKhac_Input"
            );

        self.khuong_1 = self.base().try_get_node_as::<KhuongNhac>("QuanLySheet/K1");
        self.khuong_2 = self.base().try_get_node_as::<KhuongNhac>("QuanLySheet/K2");
        if let Some(mut qls) = self.quan_ly_sheet.clone() {
            qls.bind_mut().bat_tat_cho_phep_xoa_not(false);
            qls.bind_mut().set_hien_thi_thoi_gian(false);
            qls.set_process(false);
        }
        self.quan_ly_tao_sheet = self.base().try_get_node_as::<QuanLyTaoSheet>("QuanLyTaoSheet");

        // Lấy các Node UI trên ThanhThongTin
        self.ten_input = self
            .base()
            .try_get_node_as::<LineEdit>("ThanhThongTin/HBox/TenBaiHat_Input");
        self.tac_gia_input = self
            .base()
            .try_get_node_as::<LineEdit>("ThanhThongTin/HBox/TacGia_Input");
        self.nguoi_tao_input = self
            .base()
            .try_get_node_as::<LineEdit>("ThanhThongTin/HBox/NguoiTao_Input");
        self.loai_nhip_opt = self
            .base()
            .try_get_node_as::<OptionButton>("ThanhThongTin/HBox/LoaiNhip_Option");
        self.bpm_spin = self.base().try_get_node_as::<SpinBox>("ThanhThongTin/HBox/BPM_Spin");
        self.mien_k1_spin = self
            .base()
            .try_get_node_as::<SpinBox>("ThanhThongTin/HBox/MienK1_Spin");
        self.mien_k2_spin = self
            .base()
            .try_get_node_as::<SpinBox>("ThanhThongTin/HBox/MienK2_Spin");

        // Lấy các Node UI trên ThanhCongCuSoan
        self.nhip_label = self
            .base()
            .try_get_node_as::<Label>("ThanhCongCuSoan/VBox/HangTuaNhip/NhipHienTai_Label");
        self.thanh_tua = self
            .base()
            .try_get_node_as::<HSlider>("ThanhCongCuSoan/VBox/HangTuaNhip/ThanhTuaNhip");
        self.khuong_chon_opt = self
            .base()
            .try_get_node_as::<OptionButton>("ThanhCongCuSoan/VBox/HangCongCu/KhuongChon_Option");
        self.buoc_nhip_opt = self
            .base()
            .try_get_node_as::<OptionButton>("ThanhCongCuSoan/VBox/HangCongCu/BuocNhip_Option");
        self.mau_not_opt = self
            .base()
            .try_get_node_as::<OptionButton>("ThanhCongCuSoan/VBox/HangCongCu/MauNot_Option");
        self.tu_dong_tien_check = self
            .base()
            .try_get_node_as::<CheckButton>("ThanhCongCuSoan/VBox/HangCongCu/TuDongTien_Check");
        self.hien_ten_khac_check = self
            .base()
            .try_get_node_as::<CheckButton>("ThanhCongCuSoan/VBox/HangCongCu/HienTenKhac_Check");
        let call_ten_khac: Callable = self.base().callable("khi_bat_tat_ten_khac");
        if let Some(mut chk) = self.hien_ten_khac_check.clone() {
            chk.connect("toggled", &call_ten_khac);
        }
        self.nghe_thu_btn = self
            .base()
            .try_get_node_as::<Button>("ThanhCongCuSoan/VBox/HangCongCu/NgheThu_Btn");
        self.ghi_hanh_trinh_btn = self
            .base()
            .try_get_node_as::<Button>("ThanhCongCuSoan/VBox/HangCongCu/GhiHanhTrinh_Btn");

        // Lấy các Node UI trên PanelSuaNot (Inspector)
        self.panel_sua_not = self.base().try_get_node_as::<Panel>("PanelSuaNot");
        self.tieu_de_sua_label = self
            .base()
            .try_get_node_as::<Label>("PanelSuaNot/VBox/TieuDe_Label");
        self.hop_thuoc_tinh_not = self
            .base()
            .try_get_node_as::<VBoxContainer>("PanelSuaNot/VBox/HopThuocTinhNot");
        self.nhan_sua_id_not = self
            .base()
            .try_get_node_as::<Label>("PanelSuaNot/VBox/HopThuocTinhNot/HangIdNot/NhanSuaIdNot");
        self.sua_id_not_spin = self
            .base()
            .try_get_node_as::<SpinBox>("PanelSuaNot/VBox/HopThuocTinhNot/HangIdNot/SuaIdNot_Spin");
        self.sua_mau_opt = self
            .base()
            .try_get_node_as::<OptionButton>(
                "PanelSuaNot/VBox/HopThuocTinhNot/HangMau/SuaMau_Option"
            );
        self.sua_dau_hoa_opt = self
            .base()
            .try_get_node_as::<OptionButton>(
                "PanelSuaNot/VBox/HopThuocTinhNot/HangDauHoa/SuaDauHoa_Option"
            );
        self.sua_nhip_giu_spin = self
            .base()
            .try_get_node_as::<SpinBox>(
                "PanelSuaNot/VBox/HopThuocTinhNot/HangNhipGiu/SuaNhipGiu_Spin"
            );
        self.nhan_sua_nhip_dich = self
            .base()
            .try_get_node_as::<Label>(
                "PanelSuaNot/VBox/HopThuocTinhNot/HangNhipDich/NhanSuaNhipDich"
            );
        self.sua_nhip_dich_spin = self
            .base()
            .try_get_node_as::<SpinBox>(
                "PanelSuaNot/VBox/HopThuocTinhNot/HangNhipDich/SuaNhipDich_Spin"
            );

        self.hop_thuoc_tinh_chuyen_mien = self
            .base()
            .try_get_node_as::<VBoxContainer>("PanelSuaNot/VBox/HopThuocTinhChuyenMien");
        self.sua_chuyen_not_spin = self
            .base()
            .try_get_node_as::<SpinBox>(
                "PanelSuaNot/VBox/HopThuocTinhChuyenMien/HangChuyenNot/SuaChuyenNot_Spin"
            );
        self.nhan_sua_nhip_chuyen = self
            .base()
            .try_get_node_as::<Label>(
                "PanelSuaNot/VBox/HopThuocTinhChuyenMien/HangNhipChuyen/NhanSuaNhipChuyen"
            );
        self.sua_nhip_chuyen_spin = self
            .base()
            .try_get_node_as::<SpinBox>(
                "PanelSuaNot/VBox/HopThuocTinhChuyenMien/HangNhipChuyen/SuaNhipChuyen_Spin"
            );

        self.hop_thoai_luu = self.base().try_get_node_as::<FileDialog>("HopThoaiLuuFile");
        self.hop_thoai_mo = self.base().try_get_node_as::<FileDialog>("HopThoaiMoFile");

        // Kết nối tín hiệu thay đổi chọn nốt từ QuanLyTaoSheet
        let call_chon: Callable = self.base().callable("khi_thay_doi_chon_not");
        if let Some(mut ql) = self.quan_ly_tao_sheet.clone() {
            ql.connect("thay_doi_chon_not", &call_chon);
        }

        // Kết nối sự kiện đổi ID nốt trên SpinBox để cập nhật tên nốt (C4, D4...)
        let call_spin_id: Callable = self.base().callable("khi_doi_spin_id_not");
        if let Some(mut spin_id) = self.sua_id_not_spin.clone() {
            spin_id.connect("value_changed", &call_spin_id);
        }

        // Kết nối sự kiện kéo thả file
        let call_keo_tha: Callable = self.base().callable("khi_keo_tha_file");
        if let Some(mut window) = self.base().get_window() {
            window.connect("files_dropped", &call_keo_tha);
        }

        // Kết nối các nút trong PanelSuaNot
        let call_luu_sua: Callable = self.base().callable("luu_thay_doi_not_chon");
        if
            let Some(mut btn) = self
                .base()
                .try_get_node_as::<Button>("PanelSuaNot/VBox/HangNutPanel/LuuSuaNot_Btn")
        {
            btn.connect("pressed", &call_luu_sua);
        }

        let call_nhap_mid = self.base().callable("mo_hop_thoai_nhap_midi");
        if let Some(mut btn) = self.base().try_get_node_as::<Button>("ThanhThongTin/HBox/NhapMid") {
            btn.connect("pressed", &call_nhap_mid);
        }
        let call_nhap_xong_mid: Callable = self.base().callable("thuc_hien_nhap_midi");
        if let Some(mut dialog) = self.base().try_get_node_as::<FileDialog>("HopThoaiNhapMid") {
            dialog.connect("file_selected", &call_nhap_xong_mid);
        }

        let call_xoa_panel = self.base().callable("xoa_not_tai_nhip");
        if
            let Some(mut btn) = self
                .base()
                .try_get_node_as::<Button>("PanelSuaNot/VBox/HangNutPanel/XoaChon_Btn")
        {
            btn.connect("pressed", &call_xoa_panel);
        }

        let call_bo_chon = self.base().callable("bo_chon_tat_ca");
        if
            let Some(mut btn) = self
                .base()
                .try_get_node_as::<Button>("PanelSuaNot/VBox/HangNutPanel/BoChon_Btn")
        {
            btn.connect("pressed", &call_bo_chon);
        }

        // Kết nối nút Thêm Chuyển Miền
        let call_them_cm = self.base().callable("them_ky_tu_chuyen_mien");
        if
            let Some(mut btn) = self
                .base()
                .try_get_node_as::<Button>("ThanhCongCuSoan/VBox/HangCongCu/ThemChuyenMien_Btn")
        {
            btn.connect("pressed", &call_them_cm);
        }

        // Kết nối nút Ghi Hành Trình Nốt
        let call_ghi_ht = self.base().callable("bat_tat_ghi_hanh_trinh");
        if let Some(mut btn) = self.ghi_hanh_trinh_btn.clone() {
            btn.connect("pressed", &call_ghi_ht);
        }

        // Kết nối sự kiện bàn phím MIDI
        let call_midi = self.base().callable("khi_bam_phim_midi");
        let call_nha_midi = self.base().callable("khi_nha_phim_midi");
        if let Some(mut midi) = self.ban_phim_midi.clone() {
            midi.connect("phim_vua_duoc_bam", &call_midi);
            midi.connect("phim_vua_duoc_nha", &call_nha_midi);
        }

        let call_tua = self.base().callable("khi_tua_nhip");
        if let Some(mut slider) = self.thanh_tua.clone() {
            slider.set_min(NHIP_CHO_DAU as f64);
            slider.connect("value_changed", &call_tua);
        }

        let call_mien_k1: Callable = self.base().callable("khi_doi_mien_k1");
        if let Some(mut s1) = self.mien_k1_spin.clone() {
            s1.connect("value_changed", &call_mien_k1);
        }

        let call_mien_k2: Callable = self.base().callable("khi_doi_mien_k2");
        if let Some(mut s2) = self.mien_k2_spin.clone() {
            s2.connect("value_changed", &call_mien_k2);
        }

        let call_loai_nhip: Callable = self.base().callable("khi_doi_loai_nhip");
        if let Some(mut opt) = self.loai_nhip_opt.clone() {
            opt.connect("item_selected", &call_loai_nhip);
        }

        let call_lui = self.base().callable("lui_nhip");
        if
            let Some(mut btn) = self
                .base()
                .try_get_node_as::<Button>("ThanhCongCuSoan/VBox/HangCongCu/LuiNhip_Btn")
        {
            btn.connect("pressed", &call_lui);
        }

        let call_tien: Callable = self.base().callable("tien_nhip");
        if
            let Some(mut btn) = self
                .base()
                .try_get_node_as::<Button>("ThanhCongCuSoan/VBox/HangCongCu/TienNhip_Btn")
        {
            btn.connect("pressed", &call_tien);
        }

        // let call_xoa = self.base().callable("xoa_not_tai_nhip");
        // if let Some(mut btn) = self
        //     .base()
        //     .try_get_node_as::<Button>("ThanhCongCuSoan/VBox/HangCongCu/XoaNot_Btn")
        // {
        //     btn.connect("pressed", &call_xoa);
        // }

        let call_nghe = self.base().callable("bat_tat_nghe_thu");
        if let Some(mut btn) = self.nghe_thu_btn.clone() {
            btn.connect("pressed", &call_nghe);
        }

        let call_mo_luu = self.base().callable("mo_hop_thoai_luu");
        if
            let Some(mut btn) = self
                .base()
                .try_get_node_as::<Button>("ThanhThongTin/HBox/LuuFile_Btn")
        {
            btn.connect("pressed", &call_mo_luu);
        }

        let call_mo_file = self.base().callable("mo_hop_thoai_mo");
        if
            let Some(mut btn) = self
                .base()
                .try_get_node_as::<Button>("ThanhThongTin/HBox/MoFile_Btn")
        {
            btn.connect("pressed", &call_mo_file);
        }

        let call_luu_xong = self.base().callable("thuc_hien_luu_file");
        if let Some(mut dialog) = self.hop_thoai_luu.clone() {
            dialog.connect("file_selected", &call_luu_xong);
        }

        let call_mo_xong = self.base().callable("thuc_hien_mo_file");
        if let Some(mut dialog) = self.hop_thoai_mo.clone() {
            dialog.connect("file_selected", &call_mo_xong);
        }

        self.ve_lai_sheet_tuy_chon(false);
    }

    fn process(&mut self, delta: f64) {
        let dt: f32 = delta as f32;

        // 1. Xử lý khi đang GHI HÀNH TRÌNH NỐT (Chạy mượt không tạo/xóa Node)
        if self.dang_ghi_hanh_trinh {
            for phim in &mut self.phim_dang_giu_khi_ghi {
                phim.thoi_gian_giu += dt;
            }

            let bpm = self.bpm_spin
                .as_ref()
                .map(|s| s.get_value() as f32)
                .unwrap_or(120.0);
            let nhip_moi = self.nhip_hien_tai + (dt * bpm) / 60.0;
            self.dat_nhip_hien_tai(nhip_moi);
            return;
        }

        // 2. Xử lý gom hợp âm ở chế độ nhập từng bước
        if self.dang_gom_hop_am {
            self.thoi_gian_gom_hop_am += dt;

            for phim in &mut self.cac_phim_dang_giu {
                phim.thoi_gian_giu += dt;
            }

            if
                self.thoi_gian_gom_hop_am >= THOI_GIAN_GOM_HOP_AM &&
                self.cac_phim_dang_giu.is_empty()
            {
                self.ket_thuc_gom_hop_am();
            }
        }

        // 3. Xử lý khi đang NGHE THỬ
        if !self.dang_nghe_thu {
            return;
        }

        let bpm = self.bpm_spin
            .as_ref()
            .map(|s| s.get_value() as f32)
            .unwrap_or(120.0);
        let nhip_truoc = self.nhip_hien_tai;
        let nhip_moi = nhip_truoc + (dt * bpm) / 60.0;

        if let Some(mut midi) = self.ban_phim_midi.clone() {
            let mut midi_bind = midi.bind_mut();
            for not in self.ds_not_k1.iter().chain(self.ds_not_k2.iter()) {
                if not.id_not != 100 && not.nhip_dich > nhip_truoc && not.nhip_dich <= nhip_moi {
                    let thoi_gian_giu_ms = if not.nhip_giu > 0.0 {
                        (((not.nhip_giu * 60.0) / bpm) * 1000.0) as u64
                    } else {
                        300
                    };
                    midi_bind.phat_am_thanh_not(not.id_not, 100, thoi_gian_giu_ms);
                }
            }
        }

        self.dat_nhip_hien_tai(nhip_moi);

        if self.nhip_hien_tai >= self.tinh_max_thanh_tua() {
            self.dang_nghe_thu = false;
            if let Some(mut btn) = self.nghe_thu_btn.clone() {
                btn.set_text("▶ Nghe thử");
            }
        }
    }
}
