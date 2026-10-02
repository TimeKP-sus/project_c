use godot::{
    builtin::{ Color, GString },
    classes::{ Control, IControl, Label, TextureRect },
    obj::{ Base, Gd, WithBaseField },
    register::{ GodotClass, godot_api },
};

use crate::ban_phim_midi::BanPhimMidi;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct CumPhimDan {
    #[base]
    base: Base<Control>,
    #[export]
    cum_so: i32,
    ban_phim_midi: Option<Gd<BanPhimMidi>>,
}

impl CumPhimDan {
    pub fn bam_phim_thu(&mut self, so_phim: i32) {
        let path: String = format!("{}", so_phim);
        if let Some(mut phim_thu) = self.base_mut().try_get_node_as::<TextureRect>(&path) {
            let mut mau_hien_tai: Color = phim_thu.get_modulate();
            mau_hien_tai.r = 0.2;
            mau_hien_tai.b = 1.0;
            mau_hien_tai.g = 0.3;
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
            mau_hien_tai.r = 1.0;
            mau_hien_tai.b = 1.0;
            mau_hien_tai.g = 1.0;
            mau_hien_tai.a = 1.0;
            phim_thu.set_modulate(mau_hien_tai); // Đổi
        }
        // godot_print!("Thả phím {}", so_phim);
    }
    pub fn kiem_tra_hien_thi(&mut self, hien_thi_c: bool, hien_thi_all: bool) {
        if hien_thi_c {
            let text_c = GString::from(format!("C{}", self.cum_so));
            if let Some(mut label) = self.base().try_get_node_as::<Label>("p1") {
                label.set_visible(true);
                label.set_text(&text_c);
            }       
        }
        
        if hien_thi_all {
            if let Some(mut cac_not_khac) = self.base().try_get_node_as::<Control>("cac_not_khac") {
                cac_not_khac.set_visible(true);
            }     
        }
    }
}


#[godot_api]
impl IControl for CumPhimDan {
    fn ready(&mut self) {
        self.ban_phim_midi = self.base().try_get_node_as::<BanPhimMidi>("..");

    }
}
// fn ready(&mut self) {
//     let text_c = GString::from(format!("C{}", self.cum_so));
//     if let Some(mut label) = self.base_mut().try_get_node_as::<Label>("p1") {
//         label.set_text(&text_c);
//     }
//     let text_d = GString::from(format!("D{}", self.cum_so));
//     if let Some(mut label) = self.base_mut().try_get_node_as::<Label>("p3") {
//         label.set_text(&text_d);
//     }
// }
