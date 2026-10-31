use std::collections::HashMap;
use godot::classes::FileAccess;
use godot::global::godot_print;
use midly::{ MetaMessage, MidiMessage, Smf, Timing, TrackEventKind };

use crate::doc_sheet_json::{ tien_xu_ly_khuong, NotNhacJson };

pub struct MIDIRead {
    pub file_midi: String,
    pub ten_bai_hat: String,
    pub ten_tac_gia: String,
    pub ten_nguoi_tao: String,
    pub loai_vach_nhip: String,
    pub bpm: f32,
    pub khuong_1: Vec<NotNhacJson>,
    pub khuong_2: Vec<NotNhacJson>,
    pub mien_bat_dau_k1: i32,
    pub mien_bat_dau_k2: i32,
}

impl MIDIRead {
    pub fn new(
        file_midi: &str,
        ten_bai_hat: &str,
        ten_tac_gia: &str,
        ten_nguoi_tao: &str
    ) -> Self {
        Self {
            file_midi: file_midi.to_string(),
            ten_bai_hat: ten_bai_hat.to_string(),
            ten_tac_gia: ten_tac_gia.to_string(),
            ten_nguoi_tao: ten_nguoi_tao.to_string(),
            loai_vach_nhip: String::from("4/4"),
            bpm: 120.0,
            khuong_1: Vec::new(),
            khuong_2: Vec::new(),
            mien_bat_dau_k1: 36,
            mien_bat_dau_k2: 24,
        }
    }

    pub fn khoi_tao(&mut self) -> bool {
        let packed_bytes = FileAccess::get_file_as_bytes(&self.file_midi);
        let bytes: Vec<u8> = packed_bytes.to_vec();

        if bytes.is_empty() {
            godot_print!("Lỗi MIDIRead: Không thể đọc hoặc file rỗng: {}", self.file_midi);
            return false;
        }

        let smf = match Smf::parse(&bytes) {
            Ok(s) => s,
            Err(loi) => {
                godot_print!("Lỗi phân tích cấu trúc MIDI: {}", loi);
                return false;
            }
        };

        let ticks_per_beat: f32 = match smf.header.timing {
            Timing::Metrical(t) => t.as_int() as f32,
            Timing::Timecode(fps, sub) => (fps.as_f32() * (sub as f32)).max(1.0),
        };

        let mut cac_track_co_not: Vec<Vec<NotNhacJson>> = Vec::new();

        for (track_idx, track) in smf.tracks.iter().enumerate() {
            let mut tick_hien_tai: u64 = 0;
            let mut phim_dang_giu: HashMap<u8, u64> = HashMap::new();
            let mut ds_not_trong_track: Vec<NotNhacJson> = Vec::new();
            let mau_mac_dinh = if cac_track_co_not.is_empty() { "1" } else { "3" };

            for event in track {
                tick_hien_tai += event.delta.as_int() as u64;

                match event.kind {
                    TrackEventKind::Meta(meta) => match meta {
                        MetaMessage::Tempo(micro_giay_moi_nhip) => {
                            let us = micro_giay_moi_nhip.as_int() as f32;
                            if us > 0.0 {
                                self.bpm = (60_000_000.0 / us).round();
                            }
                        }
                        MetaMessage::TimeSignature(tu_so, mau_so_luy_thua, _, _) => {
                            let mau_so = 2_u32.pow(mau_so_luy_thua as u32);
                            let chuoi_nhip = format!("{}/{}", tu_so, mau_so);
                            if matches!(chuoi_nhip.as_str(), "4/4" | "3/4" | "3/8" | "2/4" | "6/8") {
                                self.loai_vach_nhip = chuoi_nhip;
                            }
                        }
                        MetaMessage::TrackName(bytes) => {
                            if track_idx == 0 && self.ten_bai_hat.is_empty() {
                                if let Ok(chuoi) = std::str::from_utf8(bytes) {
                                    let chuoi_sach = chuoi.trim();
                                    if !chuoi_sach.is_empty() {
                                        self.ten_bai_hat = chuoi_sach.to_string();
                                    }
                                }
                            }
                        }
                        MetaMessage::Copyright(bytes) => {
                            if self.ten_nguoi_tao.is_empty() {
                                if let Ok(chuoi) = std::str::from_utf8(bytes) {
                                    let chuoi_sach = chuoi.trim();
                                    if !chuoi_sach.is_empty() {
                                        self.ten_nguoi_tao = chuoi_sach.to_string();
                                    }
                                }
                            }
                        }
                        MetaMessage::Text(bytes) => {
                            if self.ten_tac_gia.is_empty() {
                                if let Ok(chuoi) = std::str::from_utf8(bytes) {
                                    let chuoi_sach = chuoi.trim();
                                    if !chuoi_sach.is_empty() {
                                        self.ten_tac_gia = chuoi_sach.to_string();
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    TrackEventKind::Midi { message, .. } => match message {
                        MidiMessage::NoteOn { key, vel } => {
                            let k = key.as_int();
                            if vel.as_int() > 0 {
                                phim_dang_giu.insert(k, tick_hien_tai);
                            } else if let Some(tick_bat_dau) = phim_dang_giu.remove(&k) {
                                if let Some(not) = Self::tao_not_tu_tick(
                                    k,
                                    tick_bat_dau,
                                    tick_hien_tai,
                                    ticks_per_beat,
                                    mau_mac_dinh
                                ) {
                                    ds_not_trong_track.push(not);
                                }
                            }
                        }
                        MidiMessage::NoteOff { key, .. } => {
                            let k = key.as_int();
                            if let Some(tick_bat_dau) = phim_dang_giu.remove(&k) {
                                if let Some(not) = Self::tao_not_tu_tick(
                                    k,
                                    tick_bat_dau,
                                    tick_hien_tai,
                                    ticks_per_beat,
                                    mau_mac_dinh
                                ) {
                                    ds_not_trong_track.push(not);
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            if !ds_not_trong_track.is_empty() {
                cac_track_co_not.push(ds_not_trong_track);
            }
        }

        if cac_track_co_not.is_empty() {
            godot_print!("Cảnh báo: File MIDI không chứa nốt nhạc nào!");
            return false;
        }

        if self.ten_bai_hat.is_empty() {
            if let Some(ten_file) = std::path::Path::new(&self.file_midi)
                .file_stem()
                .and_then(|s| s.to_str())
            {
                self.ten_bai_hat = ten_file.to_string();
            }
        }

        let (mut ds_tho_k1, mut ds_tho_k2) = if cac_track_co_not.len() >= 2 {
            let mut iter = cac_track_co_not.into_iter();
            let mut t1 = iter.next().unwrap_or_default();
            let mut t2 = iter.next().unwrap_or_default();

            let tb_t1: f32 = t1.iter().map(|n| n.id_not as f32).sum::<f32>() / (t1.len() as f32);
            let tb_t2: f32 = t2.iter().map(|n| n.id_not as f32).sum::<f32>() / (t2.len() as f32);
            if tb_t1 < tb_t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            (t1, t2)
        } else {
            let mut k1 = Vec::new();
            let mut k2 = Vec::new();
            for mut not in cac_track_co_not.into_iter().next().unwrap_or_default() {
                if not.id_not >= 36 {
                    not.mau_not = String::from("1");
                    k1.push(not);
                } else {
                    not.mau_not = String::from("3");
                    k2.push(not);
                }
            }
            (k1, k2)
        };
        // GỌI HÀM THU GỌN QUÃNG TÁM TRƯỚC KHI TIỀN XỬ LÝ
        Self::thu_gon_quang_tam_khuong(&mut ds_tho_k1);
        Self::thu_gon_quang_tam_khuong(&mut ds_tho_k2);

        let mien_goi_y_k1 = Self::tim_mien_khoi_dau(&ds_tho_k1, 36);
        let mien_goi_y_k2 = Self::tim_mien_khoi_dau(&ds_tho_k2, 24);

        
        let (mien_k1_chuan, k1_da_xu_ly) = tien_xu_ly_khuong(
            ds_tho_k1,
            mien_goi_y_k1,
            false,
            0.0,
            f32::MAX
        );
        let (mien_k2_chuan, k2_da_xu_ly) = tien_xu_ly_khuong(
            ds_tho_k2,
            mien_goi_y_k2,
            false,
            0.0,
            f32::MAX
        );

        self.mien_bat_dau_k1 = mien_k1_chuan;
        self.mien_bat_dau_k2 = mien_k2_chuan;
        self.khuong_1 = k1_da_xu_ly;
        self.khuong_2 = k2_da_xu_ly;

        true
    }
    /// Thu gọn các nốt nhảy vọt quá xa (> 16 phím) về đúng quãng tám (±12) của cụm hợp âm lân cận
    fn thu_gon_quang_tam_khuong(ds_not: &mut Vec<NotNhacJson>) {
        if ds_not.len() < 2 {
            return;
        }

        // Sắp xếp theo nhịp tăng dần trước khi xử lý
        ds_not.sort_by(|a, b| {
            a.nhip_dich
                .partial_cmp(&b.nhip_dich)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.id_not.cmp(&b.id_not))
        });

        // 1. Tại cùng 1 nhịp (hợp âm): Nếu có nốt lẻ loi cách các nốt còn lại > 12 phím -> Dịch ±12 về gần hợp âm
        let mut i = 0;
        while i < ds_not.len() {
            let nhip_ht = ds_not[i].nhip_dich;
            let mut cuoi = i;
            while cuoi < ds_not.len() && (ds_not[cuoi].nhip_dich - nhip_ht).abs() <= 0.01 {
                cuoi += 1;
            }

            if cuoi - i >= 2 {
                // Lấy trung vị (median) của hợp âm tại nhịp này
                let mut ids: Vec<i32> = ds_not[i..cuoi].iter().map(|n| n.id_not).collect();
                ids.sort();
                let trung_vi = ids[ids.len() / 2];

                for k in i..cuoi {
                    while ds_not[k].id_not < trung_vi - 12 && ds_not[k].id_not + 12 <= 83 {
                        ds_not[k].id_not += 12;
                    }
                    while ds_not[k].id_not > trung_vi + 12 && ds_not[k].id_not - 12 >= 0 {
                        ds_not[k].id_not -= 12;
                    }
                }
            }
            i = cuoi;
        }

        // 2. Giữa các nhịp liên tiếp (trong cửa sổ ±4.0 nhịp):
        // Nếu 1 nốt nhảy vọt quá xa khỏi trung vị của đoạn nhạc -> Dịch quãng tám (±12) để giữ mạch mượt mà
        let ban_sao_id: Vec<(f32, i32)> = ds_not.iter().map(|n| (n.nhip_dich, n.id_not)).collect();
        for idx in 0..ds_not.len() {
            let nhip_ht = ds_not[idx].nhip_dich;
            let mut lan_can: Vec<i32> = ban_sao_id
                .iter()
                .filter(|(nhip, _)| (nhip - nhip_ht).abs() <= 4.0)
                .map(|(_, id)| *id)
                .collect();

            if lan_can.len() >= 3 {
                lan_can.sort();
                let trung_vi_doan = lan_can[lan_can.len() / 2];

                while ds_not[idx].id_not < trung_vi_doan - 10 && ds_not[idx].id_not + 12 <= 83 {
                    ds_not[idx].id_not += 12;
                }
                while ds_not[idx].id_not > trung_vi_doan + 10 && ds_not[idx].id_not - 12 >= 0 {
                    ds_not[idx].id_not -= 12;
                }
            }
        }

        // 3. Loại bỏ các nốt bị trùng nhau (cùng id_not và cùng nhip_dich) sau khi gộp quãng tám
        let mut ds_sach: Vec<NotNhacJson> = Vec::with_capacity(ds_not.len());
        for not in ds_not.drain(..) {
            if let Some( ton_tai ) = ds_sach.iter_mut().find(|n| {
                n.id_not == not.id_not && (n.nhip_dich - not.nhip_dich).abs() <= 0.01
            }) {
                ton_tai.nhip_giu = ton_tai.nhip_giu.max(not.nhip_giu);
            } else {
                ds_sach.push(not);
            }
        }
        *ds_not = ds_sach;
    }

    fn tao_not_tu_tick(
        midi_key: u8,
        tick_bat_dau: u64,
        tick_ket_thuc: u64,
        ticks_per_beat: f32,
        mau_not: &str
    ) -> Option<NotNhacJson> {
        let id_not = (midi_key as i32) - 24;
        if !(0..=83).contains(&id_not) {
            return None;
        }

        let nhip_bat_dau_tho = (tick_bat_dau as f32) / ticks_per_beat;
        let do_dai_nhip_tho = ((tick_ket_thuc.saturating_sub(tick_bat_dau)) as f32) / ticks_per_beat;

        let nhip_dich = (nhip_bat_dau_tho * 4.0).round() / 4.0;
        let mut nhip_giu = (do_dai_nhip_tho * 4.0).round() / 4.0;

        if nhip_giu <= 0.5 {
            nhip_giu = 0.0;
        }

        Some(NotNhacJson {
            id_not,
            mau_not: mau_not.to_string(),
            nhip_dich,
            nhip_giu,
            chuyen_not: 0,
            dau_hoa: String::new(),
            ten_khac: String::new(),
        })
    }

    fn tim_mien_khoi_dau(ds_not: &[NotNhacJson], mac_dinh: i32) -> i32 {
        let Some(not_dau) = ds_not.first() else {
            return mac_dinh;
        };
        let mut min_id = not_dau.id_not;
        let mut max_id = not_dau.id_not;

        for not in ds_not.iter().take(12) {
            let tam_min = min_id.min(not.id_not);
            let tam_max = max_id.max(not.id_not);
            if tam_max - tam_min <= 16 {
                min_id = tam_min;
                max_id = tam_max;
            } else {
                break;
            }
        }
        min_id.clamp(0, 67)
    }

    // pub fn sang_bai_hat_json(&self) -> BaiHatJson {
    //     BaiHatJson {
    //         ten_bai_hat: self.ten_bai_hat.clone(),
    //         tac_gia: self.ten_tac_gia.clone(),
    //         nguoi_tao_sheet: self.ten_nguoi_tao.clone(),
    //         loai_vach_nhip: self.loai_vach_nhip.clone(),
    //         bpm: self.bpm,
    //         mien_not_bat_dau_k1: self.mien_bat_dau_k1,
    //         mien_not_bat_dau_k2: self.mien_bat_dau_k2,
    //         khuong_1: self.khuong_1.clone(),
    //         khuong_2: self.khuong_2.clone(),
    //     }
    // }
}