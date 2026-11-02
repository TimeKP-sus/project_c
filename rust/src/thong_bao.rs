use godot::builtin::{Callable, Color, GString, NodePath, Variant, Vector2};
use godot::classes::{
    IPanel, Label, Object, Panel, StyleBoxFlat, Tween, tween::EaseType, tween::TransitionType,
};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{GodotClass, godot_api};

#[derive(GodotClass)]
#[class(base = Panel)]
pub struct ThongBao {
    #[base]
    base: Base<Panel>,

    /// Lưu vị trí gốc trên màn hình để làm mốc chạy hiệu ứng trượt
    vi_tri_goc: Vector2,
    da_luu_vi_tri: bool,

    /// Quản lý Tween hiện tại để hủy nếu có thông báo mới đè lên liên tiếp
    tween_hien_tai: Option<Gd<Tween>>,

    /// Cho phép tự động hủy node (queue_free) sau khi biến mất (dùng khi instantiate động)
    #[export]
    tu_dong_xoa_khi_an: bool,
}

#[godot_api]
impl ThongBao {
    /// Tín hiệu phát ra ngay khi thông báo bắt đầu hiển thị
    #[signal]
    fn bat_dau_hien(noi_dung: GString);

    /// Tín hiệu phát ra sau khi hiệu ứng biến mất hoàn tất
    #[signal]
    fn da_bien_mat();

    /// Hàm chính: Khởi tạo và hiển thị thông báo với nội dung, màu chủ đạo và thời gian giữ trên màn hình
    #[func]
    pub fn hien_thong_bao(&mut self, noi_dung: GString, mau_sac: Color, thoi_gian_hien: f64) {
        if !self.da_luu_vi_tri {
            self.vi_tri_goc = self.base().get_position();
            self.da_luu_vi_tri = true;
        }

        // Hủy hiệu ứng cũ nếu thông báo trước đó vẫn đang chạy dở
        if let Some(mut t) = self.tween_hien_tai.take() {
            if t.is_valid() {
                t.kill();
            }
        }

        // 1. Cập nhật văn bản vào Label
        if let Some(mut lbl) = self.base().try_get_node_as::<Label>("HBox/Label") {
            lbl.set_text(&noi_dung);
        } else if let Some(mut lbl) = self.base().try_get_node_as::<Label>("Label") {
            lbl.set_text(&noi_dung);
        }

        // 2. Cập nhật màu sắc cho Thanh Điểm Nhấn và Viền Panel
        self.cap_nhat_mau_giao_dien(mau_sac);

        // 3. Chuẩn bị trạng thái ban đầu trước khi chạy hiệu ứng Đến (ẩn và dịch lên trên 35px)
        let vi_tri_bat_dau = Vector2::new(self.vi_tri_goc.x, self.vi_tri_goc.y - 35.0);
        let vi_tri_ket_thuc = self.vi_tri_goc;
        let vi_tri_bien_mat = Vector2::new(self.vi_tri_goc.x, self.vi_tri_goc.y - 25.0);

        self.base_mut().set_position(vi_tri_bat_dau);
        self.base_mut().set_scale(Vector2::new(0.95, 0.95));
        self.base_mut()
            .set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 0.0));
        self.base_mut().set_visible(true);

        // Phát tín hiệu bắt đầu hiện
        self.base_mut()
            .emit_signal("bat_dau_hien", &[Variant::from(noi_dung)]);

        let doi_tuong: Gd<Object> = self.base().clone().upcast::<Object>();
        let obj_gd = self.to_gd();

        // 4. Tạo chuỗi hiệu ứng: Trượt đến -> Đợi -> Biến mất
        let tween_moi = { self.base_mut().create_tween() };
        if let Some(mut tween) = tween_moi {
            // --- HIỆU ỨNG ĐẾN (0.35 giây) ---
            tween.set_parallel();
            if let Some(mut step) = tween.tween_property(
                &doi_tuong,
                &NodePath::from("position"),
                &Variant::from(vi_tri_ket_thuc),
                0.35,
            ) {
                step.set_trans(TransitionType::BACK);
                step.set_ease(EaseType::OUT);
            }
            if let Some(mut step) = tween.tween_property(
                &doi_tuong,
                &NodePath::from("scale"),
                &Variant::from(Vector2::new(1.0, 1.0)),
                0.3,
            ) {
                step.set_trans(TransitionType::CUBIC);
                step.set_ease(EaseType::OUT);
            }
            tween.tween_property(
                &doi_tuong,
                &NodePath::from("modulate:a"),
                &Variant::from(1.0_f32),
                0.25,
            );

            // --- THỜI GIAN ĐỨNG YÊN ĐỂ ĐỌC ---
            tween.set_parallel_ex().parallel(false).done();
            tween.tween_interval(thoi_gian_hien.max(0.5));

            // --- HIỆU ỨNG BIẾN MẤT (0.3 giây) ---
            tween.set_parallel();
            if let Some(mut step) = tween.tween_property(
                &doi_tuong,
                &NodePath::from("position"),
                &Variant::from(vi_tri_bien_mat),
                0.3,
            ) {
                step.set_trans(TransitionType::CUBIC);
                step.set_ease(EaseType::IN);
            }
            tween.tween_property(
                &doi_tuong,
                &NodePath::from("modulate:a"),
                &Variant::from(0.0_f32),
                0.28,
            );

            // --- KẾT THÚC ---
            tween.set_parallel_ex().parallel(false).done();
            tween.tween_callback(&Callable::from_object_method(
                &obj_gd,
                "_khi_hoan_tat_bien_mat",
            ));

            self.tween_hien_tai = Some(tween);
        }
    }

    // ==================== CÁC HÀM GỌI NHANH THEO LOẠI ====================

    #[func]
    pub fn thong_bao_thanh_cong(&mut self, noi_dung: GString) {
        self.dat_icon("✔");
        self.hien_thong_bao(noi_dung, Color::from_rgba(0.2, 0.85, 0.45, 1.0), 2.5);
    }

    #[func]
    pub fn thong_bao_canh_bao(&mut self, noi_dung: GString) {
        self.dat_icon("⚠");
        self.hien_thong_bao(noi_dung, Color::from_rgba(1.0, 0.75, 0.2, 1.0), 3.0);
    }

    #[func]
    pub fn thong_bao_loi(&mut self, noi_dung: GString) {
        self.dat_icon("✖");
        self.hien_thong_bao(noi_dung, Color::from_rgba(0.95, 0.3, 0.3, 1.0), 3.2);
    }

    #[func]
    pub fn thong_bao_thong_tin(&mut self, noi_dung: GString) {
        self.dat_icon("🔔");
        self.hien_thong_bao(noi_dung, Color::from_rgba(0.35, 0.78, 1.0, 1.0), 2.5);
    }

    fn dat_icon(&mut self, ky_tu: &str) {
        if let Some(mut icon_lbl) = self.base().try_get_node_as::<Label>("HBox/Icon") {
            icon_lbl.set_text(ky_tu);
        }
    }

    /// Đổi màu viền Panel, màu chữ Icon và vạch màu bên trái theo màu sắc được truyền vào
    fn cap_nhat_mau_giao_dien(&mut self, mau_sac: Color) {
        // Đổi màu vạch điểm nhấn bên trái
        if let Some(mut vach_mau) = self.base().try_get_node_as::<Panel>("VachMau") {
            vach_mau.set_self_modulate(mau_sac);
        }

        // Đổi màu Icon
        if let Some(mut icon_lbl) = self.base().try_get_node_as::<Label>("HBox/Icon") {
            icon_lbl.set_modulate(mau_sac);
        }

        // Tạo bản sao StyleBoxFlat của Panel chính để đổi màu viền không ảnh hưởng các instance khác
        if let Some(style) = self.base().get_theme_stylebox("panel") {
            if let Some(duplicated) = style.duplicate() {
                if let Ok(mut flat) = duplicated.try_cast::<StyleBoxFlat>() {
                    let mut mau_vien = mau_sac;
                    mau_vien.a = 0.9;
                    flat.set_border_color(mau_vien);
                    self.base_mut().add_theme_stylebox_override("panel", &flat);
                }
            }
        }
    }

    #[func]
    fn _khi_hoan_tat_bien_mat(&mut self) {
        let vi_tri_goc = self.vi_tri_goc;
        self.base_mut().set_visible(false);
        self.base_mut().set_position(vi_tri_goc);
        self.base_mut().emit_signal("da_bien_mat", &[]);

        if self.tu_dong_xoa_khi_an {
            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl IPanel for ThongBao {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            vi_tri_goc: Vector2::ZERO,
            da_luu_vi_tri: false,
            tween_hien_tai: None,
            tu_dong_xoa_khi_an: false,
        }
    }

    fn ready(&mut self) {
        self.vi_tri_goc = self.base().get_position();
        self.da_luu_vi_tri = true;
        // Ẩn mặc định khi mới vào scene, chỉ hiện khi gọi hàm thông báo
        self.base_mut().set_visible(false);
    }
}
