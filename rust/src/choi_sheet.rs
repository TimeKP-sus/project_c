use godot::classes::{ Control, IControl, Label };

use godot::global::godot_print;
use godot::obj::{ Gd, GdRef, WithBaseField };
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
    #[export]
    ten_bai_hat_label: Option<Gd<Label>>,
    #[export]
    tac_gia_label: Option<Gd<Label>>,
    #[export]
    nguoi_tao_label: Option<Gd<Label>>,

    so_phim_tot: i32,
    so_phim_kha: i32,
    so_phim_truot: i32,
}

#[godot_api]
impl ChoiSheet {
    #[func]
    pub fn bat_tat_che_do_cho(&mut self, bat: bool) {
        if let Some(mut node) = self.quan_ly_sheet_node.clone() {
            node.bind_mut().set_cho_phep_cho(bat);
        }
    }
    pub fn set_thong_tin_bai_hat(&mut self) {
        if let Some(node) = &self.quan_ly_sheet_node {
            let sheet_bind: GdRef<'_, QuanLySheet> = node.bind();
            let ten_bai_hat = sheet_bind.get_ten_bai_hat();
            let tac_gia = sheet_bind.get_tac_gia();
            let nguoi_tao_sheet = sheet_bind.get_nguoi_tao_sheet();

            if let Some(mut label) = self.ten_bai_hat_label.clone() {
                label.set_text(ten_bai_hat);
            }
            if let Some(mut label) = self.tac_gia_label.clone() {
                label.set_text(tac_gia);
            }
            if let Some(mut label) = self.nguoi_tao_label.clone() {
                label.set_text(nguoi_tao_sheet);
            }
        }
    }
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

                let vi_tri_x = not_node.get_position().x;
                let chieu_dai_duoi = not_node.bind().get_chieu_dai_duoi();

                if chieu_dai_duoi < 0.0 {
                    // Tính độ lệch khoảng cách so với vạch đích 115.0
                    let do_lech: f32 = (vi_tri_x - 115.0).abs();

                    if do_lech <= 15.0 {
                        self.diem += 100;
                        self.so_phim_tot += 1;
                    } else if do_lech <= 40.0 {
                        self.diem += 50;
                        self.so_phim_kha += 1;
                    } else {
                        self.so_phim_truot += 1;
                    }
                    self.cat_nhat_diem_label(self.diem);
                }
                break;
            }
        }
    }

    #[func]
    pub fn xu_ly_nha_phim(&mut self, so_phim: i32) {
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
                        self.so_phim_tot += 1;
                    } else if toa_do_ket_thuc <= 250.0 {
                        self.diem += 50;
                        self.so_phim_kha += 1;
                    } else {
                        self.so_phim_truot += 1;
                    }
                    self.cat_nhat_diem_label(self.diem);
                }
                // Xóa phần else cộng điểm mặc định cho nốt ngắn ở đây

                break;
            }
        }
    }
    #[func]
    pub fn tong_ket(&mut self) {
        godot_print!("Điểm cuối cùng: {}", self.diem);
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
        self.set_thong_tin_bai_hat();
    }
}
