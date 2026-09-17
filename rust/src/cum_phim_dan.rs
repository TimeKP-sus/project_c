use godot::{
    builtin::{Color, GString},
    classes::{Control, IControl, Label, TextureRect},
    obj::{Base, WithBaseField},
    register::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct CumPhimDan {
    #[base]
    base: Base<Control>,
    #[export]
    cum_so: i32,
}

impl CumPhimDan {
    pub fn bam_phim_thu(&mut self, so_phim: i32) {
        let path: String = format!("{}", so_phim);
        if let Some(mut phim_thu) = self.base_mut().try_get_node_as::<TextureRect>(&path) {
            let mut mau_hien_tai: Color = phim_thu.get_modulate();
            mau_hien_tai.a = 0.6;
            phim_thu.set_modulate(mau_hien_tai);
            // godot_print!("Không tìm thấy nút PhimThu {}", so_phim);
        }
        // godot_print!("Bấm phím {}", so_phim);
    }
    pub fn tha_phim_thu(&mut self, so_phim: i32) {
        let path: String = format!("{}", so_phim);
        if let Some(mut phim_thu) = self.base_mut().try_get_node_as::<TextureRect>(&path) {
            let mut mau_hien_tai: Color = phim_thu.get_modulate();
            mau_hien_tai.a = 1.0;
            phim_thu.set_modulate(mau_hien_tai); // Đổi 
        }
        // godot_print!("Thả phím {}", so_phim);
    }
}

#[godot_api]
impl IControl for CumPhimDan {
    fn ready(&mut self) {
        let text = GString::from(format!("C{}", self.cum_so));
        if let Some(mut label) = self.base_mut().try_get_node_as::<Label>("Label") {
            label.set_text(&text);
        }
    }
}
