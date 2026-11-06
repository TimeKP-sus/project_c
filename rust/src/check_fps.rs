use godot::{
    builtin::{Color, GString},
    classes::{Engine, ILabel, Label},
    obj::{Base, WithBaseField},
    prelude::{godot_api, GodotClass},
};

use crate::du_lieu_nguoi_dung::CaiDatGameJson;

#[derive(GodotClass)]
#[class(base = Label)]
pub struct CheckFPS {
    #[base]
    base: Base<Label>,

    /// Khoảng thời gian giữa 2 lần cập nhật chữ (mặc định 0.25s để số không bị nháy liên tục)
    #[export]
    chu_ky_cap_nhat: f32,

    /// Nếu bật true, Label sẽ đổi màu Xanh / Vàng / Đỏ tùy theo mức FPS
    #[export]
    doi_mau_theo_fps: bool,

    dem_thoi_gian: f32,
}

#[godot_api]
impl CheckFPS {
    /// Hàm đọc trực tiếp số FPS hiện tại từ Godot Engine
    #[func]
    pub fn doc_fps(&self) -> i32 {
        Engine::singleton().get_frames_per_second() as i32
    }

    /// Hàm lấy chuỗi FPS đã định dạng (VD: "FPS: 60")
    #[func]
    pub fn lay_chuoi_fps(&self) -> GString {
        GString::from(format!("FPS: {}", self.doc_fps()))
    }

    /// Bật hoặc tắt hiển thị FPS ngay lập tức (có thể gọi từ màn hình Cài đặt)
    #[func]
    pub fn dat_hien_thi_fps(&mut self, bat: bool) {
        self.base_mut().set_visible(bat);
        self.base_mut().set_process(bat);
        if bat {
            self.cap_nhat_hien_thi();
        }
    }

    /// Đọc lại trạng thái `hien_thi_fps` từ file cài đặt của game.
    #[func]
    pub fn tai_lai_tu_cai_dat(&mut self) {
        let cai_dat = CaiDatGameJson::doc_cai_dat();
        self.dat_hien_thi_fps(cai_dat.hien_thi_fps);
    }

    /// Cập nhật chữ và màu sắc lên chính Node Label này
    #[func]
    pub fn cap_nhat_hien_thi(&mut self) {
        let fps = self.doc_fps();
        self.base_mut().set_text(&format!("FPS: {}", fps));

        if self.doi_mau_theo_fps {
            let mau = if fps >= 55 {
                // Xanh lá sáng (Mượt)
                Color::from_rgba(0.3, 1.0, 0.45, 1.0)
            } else if fps >= 30 {
                // Vàng cam (Trung bình)
                Color::from_rgba(1.0, 0.82, 0.2, 1.0)
            } else {
                // Đỏ (Giật/Lag)
                Color::from_rgba(1.0, 0.3, 0.3, 1.0)
            };
            self.base_mut().add_theme_color_override("font_color", mau);
        }
    }
}

#[godot_api]
impl ILabel for CheckFPS {
    fn init(base: Base<Label>) -> Self {
        Self {
            base,
            chu_ky_cap_nhat: 1.0,
            doi_mau_theo_fps: true,
            dem_thoi_gian: 0.0,
        }
    }

    fn ready(&mut self) {
        // Thêm viền đen nhẹ để đặt ở góc màn hình nào cũng đọc rõ chữ
        self.base_mut()
            .add_theme_color_override("font_outline_color", Color::from_rgba(0.0, 0.0, 0.0, 0.9));
        self.base_mut().add_theme_constant_override("outline_size", 4);

        // Tự động kiểm tra file cai_dat.json xem người chơi có bật Hiển thị FPS không
        self.tai_lai_tu_cai_dat();
    }

    fn process(&mut self, delta: f64) {
        self.dem_thoi_gian += delta as f32;
        if self.dem_thoi_gian >= self.chu_ky_cap_nhat {
            self.dem_thoi_gian = 0.0;
            self.cap_nhat_hien_thi();
        }
    }
}