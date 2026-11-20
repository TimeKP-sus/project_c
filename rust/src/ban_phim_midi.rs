use std::collections::{HashMap, HashSet};
use crate::ham_phu::lay_duong_dan_goc;
use std::thread;
use std::time::Duration;

use crate::{ hop_am::HopAm, sf2_read };
use crate::cum_phim_dan::CumPhimDan;
use godot::builtin::{ Color, Variant };
use godot::global::MidiMessage;
use godot::meta::ToGodot;
use godot::{
    builtin::GString,
    classes::{
        HBoxContainer,
        IHBoxContainer,
        InputEvent,
        InputEventKey,
        InputEventMidi,
        Label,
        Os,
        ProjectSettings,
    },
    global::godot_print,
    obj::{ Base, EngineEnum, Gd, WithBaseField },
    register::{ GodotClass, godot_api },
};

#[derive(GodotClass)]
#[class(base = HBoxContainer)]
pub struct BanPhimMidi {
    #[base]
    base: Base<HBoxContainer>,
    pedal_phai: bool,
    pedal_giua: bool,
    pedal_trai: bool,
    midi_input: GString,
    cum_1: Option<Gd<CumPhimDan>>,
    cum_2: Option<Gd<CumPhimDan>>,
    cum_3: Option<Gd<CumPhimDan>>,
    cum_4: Option<Gd<CumPhimDan>>,
    cum_5: Option<Gd<CumPhimDan>>,
    cum_6: Option<Gd<CumPhimDan>>,
    cum_7: Option<Gd<CumPhimDan>>,

    #[export]
    path_sf2: GString,
    #[export]
    hop_am: GString,
    danh_sach_phim_bam: HashSet<i32>,
    cho_phep_bam_ban_phim: bool,
    ban_do_phim_may_tinh: HashMap<i64, i32>,

    #[export]
    cho_phep_kiem_tra_hop_am: bool,
    #[export]
    cho_phep_hien_thi_phim: bool,
    #[export]
    hien_thi_ten_not_c: bool,
    #[export]
    hien_thi_ten_not_all: bool,
    #[export]
    pub cho_phep_bam_chuot: bool,
}

#[godot_api]
impl BanPhimMidi {
    #[signal]
    fn phim_vua_duoc_bam(so_phim: i32);
    #[signal]
    fn phim_vua_duoc_nha(so_phim: i32);

    fn cat_nhat_hop_am(&mut self) {
        let hop_am: String = if self.cho_phep_kiem_tra_hop_am {
            HopAm::kiem_tra(&self.danh_sach_phim_bam)
        } else {
            String::new()
        };

        self.hop_am = GString::from(hop_am.clone());
        if let Some(mut label_hop_am) = self.base_mut().try_get_node_as::<Label>("../hop_am") {
            label_hop_am.set_text(&hop_am);
        }
    }
    ///mili giay
    #[func]
    pub fn phat_am_thanh_not(&mut self, so_phim: i32, luc: i32, thoi_gian: u64) {
        let nut: i32 = so_phim + 24;
        sf2_read::bat_not_nhac(nut, luc);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(thoi_gian));
            sf2_read::tat_not_nhac(nut);
        });
    }

    fn kiem_tra_phim_midi(&mut self, event: Gd<InputEvent>) {
        if let Ok(midi_event) = event.try_cast::<InputEventMidi>() {
            let tha_phim: MidiMessage = midi_event.get_message();
            let nut: i32 = midi_event.get_pitch();
            let luc: i32 = midi_event.get_velocity();

            match tha_phim {
                MidiMessage::NOTE_ON if luc > 0 => {
                    self.bam_phim(nut - 24);
                    sf2_read::bat_not_nhac(nut, luc);
                }

                MidiMessage::NOTE_ON if luc <= 0 => {
                    self.tha_phim(nut - 24);
                    sf2_read::tat_not_nhac(nut);
                }
                MidiMessage::NOTE_OFF => {
                    self.tha_phim(nut - 24);
                    sf2_read::tat_not_nhac(nut);
                }
                MidiMessage::CONTROL_CHANGE => {
                    let id_pedals: i32 = midi_event.get_controller_number();
                    let gia_tri_pedal: i32 = midi_event.get_controller_value();
                    sf2_read::thay_doi_dieu_khien(id_pedals, gia_tri_pedal);
                    self.pedal_phai = id_pedals == 64 && gia_tri_pedal > 0;
                    self.pedal_giua = id_pedals == 66 && gia_tri_pedal > 0;
                    self.pedal_trai = id_pedals == 67 && gia_tri_pedal > 0;
                    self.kiem_tra_pedal();
                }
                _ => {}
            }
        }
    }

    fn kiem_tra_phim_may_tinh(&mut self, event: Gd<InputEvent>) {
        if !self.cho_phep_bam_ban_phim {
            return;
        }
        if let Some(viewport) = self.base().get_viewport() {
            if let Some(focus) = viewport.gui_get_focus_owner() {
                if focus.is_class("LineEdit") || focus.is_class("TextEdit") {
                    return;
                }
            }
        }
        if let Ok(key_event) = event.try_cast::<InputEventKey>() {
            if key_event.is_echo() {
                return;
            }
            let code = key_event.get_physical_keycode().ord() as i64;
            if let Some(&so_phim) = self.ban_do_phim_may_tinh.get(&code) {
                if key_event.is_pressed() {
                    if !self.danh_sach_phim_bam.contains(&so_phim) {
                        self.bam_phim_tu_chuot(so_phim);
                    }
                } else {
                    self.tha_phim_tu_chuot(so_phim);
                }
            }
        }
    }
    #[func]
    pub fn bat_tat_bam_bang_chuot(&mut self, bat: bool) {
        self.cho_phep_bam_chuot = bat;
        if !bat {
            for cum in [
                &mut self.cum_1,
                &mut self.cum_2,
                &mut self.cum_3,
                &mut self.cum_4,
                &mut self.cum_5,
                &mut self.cum_6,
                &mut self.cum_7,
            ] {
                if let Some(cum) = cum {
                    cum.call_deferred("nha_phim_chuot_hien_tai", &[]);
                }
            }
        }
    }

    #[func]
    pub fn luu_cai_dat_ban_phim(
        &mut self,
        hien_c: bool,
        hien_all: bool,
        hop_am: bool,
        hien_phim: bool,
        bam_chuot: bool,
    ) {
        self.hien_thi_ten_not_c = hien_c;
        self.hien_thi_ten_not_all = hien_all;
        self.cho_phep_kiem_tra_hop_am = hop_am;
        self.cho_phep_hien_thi_phim = hien_phim;
        self.cho_phep_bam_chuot = bam_chuot;

        if !bam_chuot {
            self.bat_tat_bam_bang_chuot(false);
        }

        let mut cai_dat = crate::du_lieu_nguoi_dung::CaiDatGameJson::doc_cai_dat();
        cai_dat.hien_thi_ten_not_c = hien_c;
        cai_dat.hien_thi_ten_not_all = hien_all;
        cai_dat.kiem_tra_hop_am = hop_am;
        cai_dat.hien_thi_phim_bam = hien_phim;
        cai_dat.bam_bang_chuot = bam_chuot;
        cai_dat.luu_cai_dat();
    }
    #[func]
    pub fn hien_thi_ten_not(&mut self) {
        // Lấy giá trị lưu vào biến tạm trước
        let hien_c: bool = self.hien_thi_ten_not_c;
        let hien_all: bool = self.hien_thi_ten_not_all;

        if let Some(cum) = &mut self.cum_1 {
            cum.bind_mut().kiem_tra_hien_thi(hien_c, hien_all);
        }
        if let Some(cum) = &mut self.cum_2 {
            cum.bind_mut().kiem_tra_hien_thi(hien_c, hien_all);
        }
        if let Some(cum) = &mut self.cum_3 {
            cum.bind_mut().kiem_tra_hien_thi(hien_c, hien_all);
        }
        if let Some(cum) = &mut self.cum_4 {
            cum.bind_mut().kiem_tra_hien_thi(hien_c, hien_all);
        }
        if let Some(cum) = &mut self.cum_5 {
            cum.bind_mut().kiem_tra_hien_thi(hien_c, hien_all);
        }
        if let Some(cum) = &mut self.cum_6 {
            cum.bind_mut().kiem_tra_hien_thi(hien_c, hien_all);
        }
        if let Some(cum) = &mut self.cum_7 {
            cum.bind_mut().kiem_tra_hien_thi(hien_c, hien_all);
        }
    }

    pub fn cat_nhat_danh_sach_phim_bam(&mut self) {
        // KIỂM TRA BẬT/TẮT: Nếu tắt, trả về chuỗi rỗng
        let ket_qua: String = if self.cho_phep_hien_thi_phim {
            let mut danh_sach: Vec<i32> = self.danh_sach_phim_bam.iter().cloned().collect();
            danh_sach.sort();
            let danh_sach_str: Vec<String> = danh_sach
                .iter()
                .map(|&num| num.to_string())
                .collect();
            danh_sach_str.join(" ")
        } else {
            String::new()
        };

        if let Some(mut label_danh_sach) = self.base_mut().try_get_node_as::<Label>("../cac_phim") {
            label_danh_sach.set_text(&ket_qua);
        }
        self.cat_nhat_hop_am();
    }

    // HÀM HỖ TRỢ ĐỂ CHUYỂN TÍN HIỆU ĐỔI MÀU THÀNH HÀNG ĐỢI (DEFERRED)
    fn goi_cum_phim_deferred(&mut self, id_cum: i32, ten_ham: &str, tham_so: &[Variant]) {
        match id_cum {
            1 => if let Some(cum) = &mut self.cum_1 {
                cum.call_deferred(ten_ham, tham_so);
            }
            2 => if let Some(cum) = &mut self.cum_2 {
                cum.call_deferred(ten_ham, tham_so);
            }
            3 => if let Some(cum) = &mut self.cum_3 {
                cum.call_deferred(ten_ham, tham_so);
            }
            4 => if let Some(cum) = &mut self.cum_4 {
                cum.call_deferred(ten_ham, tham_so);
            }
            5 => if let Some(cum) = &mut self.cum_5 {
                cum.call_deferred(ten_ham, tham_so);
            }
            6 => if let Some(cum) = &mut self.cum_6 {
                cum.call_deferred(ten_ham, tham_so);
            }
            7 => if let Some(cum) = &mut self.cum_7 {
                cum.call_deferred(ten_ham, tham_so);
            }
            _ => {}
        }
    }

    #[func]
    pub fn to_phim(&mut self, so_phim: i32, mau: Color) {
        let id_cum: i32 = so_phim / 12 + 1;
        let so_phim_trong_cum: i32 = (so_phim % 12) + 1;
        // Chuyển toàn bộ 40 dòng lệnh match rườm rà thành 1 dòng duy nhất
        self.goi_cum_phim_deferred(
            id_cum,
            "to_mau_phim",
            &[so_phim_trong_cum.to_variant(), mau.to_variant()]
        );
    }

    #[func]
    pub fn dung_to_phim(&mut self, so_phim: i32) {
        let id_cum: i32 = so_phim / 12 + 1;
        let so_phim_trong_cum = (so_phim % 12) + 1;
        self.goi_cum_phim_deferred(id_cum, "dung_to_mau_phim", &[so_phim_trong_cum.to_variant()]);
    }

    pub fn bam_phim(&mut self, so_phim: i32) {
        self.danh_sach_phim_bam.insert(so_phim);
        self.base_mut().emit_signal("phim_vua_duoc_bam", &[so_phim.to_variant()]);
        self.cat_nhat_danh_sach_phim_bam();

        let id_cum: i32 = so_phim / 12 + 1;
        let so_phim_trong_cum = (so_phim % 12) + 1;
        self.goi_cum_phim_deferred(id_cum, "bam_phim_thu", &[so_phim_trong_cum.to_variant()]);
    }

    pub fn bam_phim_tu_chuot(&mut self, so_phim: i32) {
        sf2_read::bat_not_nhac(so_phim + 24, 100);
        self.bam_phim(so_phim);
    }

    pub fn tha_phim(&mut self, so_phim: i32) {
        self.danh_sach_phim_bam.remove(&so_phim);
        self.cat_nhat_danh_sach_phim_bam();
        self.base_mut().emit_signal("phim_vua_duoc_nha", &[so_phim.to_variant()]);

        let id_cum: i32 = so_phim / 12 + 1;
        let so_phim_trong_cum = (so_phim % 12) + 1;
        self.goi_cum_phim_deferred(id_cum, "tha_phim_thu", &[so_phim_trong_cum.to_variant()]);
    }

    pub fn tha_phim_tu_chuot(&mut self, so_phim: i32) {
        sf2_read::tat_not_nhac(so_phim + 24);
        self.tha_phim(so_phim);
    }

    #[func]
    pub fn kiem_tra_dau_vao(&mut self) {
        godot_print!("Ban phim is ready!");
        Os::singleton().close_midi_inputs();
        self.base_mut().set_process_input(true);
        self.base_mut().set_process_unhandled_input(true);
        Os::singleton().open_midi_inputs();
        self.midi_input = Os::singleton()
            .get_connected_midi_inputs()
            .get(0)
            .unwrap_or_else(|| GString::from(""));
        let midi_input_text = self.midi_input.clone();
        godot_print!("Midi: {}", midi_input_text.clone());
        if let Some(mut label_midi) = self.base_mut().try_get_node_as::<Label>("../midi") {
            label_midi.set_text(&midi_input_text);
        }
    }

    #[func]
    pub fn thay_doi_sf2(&mut self, duong_dan: GString) {
        self.path_sf2 = duong_dan.clone();
        let sf2_path: String = ProjectSettings::singleton().globalize_path(&duong_dan).to_string();
        let _ = sf2_read::thay_doi_sf2(&sf2_path);
    }

    pub fn kiem_tra_sf2(&mut self, duong_dan: GString) {
        self.path_sf2 = duong_dan.clone();
        let sf2_path: String = ProjectSettings::singleton().globalize_path(&duong_dan).to_string();
        match sf2_read::khoi_tao_bo_doc(&sf2_path) {
            Ok(()) => godot_print!("San sang phat SF2 tu: {}", sf2_path),
            Err(err) => godot_print!("Loi khoi tao SF2: {}", err),
        }
    }

    pub fn kiem_tra_pedal(&mut self) {
        let pedal_text: String = format!(
            "{}, {}, {}",
            if self.pedal_trai {
                "L"
            } else {
                "N"
            },
            if self.pedal_giua {
                "M"
            } else {
                "N"
            },
            if self.pedal_phai {
                "R"
            } else {
                "N"
            }
        );
        if let Some(mut label_pedal) = self.base_mut().try_get_node_as::<Label>("../pedal") {
            label_pedal.set_text(&pedal_text);
        }
    }
}

#[godot_api]
impl IHBoxContainer for BanPhimMidi {
    fn init(base: Base<HBoxContainer>) -> Self {
        Self {
            base,
            pedal_phai: false,
            pedal_giua: false,
            pedal_trai: false,
            midi_input: GString::from(""),
            cum_1: None,
            cum_2: None,
            cum_3: None,
            cum_4: None,
            cum_5: None,
            cum_6: None,
            cum_7: None,
            path_sf2: GString::from(
                format!("{}sf2/Florestan_Basic_GM_GS_Plus.sf2", lay_duong_dan_goc())
            ),
            hop_am: GString::from(""),
            danh_sach_phim_bam: HashSet::new(),
            cho_phep_bam_ban_phim: true,
            ban_do_phim_may_tinh: HashMap::new(),

            cho_phep_kiem_tra_hop_am: true,
            cho_phep_hien_thi_phim: true,
            hien_thi_ten_not_c: false,
            hien_thi_ten_not_all: false,
            cho_phep_bam_chuot: true,
        }
    }

    fn ready(&mut self) {
        godot_print!("BanPhimMidi is ready!");
        let cai_dat = crate::du_lieu_nguoi_dung::CaiDatGameJson::doc_cai_dat();
        sf2_read::cap_nhat_am_luong_piano(cai_dat.am_luong_piano);

        self.hien_thi_ten_not_c = cai_dat.hien_thi_ten_not_c;
        self.hien_thi_ten_not_all = cai_dat.hien_thi_ten_not_all;
        self.cho_phep_kiem_tra_hop_am = cai_dat.kiem_tra_hop_am;
        self.cho_phep_hien_thi_phim = cai_dat.hien_thi_phim_bam;
        self.cho_phep_bam_chuot = cai_dat.bam_bang_chuot;
        self.cho_phep_bam_ban_phim = cai_dat.bam_bang_ban_phim;
        self.ban_do_phim_may_tinh = cai_dat
            .gan_phim_midi
            .into_iter()
            .map(|(so_phim, keycode)| (keycode, so_phim))
            .collect();
        self.cum_1 = self.base().try_get_node_as::<CumPhimDan>("1");
        self.cum_2 = self.base().try_get_node_as::<CumPhimDan>("2");
        self.cum_3 = self.base().try_get_node_as::<CumPhimDan>("3");
        self.cum_4 = self.base().try_get_node_as::<CumPhimDan>("4");
        self.cum_5 = self.base().try_get_node_as::<CumPhimDan>("5");
        self.cum_6 = self.base().try_get_node_as::<CumPhimDan>("6");
        self.cum_7 = self.base().try_get_node_as::<CumPhimDan>("7");

        self.kiem_tra_dau_vao();
        self.kiem_tra_sf2(self.path_sf2.clone());

        // self.to_phim(36, Color::from_rgba(0.2, 1.0, 0.3, 0.6));

        // Gọi hàm thay đổi nhạc cụ nếu sf2_read của bạn hỗ trợ
        // sf2_read::thay_doi_nhac_cu(self.loai_piano);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.kiem_tra_phim_midi(event.clone());
        self.kiem_tra_phim_may_tinh(event);
    }

    fn exit_tree(&mut self) {
        Os::singleton().close_midi_inputs();
        godot_print!("Đóng kết nối MIDI");
    }
}
