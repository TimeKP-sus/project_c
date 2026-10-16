use godot::builtin::{ Callable, GString };
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
    SpinBox,
};
use godot::obj::{ Base, Gd, WithBaseField };
use godot::prelude::{ godot_api, GodotClass };

use crate::ban_phim_midi::BanPhimMidi;
use crate::doc_sheet_json::{
    doc_file_json,
    luu_file_json,
    tien_xu_ly_khuong,
    BaiHatJson,
    NotNhacJson,
};
use crate::khuong_nhac::KhuongNhac;
use crate::quan_ly_sheet::QuanLySheet;

const NHIP_CHO_DAU: f32 = -2.0;

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

    // Các Node UI ở Thanh Thông Tin (Trên)
    ten_input: Option<Gd<LineEdit>>,
    tac_gia_input: Option<Gd<LineEdit>>,
    nguoi_tao_input: Option<Gd<LineEdit>>,
    loai_nhip_opt: Option<Gd<OptionButton>>,
    bpm_spin: Option<Gd<SpinBox>>,
    mien_k1_spin: Option<Gd<SpinBox>>,
    mien_k2_spin: Option<Gd<SpinBox>>,

    // Các Node UI ở Thanh Công Cụ Soạn (Dưới)
    nhip_label: Option<Gd<Label>>,
    thanh_tua: Option<Gd<HSlider>>,
    khuong_chon_opt: Option<Gd<OptionButton>>,
    buoc_nhip_opt: Option<Gd<OptionButton>>,
    nhip_giu_spin: Option<Gd<SpinBox>>,
    mau_not_opt: Option<Gd<OptionButton>>,
    tu_dong_tien_check: Option<Gd<CheckButton>>,
    nghe_thu_btn: Option<Gd<Button>>,

    // Hộp thoại File
    hop_thoai_luu: Option<Gd<FileDialog>>,
    hop_thoai_mo: Option<Gd<FileDialog>>,

    // Dữ liệu nốt gốc đang soạn (chưa chèn ID 100 tự động)
    ds_not_k1: Vec<NotNhacJson>,
    ds_not_k2: Vec<NotNhacJson>,

    nhip_hien_tai: f32,
    dang_nghe_thu: bool,
}

#[godot_api]
impl TaoSheet {
    /// Lấy giá trị bước tiến nhịp từ OptionButton (0.25, 0.5, 1.0, 2.0)
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

    /// Lấy chuỗi loại vạch nhịp ("4/4", "3/4",...)
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

    /// Xử lý khi người dùng bấm một phím trên đàn MIDI để chèn nốt vào Sheet
    #[func]
    pub fn khi_bam_phim_midi(&mut self, so_phim: i32) {
        // Nếu đang ở chế độ phát nghe thử thì không chèn nốt
        if self.dang_nghe_thu {
            return;
        }
        if self.nhip_hien_tai < 0.0 {
            self.dat_nhip_hien_tai(0.0);
        }

        let khuong_idx = self.khuong_chon_opt
            .as_ref()
            .map(|o| o.get_selected())
            .unwrap_or(0);
        let nhip_giu = self.nhip_giu_spin
            .as_ref()
            .map(|s| s.get_value() as f32)
            .unwrap_or(0.0);
        let mau_idx = self.mau_not_opt
            .as_ref()
            .map(|o| o.get_selected())
            .unwrap_or(0);
        let mau_not = format!("{}", mau_idx + 1);

        let nhip_dat = self.nhip_hien_tai;

        let ds_muc_tieu = if khuong_idx == 0 { &mut self.ds_not_k1 } else { &mut self.ds_not_k2 };

        // Kiểm tra nếu trùng đúng nốt đó ở đúng nhịp đó thì xóa đi (bật/tắt nốt), chưa có thì thêm mới
        if
            let Some(pos) = ds_muc_tieu
                .iter()
                .position(|n| n.id_not == so_phim && (n.nhip_dich - nhip_dat).abs() < 0.01)
        {
            ds_muc_tieu.remove(pos);
        } else {
            ds_muc_tieu.push(NotNhacJson {
                id_not: so_phim,
                mau_not,
                nhip_dich: nhip_dat,
                nhip_giu,
                chuyen_not: 0,
            });
        }

        // Nếu bật "Tự tăng nhịp khi bấm đàn" -> Nhảy tới nhịp tiếp theo
        let tu_dong_tien = self.tu_dong_tien_check
            .as_ref()
            .map(|c| c.is_pressed())
            .unwrap_or(false);
        if tu_dong_tien {
            let buoc = self.lay_buoc_nhip();
            self.dat_nhip_hien_tai(self.nhip_hien_tai + buoc);
        }

        self.ve_lai_sheet();
    }

    /// Vẽ lại toàn bộ nốt trên 2 khuông (Tự động chạy hàm tiền xử lý chèn vạch chuyển miền ID 100)
    #[func]
    pub fn ve_lai_sheet(&mut self) {
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
        let toc_do = self.quan_ly_sheet
            .as_ref()
            .map(|q| q.bind().get_toc_do())
            .unwrap_or(80.0);

        // Chạy tiền xử lý để tự động sắp xếp và chèn vạch chuyển miền (ID 100)
        let (mien_k1_moi, ds_k1_da_xu_ly) = tien_xu_ly_khuong(
            self.ds_not_k1.clone(),
            mien_k1,
            false,
            0.0,
            f32::MAX
        );
        let (mien_k2_moi, ds_k2_da_xu_ly) = tien_xu_ly_khuong(
            self.ds_not_k2.clone(),
            mien_k2,
            false,
            0.0,
            f32::MAX
        );

        if let Some(mut k1) = self.khuong_1.clone() {
            let mut k_bind = k1.bind_mut();
            k_bind.xoa_tat_ca_not();
            k_bind.nhip_moi_o = nhip_moi_o;
            let mut mien_ht = mien_k1_moi;
            for not in ds_k1_da_xu_ly {
                k_bind.tao_not(
                    not.id_not,
                    not.mau_not,
                    not.nhip_dich,
                    not.nhip_giu,
                    not.chuyen_not,
                    mien_ht,
                    toc_do
                );
                if not.id_not == 100 {
                    mien_ht += not.chuyen_not;
                }
            }
            k_bind.cap_nhat_vi_tri_soan(self.nhip_hien_tai, toc_do);
        }

        if let Some(mut k2) = self.khuong_2.clone() {
            let mut k_bind = k2.bind_mut();
            k_bind.xoa_tat_ca_not();
            k_bind.nhip_moi_o = nhip_moi_o;
            let mut mien_ht = mien_k2_moi;
            for not in ds_k2_da_xu_ly {
                k_bind.tao_not(
                    not.id_not,
                    not.mau_not,
                    not.nhip_dich,
                    not.nhip_giu,
                    not.chuyen_not,
                    mien_ht,
                    toc_do
                );
                if not.id_not == 100 {
                    mien_ht += not.chuyen_not;
                }
            }
            k_bind.cap_nhat_vi_tri_soan(self.nhip_hien_tai, toc_do);
        }
    }

    /// Cập nhật vị trí nhịp hiện tại và di chuyển khuông nhạc
    fn dat_nhip_hien_tai(&mut self, nhip_moi: f32) {
        // Cho phép lùi về tối đa là NHIP_CHO_DAU (ví dụ -2.0) thay vì 0.0
        self.nhip_hien_tai = nhip_moi.max(NHIP_CHO_DAU);

        if let Some(mut label) = self.nhip_label.clone() {
            label.set_text(&format!("Nhịp: {:.2}", self.nhip_hien_tai));
        }

        if let Some(mut slider) = self.thanh_tua.clone() {
            if (self.nhip_hien_tai as f64) > slider.get_max() {
                slider.set_max((self.nhip_hien_tai + 50.0) as f64);
            }
            slider.set_value_no_signal(self.nhip_hien_tai as f64);
        }

        let toc_do = self.quan_ly_sheet
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

    #[func]
    pub fn khi_tua_nhip(&mut self, gia_tri: f64) {
        self.dat_nhip_hien_tai(gia_tri as f32);
    }

    #[func]
    pub fn khi_doi_thong_so_khuong(&mut self, _gia_tri: f64) {
        self.ve_lai_sheet();
    }

    #[func]
    pub fn khi_doi_loai_nhip(&mut self, _index: i64) {
        self.ve_lai_sheet();
    }

    #[func]
    pub fn lui_nhip(&mut self) {
        let buoc = self.lay_buoc_nhip();
        self.dat_nhip_hien_tai(self.nhip_hien_tai - buoc);
    }

    #[func]
    pub fn tien_nhip(&mut self) {
        let buoc = self.lay_buoc_nhip();
        self.dat_nhip_hien_tai(self.nhip_hien_tai + buoc);
    }

    /// Xóa toàn bộ nốt tại vị trí nhịp hiện tại của khuông đang chọn
    #[func]
    pub fn xoa_not_tai_nhip(&mut self) {
        let khuong_idx = self.khuong_chon_opt
            .as_ref()
            .map(|o| o.get_selected())
            .unwrap_or(0);
        let nhip_ht = self.nhip_hien_tai;

        if khuong_idx == 0 {
            self.ds_not_k1.retain(|n| (n.nhip_dich - nhip_ht).abs() >= 0.01);
        } else {
            self.ds_not_k2.retain(|n| (n.nhip_dich - nhip_ht).abs() >= 0.01);
        }
        self.ve_lai_sheet();
    }

    /// Bật / Tắt chế độ Nghe thử (Preview)
    #[func]
    pub fn bat_tat_nghe_thu(&mut self) {
        self.dang_nghe_thu = !self.dang_nghe_thu;

        // Nếu bấm Nghe thử khi đang ở đầu bài (<= 0.0) -> Lùi về -2.0 để nghe được nốt ở 0.0
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

    /// Lưu dữ liệu ra file JSON khi chọn đường dẫn xong
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

        // Chạy tiền xử lý lần cuối để lưu cả danh sách đã sắp xếp chuẩn xác
        let (mien_not_bat_dau_k1, khuong_1) = tien_xu_ly_khuong(
            self.ds_not_k1.clone(),
            mien_k1,
            false,
            0.0,
            f32::MAX
        );
        let (mien_not_bat_dau_k2, khuong_2) = tien_xu_ly_khuong(
            self.ds_not_k2.clone(),
            mien_k2,
            false,
            0.0,
            f32::MAX
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

    /// Đọc file JSON có sẵn lên trình tạo Sheet để sửa tiếp
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
            
            // SỬ DỤNG set_value_no_signal ĐỂ KHÔNG KÍCH HOẠT TÍN HIỆU NGƯỢC LẠI TAOSHEET
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

            // Lọc bỏ các vạch ID 100 tự động cũ để lấy danh sách nốt gốc sạch sẽ
            self.ds_not_k1 = bai_hat
                .khuong_1
                .into_iter()
                .filter(|n| n.id_not != 100)
                .collect();
            self.ds_not_k2 = bai_hat
                .khuong_2
                .into_iter()
                .filter(|n| n.id_not != 100)
                .collect();

            self.dat_nhip_hien_tai(0.0);
            self.ve_lai_sheet();
        }
    }
    /// Tự động tính miền K1, K2 tại nhịp hiện tại (Reset về miền gốc khi tua/phát lại từ đầu)
    pub fn cap_nhat_mien_theo_vi_tri_nhip(&mut self) {
        let mien_k1_goc = self.mien_k1_spin
            .as_ref()
            .map(|s| s.get_value() as i32)
            .unwrap_or(36);
        let mien_k2_goc = self.mien_k2_spin
            .as_ref()
            .map(|s| s.get_value() as i32)
            .unwrap_or(24);

        let (mut mien_k1_ht, ds_k1_da_xu_ly) = tien_xu_ly_khuong(
            self.ds_not_k1.clone(),
            mien_k1_goc,
            false,
            0.0,
            f32::MAX
        );
        for not in ds_k1_da_xu_ly {
            if not.id_not == 100 && not.nhip_dich <= self.nhip_hien_tai + 0.01 {
                mien_k1_ht += not.chuyen_not;
            }
        }

        let (mut mien_k2_ht, ds_k2_da_xu_ly) = tien_xu_ly_khuong(
            self.ds_not_k2.clone(),
            mien_k2_goc,
            false,
            0.0,
            f32::MAX
        );
        for not in ds_k2_da_xu_ly {
            if not.id_not == 100 && not.nhip_dich <= self.nhip_hien_tai + 0.01 {
                mien_k2_ht += not.chuyen_not;
            }
        }

        if let Some(mut qls) = self.quan_ly_sheet.clone() {
            qls.bind_mut().dat_mien_cho_tao_sheet(mien_k1_ht, mien_k2_ht);
        }
    }
}

#[godot_api]
impl IControl for TaoSheet {
    fn ready(&mut self) {
        self.ban_phim_midi = self.base().try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano");
        self.quan_ly_sheet = self.base().try_get_node_as::<QuanLySheet>("QuanLySheet");
        self.khuong_1 = self.base().try_get_node_as::<KhuongNhac>("QuanLySheet/K1");
        self.khuong_2 = self.base().try_get_node_as::<KhuongNhac>("QuanLySheet/K2");
        // Tắt process mặc định của QuanLySheet để TaoSheet làm chủ hoàn toàn thời gian
        if let Some(mut qls) = self.quan_ly_sheet.clone() {
            qls.bind_mut().bat_tat_cho_phep_xoa_not(false);
            qls.bind_mut().set_hien_thi_thoi_gian(false);
            qls.set_process(false);
        }

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
        self.nhip_giu_spin = self
            .base()
            .try_get_node_as::<SpinBox>("ThanhCongCuSoan/VBox/HangCongCu/NhipGiu_Spin");
        self.mau_not_opt = self
            .base()
            .try_get_node_as::<OptionButton>("ThanhCongCuSoan/VBox/HangCongCu/MauNot_Option");
        self.tu_dong_tien_check = self
            .base()
            .try_get_node_as::<CheckButton>("ThanhCongCuSoan/VBox/HangCongCu/TuDongTien_Check");
        self.nghe_thu_btn = self
            .base()
            .try_get_node_as::<Button>("ThanhCongCuSoan/VBox/HangCongCu/NgheThu_Btn");
        self.hop_thoai_luu = self.base().try_get_node_as::<FileDialog>("HopThoaiLuuFile");
        self.hop_thoai_mo = self.base().try_get_node_as::<FileDialog>("HopThoaiMoFile");
        //signal
        // Kết nối sự kiện bàn phím MIDI
        let call_midi = self.base().callable("khi_bam_phim_midi");
        if let Some(mut midi) = self.ban_phim_midi.clone() {
            midi.connect("phim_vua_duoc_bam", &call_midi);
        }

        // Kết nối Thanh tua nhịp & thông số khuông
        let call_tua = self.base().callable("khi_tua_nhip");
        if let Some(mut slider) = self.thanh_tua.clone() {
            slider.set_min(NHIP_CHO_DAU as f64);
            slider.connect("value_changed", &call_tua);
        }
        let call_mien = self.base().callable("khi_doi_thong_so_khuong");
        if let Some(mut s1) = self.mien_k1_spin.clone() {
            s1.connect("value_changed", &call_mien);
        }
        if let Some(mut s2) = self.mien_k2_spin.clone() {
            s2.connect("value_changed", &call_mien);
        }

        let call_loai_nhip = self.base().callable("khi_doi_loai_nhip");
        if let Some(mut opt) = self.loai_nhip_opt.clone() {
            opt.connect("item_selected", &call_loai_nhip);
        }

        // Kết nối các nút bấm
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

        let call_xoa = self.base().callable("xoa_not_tai_nhip");
        if
            let Some(mut btn) = self
                .base()
                .try_get_node_as::<Button>("ThanhCongCuSoan/VBox/HangCongCu/XoaNot_Btn")
        {
            btn.connect("pressed", &call_xoa);
        }

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

        self.ve_lai_sheet();
    }

    fn process(&mut self, delta: f64) {
        if !self.dang_nghe_thu {
            return;
        }

        let bpm = self.bpm_spin
            .as_ref()
            .map(|s| s.get_value() as f32)
            .unwrap_or(120.0);
        let nhip_truoc = self.nhip_hien_tai;
        let nhip_moi = nhip_truoc + ((delta as f32) * bpm) / 60.0;

        // Phát âm thanh SF2 tự động cho các nốt vừa chạm vạch đích trong khoảng (nhip_truoc -> nhip_moi]
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
    }
}
