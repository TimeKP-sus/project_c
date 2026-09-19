use std::collections::HashSet;

use crate::{hop_am::HopAm, sf2_read};
use crate::cum_phim_dan::CumPhimDan;
use godot::meta::ToGodot;
use godot::{
    builtin::GString,
    classes::{
        HBoxContainer, IHBoxContainer, InputEvent, InputEventMidi, Label, Os, ProjectSettings,
    },
    global::godot_print,
    obj::{Base, Gd, WithBaseField},
    register::{GodotClass, godot_api},
};


#[derive(GodotClass)]
#[class(base=HBoxContainer)]
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
    hop_am:GString,
    danh_sach_phim_bam: HashSet<i32>
}
#[godot_api]
impl BanPhimMidi {
    #[signal]
    fn phim_vua_duoc_bam(so_phim: i32);
    #[signal]
    fn phim_vua_duoc_nha(so_phim: i32);
    // pub fn get_danh_sach_phim_bam(&self) -> HashSet<i32> {
    //     self.danh_sach_phim_bam.clone()
    // }
    fn cat_nhat_hop_am(&mut self) {
        let hop_am: String = HopAm::kiem_tra(&self.danh_sach_phim_bam);
        self.hop_am = GString::from(hop_am.clone());
        if let Some(mut label_hop_am) = self.base_mut().try_get_node_as::<Label>("../hop_am") {
            label_hop_am.set_text(&hop_am);
        }
    }
    fn kiem_tra_phim_midi(&mut self, event: Gd<InputEvent>) {
        // godot_print!("kiem tra midi");
        if let Ok(midi_event) = event.try_cast::<InputEventMidi>() {
            let tha_phim: godot::global::MidiMessage = midi_event.get_message();
            let nut: i32 = midi_event.get_pitch();
            let luc: i32 = midi_event.get_velocity();

            match tha_phim {
                godot::global::MidiMessage::NOTE_ON if luc > 0 => {
                    self.bam_phim(nut - 24);
                    sf2_read::bat_not_nhac(nut, luc);
                }
                godot::global::MidiMessage::NOTE_ON if luc <= 0 => {
                    self.tha_phim(nut - 24);
                    sf2_read::tat_not_nhac(nut);
                }
                godot::global::MidiMessage::NOTE_OFF => {
                    self.tha_phim(nut - 24);
                    sf2_read::tat_not_nhac(nut);
                }
                godot::global::MidiMessage::CONTROL_CHANGE => {
                    let id_pedals: i32 = midi_event.get_controller_number();
                    let gia_tri_pedal: i32 = midi_event.get_controller_value();
                    sf2_read::thay_doi_dieu_khien(id_pedals, gia_tri_pedal);
                    self.pedal_phai = id_pedals == 64 && gia_tri_pedal > 0;
                    self.pedal_giua = id_pedals == 66 && gia_tri_pedal > 0;
                    self.pedal_trai = id_pedals == 67 && gia_tri_pedal > 0;
                    self.kiem_tra_pedal();
                    // godot_print!(
                    //     "Dieu khien thay doi: Id Pedal: {}, Gia tri: {}",
                    //     id_pedals,
                    //     gia_tri_pedal
                    // );
                }
                _ => {}
            }
        }
    }
    // pub fn tim_so_phim_trong_cum(&mut self, so_phim: i32) -> i32 {
    //     let id_cum: i32 = (so_phim / 12) + 1;
    //     let so_phim_trong_cum: i32 = so_phim - (id_cum * 7);
    //     return so_phim_trong_cum;
    // }
    pub fn cat_nhat_danh_sach_phim_bam(&mut self) {
        let mut danh_sach: Vec<i32> = self.danh_sach_phim_bam.iter().cloned().collect();
        danh_sach.sort();
        let danh_sach_str: Vec<String> = danh_sach.iter().map(|&num| num.to_string()).collect();
        let ket_qua: String = danh_sach_str.join(" ");
        if let Some(mut label_danh_sach) = self.base_mut().try_get_node_as::<Label>("../cac_phim") {
            label_danh_sach.set_text(&ket_qua);
        }
        self.cat_nhat_hop_am();
    }
    pub fn bam_phim(&mut self, so_phim: i32) {
        // godot_print!("Nhan phim MIDI: {}, Luc: {}", so_phim, 100);
        self.danh_sach_phim_bam.insert(so_phim);

        self.base_mut().emit_signal("phim_vua_duoc_bam", &[so_phim.to_variant()]);

        self.cat_nhat_danh_sach_phim_bam();
        let id_cum: i32 = (so_phim / 12) + 1;
        let so_phim_trong_cum = (so_phim % 12) + 1;
        match id_cum {
            1 => {
                if let Some(cum) = &mut self.cum_1 {
                    cum.bind_mut().bam_phim_thu(so_phim_trong_cum);
                }
            }
            2 => {
                if let Some(cum) = &mut self.cum_2 {
                    cum.bind_mut().bam_phim_thu(so_phim_trong_cum);
                }
            }
            3 => {
                if let Some(cum) = &mut self.cum_3 {
                    cum.bind_mut().bam_phim_thu(so_phim_trong_cum);
                }
            }
            4 => {
                if let Some(cum) = &mut self.cum_4 {
                    cum.bind_mut().bam_phim_thu(so_phim_trong_cum);
                }
            }
            5 => {
                if let Some(cum) = &mut self.cum_5 {
                    cum.bind_mut().bam_phim_thu(so_phim_trong_cum);
                }
            }
            6 => {
                if let Some(cum) = &mut self.cum_6 {
                    cum.bind_mut().bam_phim_thu(so_phim_trong_cum);
                }
            }
            7 => {
                if let Some(cum) = &mut self.cum_7 {
                    cum.bind_mut().bam_phim_thu(so_phim_trong_cum);
                }
            }
            _ => {}
        }
    }
    pub fn tha_phim(&mut self, so_phim: i32) {
        // godot_print!("Nhan phim MIDI: {}, Luc: {}", so_phim, 0);
        self.danh_sach_phim_bam.remove(&so_phim);
        self.cat_nhat_danh_sach_phim_bam();
        //signal
        self.base_mut().emit_signal("phim_vua_duoc_nha", &[so_phim.to_variant()]);
        // self.phim_dang_bam[so_phim as usize] = false;
        let id_cum: i32 = (so_phim / 12) + 1;
        let so_phim_trong_cum = (so_phim % 12) + 1;
        match id_cum {
            1 => {
                if let Some(cum) = &mut self.cum_1 {
                    cum.bind_mut().tha_phim_thu(so_phim_trong_cum);
                }
            }
            2 => {
                if let Some(cum) = &mut self.cum_2 {
                    cum.bind_mut().tha_phim_thu(so_phim_trong_cum);
                }
            }
            3 => {
                if let Some(cum) = &mut self.cum_3 {
                    cum.bind_mut().tha_phim_thu(so_phim_trong_cum);
                }
            }
            4 => {
                if let Some(cum) = &mut self.cum_4 {
                    cum.bind_mut().tha_phim_thu(so_phim_trong_cum);
                }
            }
            5 => {
                if let Some(cum) = &mut self.cum_5 {
                    cum.bind_mut().tha_phim_thu(so_phim_trong_cum);
                }
            }
            6 => {
                if let Some(cum) = &mut self.cum_6 {
                    cum.bind_mut().tha_phim_thu(so_phim_trong_cum);
                }
            }
            7 => {
                if let Some(cum) = &mut self.cum_7 {
                    cum.bind_mut().tha_phim_thu(so_phim_trong_cum);
                }
            }
            _ => {}
        }
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
        let sf2_path: String = ProjectSettings::singleton()
            .globalize_path(&duong_dan)
            .to_string();
        // godot_print!("Thay doi SF2: {}", sf2_path);
        let _ = sf2_read::thay_doi_sf2(&sf2_path);
    }
    pub fn kiem_tra_sf2(&mut self, duong_dan: GString) {
        self.path_sf2 = duong_dan.clone();
        let sf2_path: String = ProjectSettings::singleton()
            .globalize_path(&duong_dan)
            .to_string();
        // godot_print!("{}", sf2_path);
        match sf2_read::khoi_tao_bo_doc(&sf2_path) {
            Ok(()) => godot_print!("San sang phat SF2 tu: {}", sf2_path),
            Err(err) => godot_print!("Loi khoi tao SF2: {}", err),
        }
    }
    pub fn kiem_tra_pedal(&mut self) {
        let pedal_text: String = format!(
            "{}, {}, {}",
            if self.pedal_trai { "L" } else { "N" },
            if self.pedal_giua { "M" } else { "N" },
            if self.pedal_phai { "R" } else { "N" }
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
            path_sf2: GString::from("res://sf2/Florestan_Basic_GM_GS_Plus.sf2"),
            hop_am: GString::from(""),
            danh_sach_phim_bam: HashSet::new(),
        }
    }
    fn ready(&mut self) {
        godot_print!("BanPhimMidi is ready!");
        // try_get_node_as cực kỳ an toàn, không có thì nó trả về None chứ không crash
        self.cum_1 = self.base().try_get_node_as::<CumPhimDan>("1");
        self.cum_2 = self.base().try_get_node_as::<CumPhimDan>("2");
        self.cum_3 = self.base().try_get_node_as::<CumPhimDan>("3");
        self.cum_4 = self.base().try_get_node_as::<CumPhimDan>("4");
        self.cum_5 = self.base().try_get_node_as::<CumPhimDan>("5");
        self.cum_6 = self.base().try_get_node_as::<CumPhimDan>("6");
        self.cum_7 = self.base().try_get_node_as::<CumPhimDan>("7");

        self.kiem_tra_dau_vao();
        self.kiem_tra_sf2(self.path_sf2.clone());
    }
    fn input(&mut self, event: Gd<InputEvent>) {
        self.kiem_tra_phim_midi(event);
    }

    // fn unhandled_input(&mut self, event: Gd<InputEvent>) {
    //     self.kiem_tra_phim_midi(event);
    // }

    fn exit_tree(&mut self) {
        Os::singleton().close_midi_inputs();
        godot_print!("Đóng kết nối MIDI");
    }
}
