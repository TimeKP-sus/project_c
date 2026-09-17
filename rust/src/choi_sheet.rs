use godot::classes::{Control, IControl};
use godot::global::godot_print;
use godot::obj::{Gd, WithBaseField};
use godot::{
    obj::Base,
    prelude::{GodotClass, godot_api},
};

use crate::ban_phim_midi::BanPhimMidi;
use crate::quan_ly_sheet::QuanLySheet;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct ChoiSheet {
    #[base]
    base: Base<Control>,
    diem: i32,
    ban_phim_midi_node: Option<Gd<BanPhimMidi>>,
    quan_ly_sheet_node: Option<Gd<QuanLySheet>>,
}
#[godot_api]
impl ChoiSheet {
    // pub fn kiem_tra_phim(&mut self) {
    //     let danh_sach_phim_bam: HashSet<i32> = self
    //         .ban_phim_midi_node
    //         .as_ref()
    //         .unwrap()
    //         .bind()
    //         .get_danh_sach_phim_bam();
    //     let cac_not_trong_vung: Vec<Gd<crate::not_nhac::NotNhac>> = self
    //         .quan_ly_sheet_node
    //         .as_ref()
    //         .unwrap()
    //         .bind()
    //         .get_danh_sach_not_trong_area();
    //     for mut not_node in cac_not_trong_vung {
    //         let id_cua_not: i32 = not_node.bind().get_id_not();
    //         if danh_sach_phim_bam.contains(&id_cua_not) {
    //             not_node.bind_mut().da_duoc_danh();
    //             self.diem += 100;
    //             break;
    //         }
    //     }
    // }
    #[func]
    pub fn kiem_tra_phim(&mut self, so_phim: i32) {
        let cac_not_trong_area: Vec<Gd<crate::not_nhac::NotNhac>> = self
            .quan_ly_sheet_node
            .as_mut()
            .unwrap()
            .bind_mut()
            .get_danh_sach_not_trong_area();
        // godot_print!("Các nốt trong vùng: ");
        for mut not_node in cac_not_trong_area {
            print!("{} ", not_node.bind().get_id_not());
            let id_not: i32 = not_node.bind().get_id_not();
            let da_danh: bool = not_node.bind().get_da_duoc_danh();

            if id_not == so_phim && !da_danh {
                not_node.bind_mut().da_duoc_danh();
                self.diem += 100;
                godot_print!("Điểm: {}", self.diem);
                break;
            }
        }
    }
    // pub fn get_diem(&self) -> i32 {
    //     self.diem
    // }
}

#[godot_api]
impl IControl for ChoiSheet {
    fn ready(&mut self) {
        self.ban_phim_midi_node = self
            .base()
            .try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano");
        self.quan_ly_sheet_node = self.base().try_get_node_as::<QuanLySheet>("QuanLySheet");

        let check_phim_midi: godot::prelude::Callable = self.base().callable("kiem_tra_phim");

        if let Some(mut midi_node) = self.ban_phim_midi_node.clone() {
            midi_node.connect("phim_vua_duoc_bam", &check_phim_midi);
            godot_print!("da ket noi signal phim_vua_duoc_bam voi kiem_tra_phim");
        } else {
            godot_print!("loi tim node BanPhimMidi");
        }
    }
}
