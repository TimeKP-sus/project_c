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
    #[func]
    pub fn kiem_tra_phim(&mut self, so_phim: i32) {
        let cac_not_trong_area = self
            .quan_ly_sheet_node
            .as_mut()
            .unwrap()
            .bind_mut()
            .get_danh_sach_not_trong_area();

        for mut not_node in cac_not_trong_area {
            let id_not = not_node.bind().get_id_not();
            let da_danh = not_node.bind().get_da_duoc_danh();

            if id_not == so_phim && !da_danh {
                // Chỉ đánh dấu nốt đã được bấm và bật trạng thái đổi màu
                not_node.bind_mut().bat_dau_giu(); 
                break;
            }
        }
    }

    #[func]
    pub fn xu_ly_nha_phim(&mut self, so_phim: i32) {
        let cac_not_trong_area = self
            .quan_ly_sheet_node
            .as_mut()
            .unwrap()
            .bind_mut()
            .get_danh_sach_not_trong_area();

        for mut not_node in cac_not_trong_area {
            let id_not = not_node.bind().get_id_not();
            let dang_giu = not_node.bind().get_dang_giu();

            // Nếu người chơi nhả đúng phím đang được giữ
            if id_not == so_phim && dang_giu {
                not_node.bind_mut().ket_thuc_giu();
                
                // TÍNH ĐIỂM 1 LẦN KHI NHẢ PHÍM TẠI ĐÂY
                let chieu_dai_duoi = not_node.bind().get_chieu_dai_duoi();
                
                if chieu_dai_duoi > 0.0 {
                    // Nếu là nốt dài (có đuôi), thưởng điểm cao hơn khi nhả phím
                    self.diem += 200; 
                    godot_print!("Hoàn thành nốt ngân! Điểm: {}", self.diem);
                } else {
                    // Nếu là nốt bình thường không đuôi
                    self.diem += 100;
                    godot_print!("Hoàn thành nốt ngắn! Điểm: {}", self.diem);
                }
                break;
            }
        }
    }
}

#[godot_api]
impl IControl for ChoiSheet {
    fn ready(&mut self) {
        self.ban_phim_midi_node = self
            .base()
            .try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano");
        self.quan_ly_sheet_node = self.base().try_get_node_as::<QuanLySheet>("QuanLySheet");

        let check_phim_midi = self.base().callable("kiem_tra_phim");
        let nha_phim_midi = self.base().callable("xu_ly_nha_phim");

        if let Some(mut midi_node) = self.ban_phim_midi_node.clone() {
            // Lắng nghe cả sự kiện nhấn và nhả[cite: 3]
            midi_node.connect("phim_vua_duoc_bam", &check_phim_midi);
            midi_node.connect("phim_vua_duoc_nha", &nha_phim_midi); 
        }
    }
}