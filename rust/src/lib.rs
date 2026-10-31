// mod test;
// mod midi_node;
// mod midi_read;
mod khuong_nhac;
mod sf2_read;
mod ban_phim_midi;
mod cum_phim_dan;
mod quan_ly_sheet;
mod not_nhac;
mod hop_am;
mod choi_sheet;
mod check_ten_not;
mod doc_sheet_json;
mod du_lieu_nguoi_dung;
mod danh_sach_bai_hoc;
mod hoc_nghe_not;
mod hoc_video;
mod hoc_tim_not;
mod tao_sheet;
mod quan_ly_tao_sheet;
mod midi_read;
mod tao_tim_not;
mod tao_bai_video;
mod tao_danh_sach;
mod tao_nghe_not;
mod tao_tap_sheet;
mod cai_dat;
mod check_fps;
mod phim_tu_do;
pub mod du_lieu_lich_su;
// mod phim_dan;



use godot::prelude::*;



pub struct RustExtension;




#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}