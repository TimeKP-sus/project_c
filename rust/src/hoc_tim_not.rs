use godot::builtin::{Color, GString};
use godot::classes::{Control, FileAccess, IControl, Label};
use godot::global::godot_print;
use godot::obj::{Gd, WithBaseField};
use godot::{obj::Base, prelude::{GodotClass, ToGodot, godot_api}};
use serde::Deserialize;
use std::collections::HashSet;

use crate::ban_phim_midi::BanPhimMidi;
use crate::check_ten_not::chuyen_id_thanh_ten;
use crate::hop_am::HopAm;

#[derive(Deserialize, Debug)]
pub struct BaiTimJson {
    pub ten_bai_hoc: String,
    pub nguoi_tao: String,
    pub danh_sach_bai: Vec<String>, 
    pub cac_ghi_chu: Option<Vec<String>>,
}

#[derive(GodotClass)]
#[class(init, base=Control)] 
pub struct HocTimNot {
    #[base]
    base: Base<Control>,
    #[export]
    duong_dan_bai_hoc: GString,

    #[export]
    pub cho_phep_to_mau_goi_y: bool, // Biến bật/tắt tính năng gợi ý

    ten_bai_hoc: GString,
    nguoi_tao: GString,
    cac_bai_tim: Vec<String>,
    cac_ghi_chu: Vec<String>,

    dang_o_bai_so: usize,
    
    muc_tieu_hien_tai: Vec<Vec<i32>>,
    vi_tri_dang_danh: usize,
    phim_da_doan_dung: Vec<i32>,

    diem: i32,
    so_lan_dung: i32,
    so_lan_sai: i32,

    ban_phim_midi: Option<Gd<BanPhimMidi>>,
}

#[godot_api]
impl HocTimNot {
    #[signal]
    fn xong_het_bai();

    #[func]
    pub fn get_diem(&self) -> i32 { self.diem }
    #[func]
    pub fn get_ten_bai_hoc(&self) -> GString { self.ten_bai_hoc.clone() }
    #[func]
    pub fn get_nguoi_tao(&self) -> GString { self.nguoi_tao.clone() }

    // ================= TÍNH NĂNG TÔ MÀU GỢI Ý =================
    fn lay_mau_sac_cho_phim(so_phim: i32) -> Color {
        let vi_tri_not = so_phim % 12;
        match vi_tri_not {
            0 => Color::from_rgba(1.0, 0.2, 0.2, 0.9),   // C  - Đỏ
            1 => Color::from_rgba(1.0, 0.4, 0.4, 0.9),   // C# - Đỏ nhạt
            2 => Color::from_rgba(1.0, 0.6, 0.1, 0.9),   // D  - Cam
            3 => Color::from_rgba(1.0, 0.7, 0.4, 0.9),   // D# - Cam nhạt
            4 => Color::from_rgba(1.0, 0.9, 0.1, 0.9),   // E  - Vàng
            5 => Color::from_rgba(0.2, 0.8, 0.2, 0.9),   // F  - Xanh lá
            6 => Color::from_rgba(0.4, 0.9, 0.5, 0.9),   // F# - Xanh ngọc
            7 => Color::from_rgba(0.1, 0.6, 1.0, 0.9),   // G  - Xanh dương
            8 => Color::from_rgba(0.4, 0.7, 1.0, 0.9),   // G# - Xanh lơ
            9 => Color::from_rgba(0.6, 0.2, 1.0, 0.9),   // A  - Tím
            10 => Color::from_rgba(0.8, 0.4, 1.0, 0.9),  // A# - Tím nhạt
            11 => Color::from_rgba(1.0, 0.2, 0.7, 0.9),  // B  - Hồng
            _ => Color::from_rgba(1.0, 1.0, 1.0, 0.9),
        }
    }
    #[func]
    pub fn set_cho_phep_to_mau(&mut self, trang_thai: bool) {
        self.cho_phep_to_mau_goi_y = trang_thai;
        if trang_thai {
            self.to_mau_muc_tieu_hien_tai();
        } else {
            self.xoa_mau_tat_ca_muc_tieu();
        }
    }

    #[func]
    pub fn xoa_mau_tat_ca_muc_tieu(&mut self) {
        if let Some(ban_phim) = &mut self.ban_phim_midi {
            for hop_am in &self.muc_tieu_hien_tai {
                for &id_phim in hop_am {
                    ban_phim.bind_mut().dung_to_phim(id_phim);
                }
            }
        }
    }

    #[func]
    pub fn to_mau_muc_tieu_hien_tai(&mut self) {
        if !self.cho_phep_to_mau_goi_y { return; }
        if self.vi_tri_dang_danh >= self.muc_tieu_hien_tai.len() { return; }
        
        if self.ban_phim_midi.is_none() {
            if let Some(ban_phim) = self.base().try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano") {
                self.ban_phim_midi = Some(ban_phim);
            } else {
                godot_print!("LỖI TÔ MÀU: Không tìm thấy 'MIDI/BanPhimPiano'. Kiểm tra lại tên Node!");
                return;
            }
        }

        let muc_tieu = self.muc_tieu_hien_tai[self.vi_tri_dang_danh].clone();
        if let Some(ban_phim) = &mut self.ban_phim_midi {
            for id_phim in muc_tieu {
                if !self.phim_da_doan_dung.contains(&id_phim) {
                    let mau: Color = Self::lay_mau_sac_cho_phim(id_phim);
                    ban_phim.bind_mut().to_phim(id_phim, mau);
                }
            }
        }
    }

    // ==========================================================

    #[func]
    pub fn chuyen_bai(&mut self) {
        self.dang_o_bai_so += 1;
        
        if self.dang_o_bai_so >= self.cac_bai_tim.len() {
            self.base_mut().emit_signal("xong_het_bai", &[]);
            return;
        }
        self.chay_bai_tim();
    }

    #[func]
    pub fn chay_bai_tim(&mut self) {
        if self.dang_o_bai_so >= self.cac_bai_tim.len() { return; }

        let chuoi_cau_hoi = self.cac_bai_tim[self.dang_o_bai_so].clone();
        
        self.xoa_mau_tat_ca_muc_tieu();
        self.muc_tieu_hien_tai.clear();
        
        for cum_str in chuoi_cau_hoi.split_whitespace() {
            let mut hop_am = Vec::new();
            for id_str in cum_str.split('_') {
                if let Ok(id) = id_str.parse::<i32>() {
                    hop_am.push(id);
                }
            }
            if !hop_am.is_empty() {
                self.muc_tieu_hien_tai.push(hop_am);
            }
        }
        
        self.vi_tri_dang_danh = 0;
        self.phim_da_doan_dung.clear();
        self.base_mut().call("xoa_cac_not_tim", &[]);
        
        let mut noi_dung_ghi_chu = String::new();
        if self.dang_o_bai_so < self.cac_ghi_chu.len() {
            noi_dung_ghi_chu = self.cac_ghi_chu[self.dang_o_bai_so].clone();
        }
        if let Some(mut label) = self.base().try_get_node_as::<Label>("ghi_chu") {
            label.set_text(&noi_dung_ghi_chu);
        }
        
        let muc_tieu_hien_tai = self.muc_tieu_hien_tai.clone();
        for hop_am in muc_tieu_hien_tai {
            let chuoi_id = hop_am.iter().map(|id| id.to_string()).collect::<Vec<String>>().join("_");
            self.base_mut().call("them_not_tim", &[chuoi_id.to_variant()]);
        }
        self.base_mut().call_deferred("to_mau_muc_tieu_hien_tai", &[]);
    }

    #[func]
    pub fn cap_nhat_diem(&mut self) {
        if let Some(mut label) = self.base().try_get_node_as::<Label>("diem") {
            label.set_text(&format!("{}", self.diem));
        }
    }

    #[func]
    pub fn kiem_tra_phim(&mut self, so_phim: i32) {
        if self.vi_tri_dang_danh >= self.muc_tieu_hien_tai.len() { return; }

        let muc_tieu_hop_am = self.muc_tieu_hien_tai[self.vi_tri_dang_danh].clone();

        // 1. Lưu phím vừa bấm vào danh sách đang giữ (Dù đúng hay sai)
        if !self.phim_da_doan_dung.contains(&so_phim) {
            self.phim_da_doan_dung.push(so_phim);
        }

        // 2. Xử lý phản hồi (Đúng/Sai)
        if muc_tieu_hop_am.contains(&so_phim) {
            // Tắt màu gọi ý của nốt vừa đánh ĐÚNG
            if self.cho_phep_to_mau_goi_y {
                if let Some(ban_phim) = &mut self.ban_phim_midi {
                    ban_phim.bind_mut().dung_to_phim(so_phim);
                }
            }
        } else {
            // Bấm nốt SAI
            self.so_lan_sai += 1;
            self.diem -= 5; // Trừ 5 điểm cho Tìm Nốt
            if self.diem < 0 { self.diem = 0; }
            let vi_tri_dang_danh = self.vi_tri_dang_danh as i32;
            self.base_mut().call("hien_thi_not_tim_sai", &[vi_tri_dang_danh.to_variant()]);
        }
        
        self.cap_nhat_diem();

        // 3. KIỂM TRA ĐIỀU KIỆN THẮNG
        let hoan_thanh = self.phim_da_doan_dung.len() == muc_tieu_hop_am.len() 
            && muc_tieu_hop_am.iter().all(|phim| self.phim_da_doan_dung.contains(phim));

        if hoan_thanh {
            self.hoan_thanh_muc_tieu();
        }
    }

    #[func]
    pub fn xu_ly_phim_nha(&mut self, so_phim: i32) {
        if self.vi_tri_dang_danh >= self.muc_tieu_hien_tai.len() { return; }
        
        // 1. Xóa phím vừa nhả khỏi danh sách đang giữ
        self.phim_da_doan_dung.retain(|&x| x != so_phim);

        let muc_tieu_hop_am = self.muc_tieu_hien_tai[self.vi_tri_dang_danh].clone();

        // 2. Nếu nhả ra 1 phím nằm trong mục tiêu, tô màu lại nó để gợi ý
        if muc_tieu_hop_am.contains(&so_phim) {
            if self.cho_phep_to_mau_goi_y {
                let mau: Color = Self::lay_mau_sac_cho_phim(so_phim);
                if let Some(ban_phim) = &mut self.ban_phim_midi {
                    ban_phim.bind_mut().to_phim(so_phim, mau);
                }
            }
        }

        // 3. Kiểm tra lại điều kiện thắng (Phòng trường hợp đang giữ nốt sai rồi nhả nốt sai ra)
        let hoan_thanh = self.phim_da_doan_dung.len() == muc_tieu_hop_am.len() 
            && muc_tieu_hop_am.iter().all(|phim| self.phim_da_doan_dung.contains(phim));

        if hoan_thanh {
            self.hoan_thanh_muc_tieu();
        }
    }

    // Hàm phụ trợ nội bộ xử lý chuyển bài/chuyển hợp âm
    fn hoan_thanh_muc_tieu(&mut self) {
        let muc_tieu_hop_am = self.muc_tieu_hien_tai[self.vi_tri_dang_danh].clone();
        
        self.so_lan_dung += 1;
        self.diem += 50 * muc_tieu_hop_am.len() as i32; // 50 điểm/nốt cho Tìm Nốt

        let vi_tri_dang_danh = self.vi_tri_dang_danh as i32;
        self.base_mut().call("hien_thi_not_tim_dung", &[vi_tri_dang_danh.to_variant()]);

        self.vi_tri_dang_danh += 1;
        self.phim_da_doan_dung.clear(); // Xóa trắng để chuẩn bị cho nốt/hợp âm tiếp theo

        if self.vi_tri_dang_danh >= self.muc_tieu_hien_tai.len() {
            let self_clone = self.base_mut();
            let callable = self_clone.callable("chuyen_bai");
            if let Some(mut tree) = self_clone.get_tree() {
                if let Some(mut timer) = tree.create_timer(1.0) {
                    timer.connect("timeout", &callable);
                }
            }
        } else {
            // Chuyển sang cụm nốt tiếp theo, bật màu gợi ý
            self.to_mau_muc_tieu_hien_tai();
        }
        self.cap_nhat_diem();
    }

    #[func]
    pub fn chuyen_doi_id_thanh_ten(&self, chuoi_id: GString) -> GString {
        // Giữ nguyên như cũ...
        let chuoi_rust = chuoi_id.to_string();
        let mut tap_hop_phim = HashSet::new();
        let mut danh_sach_id = Vec::new();
        for id_str in chuoi_rust.split('_') {
            if let Ok(id) = id_str.parse::<i32>() {
                tap_hop_phim.insert(id);
                danh_sach_id.push(id);
            }
        }
        if tap_hop_phim.len() >= 2 {
            let ten_hop_am = HopAm::kiem_tra(&tap_hop_phim);
            if !ten_hop_am.is_empty() {
                return GString::from(format!("[center]{}[/center]", ten_hop_am));
            }
        }
        let mut ket_qua = Vec::new();
        for id in danh_sach_id {
            let ten = chuyen_id_thanh_ten(id, false);
            if !ten.is_empty() {
                let len = ten.len();
                let phan_chu = &ten[..len - 1];
                let phan_so = &ten[len - 1..];
                ket_qua.push(format!("{}[font_size=80]{}[/font_size]", phan_chu, phan_so));
            }
        }
        GString::from(format!("[center]{}[/center]", ket_qua.join(" ")))
    }

    #[func]
    pub fn lay_thong_tin_bai_hoc(&mut self, duong_dan: GString) {
        let json_string = FileAccess::get_file_as_string(&duong_dan);
        if let Ok(du_lieu) = serde_json::from_str::<BaiTimJson>(&json_string.to_string()) {
            self.ten_bai_hoc = GString::from(du_lieu.ten_bai_hoc);
            self.nguoi_tao = GString::from(du_lieu.nguoi_tao);
            self.cac_bai_tim = du_lieu.danh_sach_bai;
            if let Some(ghi_chu) = du_lieu.cac_ghi_chu {
                self.cac_ghi_chu = ghi_chu;
            } else {
                self.cac_ghi_chu = Vec::new();
            }
        }
    }

    #[func]
    pub fn khoi_dong_bai_hoc(&mut self) {
        self.diem = 0;
        self.so_lan_dung = 0;
        self.so_lan_sai = 0;

        let path = self.duong_dan_bai_hoc.clone();
        if !path.is_empty() {
            self.lay_thong_tin_bai_hoc(path);
            if let Some(mut label) = self.base().try_get_node_as::<Label>("ten_bai_hoc") {
                label.set_text(&format!("{}", self.ten_bai_hoc));
            }
            self.chay_bai_tim();
        }
    }
}

#[godot_api]
impl IControl for HocTimNot {
    fn ready(&mut self) {
        self.dang_o_bai_so = 0;
        self.cho_phep_to_mau_goi_y = true; // Mặc định bật tô màu khi mới vào
        
        if let Some(mut ban_phim) = self.base().try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano") {
            // Lắng nghe tín hiệu nhả phím để tô màu lại nếu lỡ tay
            let callable = self.base().callable("xu_ly_phim_nha");
            ban_phim.connect("phim_vua_duoc_nha", &callable);
            
            self.ban_phim_midi = Some(ban_phim);
        }
    }
}