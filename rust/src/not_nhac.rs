use godot::builtin::{ Color, GString, NodePath, Rect2, Variant, Vector2 };
use godot::classes::{ Area2D, CollisionShape2D, IArea2D, Label, Object, Panel, RectangleShape2D };
use godot::obj::{ NewGd, WithBaseField };
use godot::{ obj::Base, prelude::{ godot_api, GodotClass } };

use crate::check_ten_not::co_thang_giang;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct NotNhac {
    #[base]
    base: Base<Area2D>,
    id_not: i32,
    nhip_dich: f32,
    nhip_giu: f32,
    mau_not: String,
    chuyen_not: i32,
    ten_not: GString,
    ten_khac: GString,
    hien_thi_ten_khac: bool,
    da_duoc_danh: bool,
    dang_giu: bool,
    chieu_dai_duoi: f32,
    la_vach_canh_bao: bool,
    dang_duoc_chon: bool,
}

#[godot_api]
impl NotNhac {
    pub fn get_ten_not(&self) -> GString {
        self.ten_not.clone()
    }
    pub fn set_chieu_dai_duoi(&mut self, chieu_dai: f32) {
        self.chieu_dai_duoi = chieu_dai;
    }
    pub fn get_chieu_dai_duoi(&self) -> f32 {
        self.chieu_dai_duoi
    }
    pub fn get_dang_giu(&self) -> bool {
        self.dang_giu
    }
    pub fn set_ten_not(&mut self, ten: GString) {
        self.ten_not = ten;
    }

    pub fn set_ten_khac(&mut self, ten: GString) {
        self.ten_khac = ten;
    }

    pub fn get_ten_khac(&self) -> GString {
        self.ten_khac.clone()
    }

    pub fn set_hien_thi_ten_khac(&mut self, hien_thi: bool) {
        self.hien_thi_ten_khac = hien_thi;
        self.cap_nhat_nhan_ten();
    }

    /// Cập nhật chữ hiển thị trên Label dựa vào chế độ bật/tắt tên khác
    pub fn cap_nhat_nhan_ten(&mut self) {
        if self.la_vach_canh_bao {
            return;
        }
        let text: GString = if self.hien_thi_ten_khac && !self.ten_khac.is_empty() {
            self.ten_khac.clone()
        } else {
            self.ten_not.clone()
        };
        if let Some(mut label) = self.base_mut().try_get_node_as::<Label>("Label") {
            label.set_text(&text);
        }
    }

    pub fn set_nhip_dich(&mut self, nhip: f32) {
        self.nhip_dich = nhip;
    }
    pub fn get_nhip_dich(&self) -> f32 {
        self.nhip_dich
    }

    pub fn set_nhip_giu_goc(&mut self, nhip_giu: f32) {
        self.nhip_giu = nhip_giu;
    }
    pub fn get_nhip_giu_goc(&self) -> f32 {
        self.nhip_giu
    }

    pub fn get_mau_not(&self) -> String {
        self.mau_not.clone()
    }

    pub fn set_chuyen_not(&mut self, chuyen: i32) {
        self.chuyen_not = chuyen;
    }
    pub fn get_chuyen_not(&self) -> i32 {
        self.chuyen_not
    }

    /// Hiệu ứng khi được chọn hoặc hủy chọn trong TaoSheet
    pub fn set_duoc_chon(&mut self, chon: bool) {
        self.dang_duoc_chon = chon;
        if self.la_vach_canh_bao {
            if let Some(mut label) = self.base_mut().try_get_node_as::<Label>("Label") {
                if chon {
                    label.set_modulate(Color::from_rgba(1.0, 0.9, 0.1, 1.0));
                    label.set_scale(Vector2::new(1.05, 1.05));
                } else {
                    label.set_modulate(Color::from_rgba(0.6, 0.2, 0.9, 1.0));
                    label.set_scale(Vector2::new(0.8, 0.8));
                }
            }
            self.base_mut().queue_redraw();
        } else if chon {
            self.base_mut().set_modulate(Color::from_rgba(1.4, 1.4, 0.3, 1.0));
            self.base_mut().set_scale(Vector2::new(1.15, 1.15));
        } else {
            self.base_mut().set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 1.0));
            let ti_le = if co_thang_giang(self.id_not) { 0.8 } else { 1.0 };
            self.base_mut().set_scale(Vector2::new(ti_le, ti_le));
        }
    }

    pub fn bat_dau_giu(&mut self) {
        self.dang_giu = true;
        self.da_duoc_danh = true;

        self.base_mut().set_modulate(Color::from_rgba(1.0, 0.9, 0.2, 1.0));
        self.base_mut().set_scale(Vector2::new(1.2, 1.2));

        let doi_tuong = self.base().clone().upcast::<Object>();

        if let Some(mut tween) = self.base_mut().create_tween() {
            tween.tween_property(
                &doi_tuong,
                &NodePath::from("scale"),
                &Variant::from(Vector2::new(1.05, 1.05)),
                0.15
            );
        }
    }

    pub fn ket_thuc_giu(&mut self) {
        self.dang_giu = false;

        let doi_tuong = self.base().clone().upcast::<Object>();

        if let Some(mut tween) = self.base_mut().create_tween() {
            tween.tween_property(
                &doi_tuong,
                &NodePath::from("modulate"),
                &Variant::from(Color::from_rgba(0.2, 0.8, 1.0, 0.0)),
                0.2
            );
            tween.tween_property(
                &doi_tuong,
                &NodePath::from("scale"),
                &Variant::from(Vector2::new(0.8, 0.8)),
                0.2
            );
        }
    }

    pub fn da_danh_trat(&mut self) {
        self.base_mut().set_modulate(Color::from_rgba(1.0, 0.2, 0.2, 0.8));

        let doi_tuong = self.base().clone().upcast::<Object>();

        if let Some(mut tween) = self.base_mut().create_tween() {
            tween.tween_property(
                &doi_tuong,
                &NodePath::from("scale"),
                &Variant::from(Vector2::new(0.5, 0.5)),
                0.3
            );
            tween.tween_property(
                &doi_tuong,
                &NodePath::from("modulate"),
                &Variant::from(Color::from_rgba(1.0, 0.1, 0.1, 0.0)),
                0.3
            );
        }
    }

    pub fn set_mau_not(&mut self, mau: &str) {
        self.mau_not = mau.to_string();
        let mau_sac: Color = match mau {
            "1" => Color::from_rgba(0.85, 0.25, 0.25, 1.0),
            "2" => Color::from_rgba(0.2, 0.75, 0.35, 1.0),
            "3" => Color::from_rgba(0.25, 0.55, 0.9, 1.0),
            "4" => Color::from_rgba(0.95, 0.75, 0.15, 1.0),
            "5" => Color::from_rgba(0.6, 0.3, 0.8, 1.0),
            _ => Color::from_rgba(0.88, 0.88, 0.88, 1.0),
        };

        if let Some(mut panel) = self.base_mut().try_get_node_as::<Panel>("nen") {
            panel.set_self_modulate(mau_sac);
        }
    }

    pub fn get_da_duoc_danh(&self) -> bool {
        self.da_duoc_danh
    }

    #[func]
    pub fn da_duoc_danh(&mut self) {
        self.da_duoc_danh = true;
        self.base_mut().set_modulate(Color::from_rgba(0.5, 0.5, 0.5, 1.0));
    }

    pub fn set_id_not(&mut self, id: i32) {
        self.id_not = id;
    }
    pub fn get_id_not(&self) -> i32 {
        self.id_not
    }

    pub fn bien_thanh_vach_canh_bao(&mut self) {
        self.da_duoc_danh = true;
        self.la_vach_canh_bao = true;

        if let Some(mut panel) = self.base_mut().try_get_node_as::<Panel>("nen") {
            panel.queue_free();
        }

        if let Some(mut label) = self.base_mut().try_get_node_as::<Label>("Label") {
            label.set_position(Vector2::new(-4.0, -23.0));
            label.set_modulate(Color::from_rgba(0.6, 0.2, 0.9, 1.0));
            label.set_scale(Vector2::new(0.8, 0.8));
        }

        // Mở rộng vùng va chạm dọc theo vạch cảnh báo để dễ click chọn trong TaoSheet
        let children = self.base().get_children();
        for child in children.iter_shared() {
            if let Ok(mut col) = child.try_cast::<CollisionShape2D>() {
                let mut rect_shape = RectangleShape2D::new_gd();
                rect_shape.set_size(Vector2::new(18.0, 245.0));
                col.set_shape(&rect_shape);
                col.set_position(Vector2::new(1.0, 98.0));
                break;
            }
        }

        self.base_mut().queue_redraw();
    }
    pub fn thiet_lap_giao_dien_not_den(&mut self) {
        if !co_thang_giang(self.id_not) || self.la_vach_canh_bao {
            return;
        }

        // 1. Lấy node "nen" (Panel) để thu nhỏ chiều cao và làm đậm màu
        if let Some(mut panel) = self.base_mut().try_get_node_as::<Panel>("nen") {
            // Lấy kích thước hiện tại
            let size = panel.get_size();
            
            // Ép dẹp trục Y (Ví dụ từ 14px xuống 10px), giữ nguyên trục X
            let chieu_cao_moi = size.y * 0.7; 
            panel.set_size(Vector2::new(size.x, chieu_cao_moi));
            
            // Căn giữa lại trục Y sau khi thu nhỏ
            let pos = panel.get_position();
            let y_offset = (size.y - chieu_cao_moi) / 2.0;
            panel.set_position(Vector2::new(pos.x, pos.y + y_offset));
            
            // Làm đậm màu nền hiện tại lên một chút (giảm độ sáng RGB đi 20%)
            let mau_cu = panel.get_self_modulate();
            let mau_dam_hon = Color::from_rgba(
                (mau_cu.r * 0.8).max(0.0),
                (mau_cu.g * 0.8).max(0.0),
                (mau_cu.b * 0.8).max(0.0),
                mau_cu.a,
            );
            panel.set_self_modulate(mau_dam_hon);
        }

        // 2. Thu nhỏ chữ Label (C#4, D♭4...) một chút để vừa với nốt dẹp
        if let Some(mut label) = self.base_mut().try_get_node_as::<Label>("Label") {
            label.set_scale(Vector2::new(0.85, 0.85));
            // Căn chữ dịch xuống 1 chút cho cân đối
            let lbl_pos = label.get_position();
            label.set_position(Vector2::new(lbl_pos.x, lbl_pos.y + 1.5));
        }
    }
}

#[godot_api]
impl IArea2D for NotNhac {
    fn ready(&mut self) {
        let text: GString = if !self.la_vach_canh_bao && self.hien_thi_ten_khac && !self.ten_khac.is_empty() {
            self.ten_khac.clone()
        } else {
            self.ten_not.clone()
        };

        if let Some(mut label) = self.base().try_get_node_as::<Label>("Label") {
            label.set_text(&text);
            if co_thang_giang(self.id_not) {
                self.thiet_lap_giao_dien_not_den();
            }
        }
    }

    fn draw(&mut self) {
        if self.la_vach_canh_bao {
            let (do_rong, mau_canh_bao) = if self.dang_duoc_chon {
                (4.0, Color::from_rgba(1.0, 0.85, 0.1, 1.0))
            } else {
                (2.0, Color::from_rgba(0.6, 0.2, 0.9, 0.6))
            };
            let hinh_chu_nhat = Rect2::new(Vector2::new(0.0, -2.0), Vector2::new(do_rong, 220.0));
            self.base_mut().draw_rect(hinh_chu_nhat, mau_canh_bao);
        }
    }
}
