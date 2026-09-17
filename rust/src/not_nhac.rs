use godot::builtin::{Color, GString, Vector2};
use godot::classes::{Area2D, IArea2D, Label, Panel};
use godot::obj::WithBaseField;
use godot::{
    obj::Base,
    prelude::{GodotClass, godot_api},
};

use crate::check_ten_not::co_thang_giang;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct NotNhac {
    #[base]
    base: Base<Area2D>,
    // #[export]
    id_not: i32,
    // #[export]
    // thoi_gian_dich: f32,
    // #[export]
    ten_not: GString,
    // #[export]
    da_duoc_danh: bool,
    dang_giu: bool,
    chieu_dai_duoi: f32,
    // nen: Option<Gd<Panel>>,
}
#[godot_api]
impl NotNhac {
    pub fn set_chieu_dai_duoi(&mut self, chieu_dai: f32) {
        self.chieu_dai_duoi = chieu_dai;
    }
    pub fn get_chieu_dai_duoi(&self) -> f32 {
        self.chieu_dai_duoi
    }

    pub fn bat_dau_giu(&mut self) {
        self.dang_giu = true;
        self.da_duoc_danh = true; 

        self.base_mut().set_modulate(Color::from_rgba(0.5, 0.5, 0.5, 1.0)); 
    }
    
    pub fn dang_giu(&self) -> bool {
        self.dang_giu
    }
    pub fn set_mau_not(&mut self, mau: String) {
        // Dùng match để so sánh chuỗi (cần thêm .as_str() để mượn chuỗi)
        let mau_sac = match mau.as_str() {
            "1" => Color::from_rgba(0.85, 0.25, 0.25, 1.0),
            "2" => Color::from_rgba(0.20, 0.75, 0.35, 1.0),
            "3" => Color::from_rgba(0.25, 0.55, 0.90, 1.0),
            "4" => Color::from_rgba(0.95, 0.75, 0.15, 1.0),
            "5" => Color::from_rgba(0.60, 0.30, 0.80, 1.0),
            _ => Color::from_rgba(0.88, 0.88, 0.88, 1.0),
        };

        // Tìm Panel tên "nen" và chỉ đổi màu bản thân nó
        if let Some(mut panel) = self.base_mut().try_get_node_as::<Panel>("nen") {
            panel.set_self_modulate(mau_sac);
        }
    }
    pub fn get_da_duoc_danh(&self) -> bool {
        self.da_duoc_danh
    }
    #[func]
    pub fn da_duoc_danh(&mut self) {
        godot::global::godot_print!("Nốt {} đã được đánh!", self.ten_not);
        self.da_duoc_danh = true;
        self.base_mut()
            .set_modulate(Color::from_rgba(0.5, 0.5, 0.5, 1.0));
    }
    pub fn set_id_not(&mut self, id: i32) {
        self.id_not = id;
    }
    pub fn get_id_not(&self) -> i32 {
        self.id_not
    }
    pub fn set_ten_not(&mut self, ten: GString) {
        self.ten_not = ten;
    }
    // pub fn get_ten_not(&self) -> GString {
    //     self.ten_not.clone()
    // }
}

#[godot_api]
impl IArea2D for NotNhac {
    fn ready(&mut self) {
        let text: GString = self.ten_not.to_string().into();

        if let Some(mut label) = self.base().try_get_node_as::<Label>("Label") {
            label.set_text(&text);
        }
        if co_thang_giang(self.id_not) {
            self.base_mut().set_scale(Vector2::new(0.8, 0.8));
        }
    }
}
