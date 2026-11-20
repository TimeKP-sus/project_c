use godot::{
    builtin::{ Color, GString, Vector2 },
    classes::{ Control, IControl, Input, InputEvent, InputEventMouse, Label, TextureRect },
    global::MouseButton,
    obj::{ Base, Gd, WithBaseField },
    register::{ godot_api, GodotClass },
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
    phim_chuot_dang_bam: i32,
}

#[godot_api]
impl CumPhimDan {
    fn tinh_so_phim_toan_cuc(&self, so_phim_trong_cum: i32) -> i32 {
        let cum = if self.cum_so > 0 {
            self.cum_so
        } else {
            self.base().get_name().to_string().parse::<i32>().unwrap_or(1)
        };
        (cum - 1) * 12 + (so_phim_trong_cum - 1)
    }

    fn tim_phim_duoi_chuot(&self, pos_chuot: Vector2) -> i32 {
        let phim_den = [2, 4, 7, 9, 11];
        for &id in &phim_den {
            if let Some(phim) = self.base().try_get_node_as::<TextureRect>(&format!("{}", id)) {
                if phim.is_visible_in_tree() && phim.get_global_rect().has_point(pos_chuot) {
                    return id;
                }
            }
        }

        let phim_trang = [1, 3, 5, 6, 8, 10, 12];
        for &id in &phim_trang {
            if let Some(phim) = self.base().try_get_node_as::<TextureRect>(&format!("{}", id)) {
                if phim.is_visible_in_tree() && phim.get_global_rect().has_point(pos_chuot) {
                    return id;
                }
            }
        }

        0
    }

    fn kich_hoat_bam_phim_chuot(&mut self, so_phim_trong_cum: i32) {
        self.phim_chuot_dang_bam = so_phim_trong_cum;
        let so_phim_midi = self.tinh_so_phim_toan_cuc(so_phim_trong_cum);
        if let Some(mut bp) = self.ban_phim_midi.clone() {
            bp.bind_mut().bam_phim_tu_chuot(so_phim_midi);
        }
    }

    #[func]
    pub fn nha_phim_chuot_hien_tai(&mut self) {
        if self.phim_chuot_dang_bam > 0 {
            let phim_cu = self.phim_chuot_dang_bam;
            self.phim_chuot_dang_bam = 0;
            let so_phim_midi = self.tinh_so_phim_toan_cuc(phim_cu);
            if let Some(mut bp) = self.ban_phim_midi.clone() {
                bp.bind_mut().tha_phim_tu_chuot(so_phim_midi);
            }
        }
    }
    #[func]
    pub fn to_mau_phim(&mut self, so_phim_trong_cum: i32, mau: Color) {
        let path: String = format!("{}", so_phim_trong_cum);
        if let Some(mut phim_thu) = self.base_mut().try_get_node_as::<TextureRect>(&path) {
            phim_thu.set_modulate(mau);
        }
    }

    #[func]
    pub fn dung_to_mau_phim(&mut self, so_phim_trong_cum: i32) {
        let path: String = format!("{}", so_phim_trong_cum);
        if let Some(mut phim_thu) = self.base_mut().try_get_node_as::<TextureRect>(&path) {
            let mut mau_hien_tai: Color = phim_thu.get_modulate();
            mau_hien_tai.r = 1.0;
            mau_hien_tai.b = 1.0;
            mau_hien_tai.g = 1.0;
            mau_hien_tai.a = 1.0;
            phim_thu.set_modulate(mau_hien_tai);
        }
    }
    #[func]
    pub fn bam_phim_thu(&mut self, so_phim: i32) {
        let path: String = format!("{}", so_phim);
        if let Some(mut phim_thu) = self.base_mut().try_get_node_as::<TextureRect>(&path) {
            let mut mau_hien_tai: Color = phim_thu.get_modulate();
            mau_hien_tai.r = 0.2;
            mau_hien_tai.b = 1.0;
            mau_hien_tai.g = 0.3;
            mau_hien_tai.a = 0.6;
            phim_thu.set_modulate(mau_hien_tai);
        }
    }
    #[func]
    pub fn tha_phim_thu(&mut self, so_phim: i32) {
        let path: String = format!("{}", so_phim);
        if let Some(mut phim_thu) = self.base_mut().try_get_node_as::<TextureRect>(&path) {
            let mut mau_hien_tai: Color = phim_thu.get_modulate();
            mau_hien_tai.r = 1.0;
            mau_hien_tai.b = 1.0;
            mau_hien_tai.g = 1.0;
            mau_hien_tai.a = 1.0;
            phim_thu.set_modulate(mau_hien_tai);
        }
    }

    pub fn kiem_tra_hien_thi(&mut self, hien_thi_c: bool, hien_thi_all: bool) {
        let text_c: GString = GString::from(format!("C{}", self.cum_so));
        if let Some(mut label) = self.base().try_get_node_as::<Label>("p1") {
            label.set_text(&text_c);
            label.set_visible(hien_thi_c || hien_thi_all);
        }

        if let Some(mut cac_not_khac) = self.base().try_get_node_as::<Control>("cac_not_khac") {
            cac_not_khac.set_visible(hien_thi_all);
        }
    }
}

#[godot_api]
impl IControl for CumPhimDan {
    fn ready(&mut self) {
        self.ban_phim_midi = self.base().try_get_node_as::<BanPhimMidi>("..");
        self.base_mut().set_process_input(true);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.try_cast::<InputEventMouse>().is_ok() {
            let cho_phep_bam_chuot = self
                .ban_phim_midi
                .as_ref()
                .map(|ban_phim| ban_phim.bind().cho_phep_bam_chuot)
                .unwrap_or(false);

            if !cho_phep_bam_chuot {
                self.nha_phim_chuot_hien_tai();
                return;
            }

            let chuot_dang_bam: bool = Input::singleton().is_mouse_button_pressed(
                MouseButton::LEFT
            );

            if chuot_dang_bam {
                let pos = self.base().get_global_mouse_position();
                let phim_duoi_chuot = self.tim_phim_duoi_chuot(pos);

                if phim_duoi_chuot != self.phim_chuot_dang_bam {
                    self.nha_phim_chuot_hien_tai();

                    if phim_duoi_chuot > 0 {
                        self.kich_hoat_bam_phim_chuot(phim_duoi_chuot);
                    }
                }
            } else {
                self.nha_phim_chuot_hien_tai();
            }
        }
    }
}
