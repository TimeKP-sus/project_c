// mod test;
// mod midi_node;
// mod midi_read;
mod ban_phim_midi;
mod cai_dat;
mod check_fps;
mod check_ten_not;
mod choi_sheet;
mod cum_phim_dan;
mod danh_sach_bai_hoc;
mod doc_sheet_json;
pub mod du_lieu_lich_su;
mod du_lieu_nguoi_dung;
mod hoc_nghe_not;
mod hoc_tim_not;
mod hoc_video;
mod hop_am;
mod khuong_nhac;
mod midi_read;
mod not_nhac;
mod phim_tu_do;
mod quan_ly_sheet;
mod quan_ly_tao_sheet;
mod sf2_read;
mod tao_bai_video;
mod tao_danh_sach;
mod tao_nghe_not;
mod tao_sheet;
mod tao_tap_sheet;
mod tao_tim_not;
// mod phim_dan;
pub mod thong_bao;

use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
