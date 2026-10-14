use godot::builtin::{ GString };
use godot::classes::{ Button, Control, FileAccess, IControl, Label };
use godot::global::godot_print;
use godot::obj::{ Gd, WithBaseField };
use godot::{ obj::Base, prelude::{ GodotClass, ToGodot, godot_api } };
use serde::Deserialize;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

use crate::ban_phim_midi::BanPhimMidi;
use crate::check_ten_not::chuyen_id_thanh_ten;
use crate::hop_am::HopAm;
use crate::sf2_read;

#[derive(Deserialize, Debug)]
pub struct BaiNgheJson {
    pub ten_bai_hoc: String,
    pub nguoi_tao: String,
    pub danh_sach_bai: Vec<String>,
}

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct HocNgheNot {
    #[base]
    base: Base<Control>,
    #[export]
    duong_dan_bai_hoc: GString,

    ten_bai_hoc: GString,
    nguoi_tao: GString,
    cac_bai_nghe: Vec<String>,

    dang_o_bai_so: usize,

    muc_tieu_hien_tai: Vec<Vec<i32>>,
    vi_tri_dang_danh: usize,
    phim_da_doan_dung: Vec<i32>,

    diem: i32,
    so_lan_dung: i32,
    so_lan_sai: i32,
    so_lan_nghe: i32,
    so_lan_doan: i32,

    nghe_lai_button: Option<Gd<Button>>,
    ban_phim_midi: Option<Gd<BanPhimMidi>>,
}

#[godot_api]
impl HocNgheNot {
    #[signal]
    fn xong_het_bai();

    #[func]
    pub fn get_diem(&self) -> i32 {
        self.diem
    }
    #[func]
    pub fn get_ten_bai_hoc(&self) -> GString {
        self.ten_bai_hoc.clone()
    }
    #[func]
    pub fn get_nguoi_tao(&self) -> GString {
        self.nguoi_tao.clone()
    }
    #[func]
    pub fn get_so_lan_nghe(&self) -> i32 {
        self.so_lan_nghe
    }
    #[func]
    pub fn get_so_lan_sai(&self) -> i32 {
        self.so_lan_sai
    }
    #[func]
    pub fn get_so_lan_doan(&self) -> i32 {
        self.so_lan_doan
    }
    #[func]
    pub fn get_so_lan_dung(&self) -> i32 {
        self.so_lan_dung
    }

    #[func]
    pub fn chuyen_bai(&mut self) {
        self.dang_o_bai_so += 1;

        if self.dang_o_bai_so >= self.cac_bai_nghe.len() {
            // Sửa lỗi method not found bằng emit_signal
            self.base_mut().emit_signal("xong_het_bai", &[]);
            return;
        }
        self.chay_bai_nghe();
    }

    #[func]
    pub fn chay_bai_nghe(&mut self) {
        if self.dang_o_bai_so >= self.cac_bai_nghe.len() {
            return;
        }

        let chuoi_cau_hoi = self.cac_bai_nghe[self.dang_o_bai_so].clone();

        // Quét mảng mục tiêu (Cắt theo dấu _)
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
        self.so_lan_nghe = 0;
        self.phim_da_doan_dung.clear();

        self.base_mut().call("xoa_cac_not_doan", &[]);

        let muc_tieu_hien_tai = self.muc_tieu_hien_tai.clone();
        for hop_am in muc_tieu_hien_tai {
            // Ép thành chuỗi "36_40_43" để truyền qua GDScript
            let chuoi_id = hop_am
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join("_");
            self.base_mut().call("them_not_doan", &[chuoi_id.to_variant()]);
        }

        self.phat_chuoi_am_thanh();
    }

    #[func]
    pub fn phat_chuoi_am_thanh(&mut self) {
        self.so_lan_nghe += 1;

        if self.so_lan_nghe > 1 {
            self.diem -= 5;
            if self.diem < 0 {
                self.diem = 0;
            }
        }

        let chuoi_can_phat: Vec<Vec<i32>> = self.muc_tieu_hien_tai.clone();

        thread::spawn(move || {
            for hop_am in chuoi_can_phat {
                // Bật tất cả nốt cùng lúc
                for &id_not in &hop_am {
                    let nut_midi = id_not + 24;
                    sf2_read::bat_not_nhac(nut_midi, 100);
                }

                thread::sleep(Duration::from_millis(600));

                // Tắt tất cả nốt cùng lúc
                for &id_not in &hop_am {
                    let nut_midi = id_not + 24;
                    sf2_read::tat_not_nhac(nut_midi);
                }

                thread::sleep(Duration::from_millis(200));
            }
        });
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

        // 1. Lưu phím vừa bấm vào danh sách đang giữ
        if !self.phim_da_doan_dung.contains(&so_phim) {
            self.phim_da_doan_dung.push(so_phim);
        }

        // 2. Nếu phím vừa bấm không nằm trong hợp âm -> Báo lỗi sai và trừ điểm
        if !muc_tieu_hop_am.contains(&so_phim) {
            self.so_lan_sai += 1;
            self.diem -= 10;
            if self.diem < 0 { self.diem = 0; }
            
            let vi_tri_dang_danh = self.vi_tri_dang_danh as i32;
            self.base_mut().call("hien_thi_not_doan_sai", &[vi_tri_dang_danh.to_variant()]);
        }
        
        self.so_lan_doan += 1;
        self.cap_nhat_diem();

        // 3. KIỂM TRA ĐIỀU KIỆN THẮNG: 
        // - Số lượng phím đang giữ BẰNG số lượng phím mục tiêu (không bị bấm thừa)
        // - VÀ tất cả các phím mục tiêu đều nằm trong danh sách đang giữ
        let hoan_thanh = self.phim_da_doan_dung.len() == muc_tieu_hop_am.len() 
            && muc_tieu_hop_am.iter().all(|phim| self.phim_da_doan_dung.contains(phim));

        if hoan_thanh {
            self.hoan_thanh_muc_tieu();
        }
    }

    #[func]
    pub fn xu_ly_phim_nha(&mut self, so_phim: i32) {
        if self.vi_tri_dang_danh >= self.muc_tieu_hien_tai.len() { return; }
        
        // Khi nhả phím ra, lập tức xóa nó khỏi danh sách đang giữ
        self.phim_da_doan_dung.retain(|&x| x != so_phim);

        let muc_tieu_hop_am = self.muc_tieu_hien_tai[self.vi_tri_dang_danh].clone();
        
        // Nếu người chơi đang giữ nốt (Đúng + Sai), việc nhả nốt Sai ra có thể làm hợp âm trở thành ĐÚNG toàn bộ
        let hoan_thanh = self.phim_da_doan_dung.len() == muc_tieu_hop_am.len() 
            && muc_tieu_hop_am.iter().all(|phim| self.phim_da_doan_dung.contains(phim));

        if hoan_thanh {
            self.hoan_thanh_muc_tieu();
        }
    }

    fn hoan_thanh_muc_tieu(&mut self) {
        let muc_tieu_hop_am = self.muc_tieu_hien_tai[self.vi_tri_dang_danh].clone();
        
        self.so_lan_dung += 1;
        self.diem += 100 * muc_tieu_hop_am.len() as i32; // Cộng điểm dựa trên số lượng nốt

        let vi_tri_dang_danh = self.vi_tri_dang_danh as i32;
        self.base_mut().call("hien_thi_not_doan_dung", &[vi_tri_dang_danh.to_variant()]);

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
        }
        self.cap_nhat_diem();
    }

    #[func]
    pub fn chuyen_doi_id_thanh_ten(&self, chuoi_id: GString) -> GString {
        let chuoi_rust = chuoi_id.to_string();

        let mut tap_hop_phim = HashSet::new();
        let mut danh_sach_id = Vec::new();

        for id_str in chuoi_rust.split('_') {
            if let Ok(id) = id_str.parse::<i32>() {
                tap_hop_phim.insert(id);
                danh_sach_id.push(id);
            }
        }

        // ƯU TIÊN 1: Nếu từ 2 phím trở lên, thử dịch thành Hợp âm
        if tap_hop_phim.len() >= 2 {
            let ten_hop_am = HopAm::kiem_tra(&tap_hop_phim);
            if !ten_hop_am.is_empty() {
                // Trả về Hợp âm nguyên bản, không làm nhỏ số
                let chuoi_bbcode = format!("[center]{}[/center]", ten_hop_am);
                return GString::from(chuoi_bbcode);
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

        let chuoi_bbcode: String = format!("[center]{}[/center]", ket_qua.join(" "));
        GString::from(chuoi_bbcode)
    }

    #[func]
    pub fn lay_thong_tin_bai_nghe(&mut self, duong_dan: GString) {
        let json_string = FileAccess::get_file_as_string(&duong_dan);
        if let Ok(du_lieu) = serde_json::from_str::<BaiNgheJson>(&json_string.to_string()) {
            self.ten_bai_hoc = GString::from(du_lieu.ten_bai_hoc);
            self.nguoi_tao = GString::from(du_lieu.nguoi_tao);
            self.cac_bai_nghe = du_lieu.danh_sach_bai;
        } else {
            godot_print!("Lỗi JSON bài nghe: {}", duong_dan);
        }
    }
    fn sua_ghi_chu(&mut self, cac_ghi_chu: Vec<String>) {
        let mut noi_dung_ghi_chu = String::new();
        if self.dang_o_bai_so < cac_ghi_chu.len() {
            noi_dung_ghi_chu = cac_ghi_chu[self.dang_o_bai_so].clone();
        }
        if let Some(mut label) = self.base().try_get_node_as::<Label>("ghi_chu") {
            label.set_text(&noi_dung_ghi_chu);
        }
    } 
    #[func]
    pub fn khoi_dong_bai_hoc(&mut self) {
        self.diem = 0;
        self.so_lan_doan = 0;
        self.so_lan_dung = 0;
        self.so_lan_sai = 0;

        let path = self.duong_dan_bai_hoc.clone();
        if !path.is_empty() {
            self.lay_thong_tin_bai_nghe(path);
            if let Some(mut label) = self.base().try_get_node_as::<Label>("ten_bai_hoc") {
                label.set_text(&format!("{}", self.ten_bai_hoc));
            }
            self.chay_bai_nghe();
        } else {
            godot_print!("Cảnh báo: Đường dẫn bài học đang trống!");
        }
    }
}

#[godot_api]
impl IControl for HocNgheNot {
    fn ready(&mut self) {
        self.dang_o_bai_so = 0;
        self.ban_phim_midi = self.base().try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano");
        self.nghe_lai_button = self.base().try_get_node_as::<Button>("nghe_lai");
    }
}
