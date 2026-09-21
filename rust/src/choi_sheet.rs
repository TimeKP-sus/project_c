use godot::classes::{ Control, IControl };
use godot::obj::{ Gd, WithBaseField };
use godot::{ obj::Base, prelude::{ GodotClass, godot_api } };

use crate::ban_phim_midi::BanPhimMidi;
use crate::quan_ly_sheet::QuanLySheet;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct ChoiSheet {
    #[base]
    base: Base<Control>,
    diem: i32,
    ban_phim_midi_node: Option<Gd<BanPhimMidi>>,
    quan_ly_sheet_node: Option<Gd<QuanLySheet>>,
}

#[godot_api]
impl ChoiSheet {
    pub fn cat_nhat_diem_label(&mut self, diem_moi: i32) {
        self.diem = diem_moi;
        if let Some(mut label) = self.base().try_get_node_as::<godot::classes::Label>("diem") {
            label.set_text(&format!("Điểm: {}", self.diem));
        }
    }
    #[func]
    pub fn kiem_tra_phim(&mut self, so_phim: i32) {
        let cac_not_trong_area = self.quan_ly_sheet_node
            .as_mut()
            .unwrap()
            .bind_mut()
            .get_danh_sach_not_trong_area();

        for mut not_node in cac_not_trong_area {
            let id_not: i32 = not_node.bind().get_id_not();
            let da_danh: bool = not_node.bind().get_da_duoc_danh();

            if id_not == so_phim && !da_danh {
                not_node.bind_mut().bat_dau_giu();
                break;
            }
        }
        // println!("{}", self.diem);
    }

    #[func]
    pub fn xu_ly_nha_phim(&mut self, so_phim: i32) {
        // godot_print!("nnhận tín hiệu nhả phím MIDI số: {}", so_phim);

        let tat_ca_not = self.quan_ly_sheet_node
            .as_mut()
            .unwrap()
            .bind_mut()
            .get_tat_ca_not_tren_sheet();

        for mut not_node in tat_ca_not {
            let id_not: i32 = not_node.bind().get_id_not();
            let dang_giu: bool = not_node.bind().get_dang_giu();

            if id_not == so_phim && dang_giu {
                not_node.bind_mut().ket_thuc_giu();

                let chieu_dai_duoi = not_node.bind().get_chieu_dai_duoi();
                if chieu_dai_duoi >= 0.0 {
                    let vi_tri_x: f32 = not_node.get_position().x;
                    let toa_do_ket_thuc: f32 = vi_tri_x + chieu_dai_duoi;
                    if toa_do_ket_thuc <= 150.0 {
                        self.diem += 200;
                    } else {
                        self.diem += 50;
                    }
                } else {
                    self.diem += 100;
                    // godot_print!("hoàn thành nốt ngắn diểm: {}", self.diem);
                }

                self.cat_nhat_diem_label(self.diem);
                break;
            }
        }
    }
}

#[godot_api]
impl IControl for ChoiSheet {
    fn ready(&mut self) {
        self.ban_phim_midi_node = self.base().try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano");
        self.quan_ly_sheet_node = self.base().try_get_node_as::<QuanLySheet>("QuanLySheet");

        let check_phim_midi: godot::prelude::Callable = self.base().callable("kiem_tra_phim");
        let nha_phim_midi: godot::prelude::Callable = self.base().callable("xu_ly_nha_phim");

        if let Some(mut midi_node) = self.ban_phim_midi_node.clone() {
            // Lắng nghe cả sự kiện nhấn và nhả
            midi_node.connect("phim_vua_duoc_bam", &check_phim_midi);
            midi_node.connect("phim_vua_duoc_nha", &nha_phim_midi);
        }
    }
}
