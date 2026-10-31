use godot::classes::{file_access::ModeFlags, DirAccess, FileAccess};
use godot::global::godot_print;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


pub const DUONG_DAN_CAI_DAT: &str = "res://du_lieu_nguoi_dung/cai_dat.json";

// =========================================================================
// 1. DỮ LIỆU NGƯỜI DÙNG & LỊCH SỬ BÀI HÁT (Giữ nguyên 100% các hàm & trường cũ)
// =========================================================================

// Struct chứa thông tin riêng của mỗi bài hát
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThongTinBaiHat {
    pub diem_cao_nhat: i32,
    pub so_lan_choi: i32,
    pub da_hoan_thanh: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuLieuNguoiDung {
    pub ten_nguoi_choi: String,
    #[serde(default)]
    pub do_tre_phim: f32,
    #[serde(default)]
    pub lich_su_bai_hat: HashMap<String, ThongTinBaiHat>,
}

impl DuLieuNguoiDung {
    /// Đọc file JSON của người chơi bằng FileAccess của Godot (hỗ trợ trực tiếp đường dẫn res://)
    /// 
    pub fn doc_thong_tin_nguoi_choi(ten_nguoi_choi: &str) -> Option<Self> {
        let duong_dan: String = format!("res://du_lieu_nguoi_dung/{}.json", ten_nguoi_choi);
        let file_text: String = FileAccess::get_file_as_string(&duong_dan).to_string();
        if file_text.trim().is_empty() {
            return None;
        }
        match serde_json::from_str::<DuLieuNguoiDung>(&file_text) {
            Ok(du_lieu) => Some(du_lieu),
            Err(loi) => {
                godot_print!("Lỗi đọc JSON người dùng ({}): {}", duong_dan, loi);
                None
            }
        }
    }

    /// Lưu thông tin người chơi vào thư mục res://du_lieu_nguoi_dung/{ten_nguoi_choi}.json
    pub fn luu_thong_tin_nguoi_choi(&self) -> bool {
        let _ = DirAccess::make_dir_recursive_absolute("res://du_lieu_nguoi_dung");
        let duong_dan: String = format!("res://du_lieu_nguoi_dung/{}.json", self.ten_nguoi_choi);
        match serde_json::to_string_pretty(self) {
            Ok(noi_dung) => {
                if let Some(mut file) = FileAccess::open(&duong_dan, ModeFlags::WRITE) {
                    file.store_string(&noi_dung);
                    godot_print!("Đã lưu dữ liệu người chơi tại: {}", duong_dan);
                    true
                } else {
                    godot_print!("Lỗi: Không thể mở file để ghi tại {}", duong_dan);
                    false
                }
            }
            Err(loi) => {
                godot_print!("Lỗi đóng gói JSON người chơi: {}", loi);
                false
            }
        }
    }

    pub fn khoi_tao(ten_nguoi_choi: String) -> Self {
        Self {
            ten_nguoi_choi,
            do_tre_phim: 0.0,
            lich_su_bai_hat: HashMap::new(),
        }
    }

    pub fn set_bai_hat(&mut self, ten_bai_hat: String, diem: i32, da_hoan_thanh: bool) {
        let thong_tin: &mut ThongTinBaiHat = self
            .lich_su_bai_hat
            .entry(ten_bai_hat.clone())
            .or_insert(ThongTinBaiHat {
                diem_cao_nhat: diem,
                so_lan_choi: 0,
                da_hoan_thanh,
            });
        thong_tin.so_lan_choi += 1;
        if diem > thong_tin.diem_cao_nhat {
            thong_tin.diem_cao_nhat = diem;
        }
        if da_hoan_thanh {
            thong_tin.da_hoan_thanh = true;
        }
    }

    pub fn get_bai_hat_theo_ten(&self, ten_bai_hat: &str) -> Option<&ThongTinBaiHat> {
        self.lich_su_bai_hat.get(ten_bai_hat)
    }

    pub fn set_do_tre_phim(&mut self, do_tre: f32) {
        self.do_tre_phim = do_tre;
    }

    pub fn get_do_tre_phim(&self) -> f32 {
        self.do_tre_phim
    }
}

// =========================================================================
// 2. CLASS JSON CÀI ĐẶT GAME & ĐỒ HỌA RIÊNG (res://du_lieu_nguoi_dung/cai_dat.json)
// =========================================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaiDatGameJson {
    /// Hệ số tốc độ chơi (0.25 -> 2.5, mặc định: 1.0)
    pub toc_do_choi: f32,
    /// Độ trễ phím đầu vào tính bằng mili-giây (-200.0 -> +200.0, mặc định: 0.0)
    pub do_tre_phim_ms: f32,
    /// Âm lượng tổng (0.0 -> 100.0, mặc định: 80.0)
    pub am_luong_chung: f32,

    // --- Cài đặt đồ họa ---
    pub toan_man_hinh: bool,
    pub bat_vsync: bool,
    /// 0: Không giới hạn, 60, 120, 144, 240
    pub gioi_han_fps: i32,
    /// 0: 1280x720, 1: 1600x900, 2: 1920x1080, 3: 2560x1440
    pub do_phan_giai_idx: i32,
    pub hien_thi_fps: bool,
}

impl Default for CaiDatGameJson {
    fn default() -> Self {
        Self {
            toc_do_choi: 1.0,
            do_tre_phim_ms: 0.0,
            am_luong_chung: 80.0,
            toan_man_hinh: false,
            bat_vsync: true,
            gioi_han_fps: 60,
            do_phan_giai_idx: 2,
            hien_thi_fps: false,
        }
    }
}

impl CaiDatGameJson {
    /// Đọc cài đặt từ `res://du_lieu_nguoi_dung/cai_dat.json` (tự tạo file mặc định nếu chưa có)
    pub fn doc_cai_dat() -> Self {
        let txt = FileAccess::get_file_as_string(DUONG_DAN_CAI_DAT).to_string();
        if !txt.trim().is_empty() {
            if let Ok(cfg) = serde_json::from_str::<CaiDatGameJson>(&txt) {
                return cfg;
            }
        }
        let mac_dinh = Self::default();
        mac_dinh.luu_cai_dat();
        mac_dinh
    }

    /// Lưu cài đặt ngay trong thư mục game `res://du_lieu_nguoi_dung/cai_dat.json`
    pub fn luu_cai_dat(&self) -> bool {
        let _ = DirAccess::make_dir_recursive_absolute("res://du_lieu_nguoi_dung");
        match serde_json::to_string_pretty(self) {
            Ok(noi_dung) => {
                if let Some(mut file) = FileAccess::open(DUONG_DAN_CAI_DAT, ModeFlags::WRITE) {
                    file.store_string(&noi_dung);
                    godot_print!("Đã lưu cài đặt game tại: {}", DUONG_DAN_CAI_DAT);
                    true
                } else {
                    godot_print!("Lỗi: Không thể ghi file cài đặt tại {}", DUONG_DAN_CAI_DAT);
                    false
                }
            }
            Err(loi) => {
                godot_print!("Lỗi đóng gói JSON cài đặt: {}", loi);
                false
            }
        }
    }
    pub fn set_toc_do_choi(&mut self, toc_do: f32) {
        self.toc_do_choi = toc_do.clamp(0.25, 2.5);
    }

    pub fn get_toc_do_choi(&self) -> f32 {
        self.toc_do_choi
    }

    pub fn set_do_tre_phim_ms(&mut self, do_tre_ms: f32) {
        self.do_tre_phim_ms = do_tre_ms.clamp(-200.0, 200.0);
    }

    pub fn get_do_tre_phim_ms(&self) -> f32 {
        self.do_tre_phim_ms
    }

    /// Trả về độ trễ phím quy đổi sang giây (VD: -25ms -> -0.025s)
    pub fn get_do_tre_phim_giay(&self) -> f32 {
        self.do_tre_phim_ms / 1000.0
    }

    pub fn lay_kich_thuoc_cua_so(&self) -> (i32, i32) {
        match self.do_phan_giai_idx {
            0 => (1280, 720),
            1 => (1600, 900),
            2 => (1920, 1080),
            3 => (2560, 1440),
            _ => (1920, 1080),
        }
    }
}