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
// mod phim_dan;

use godot::prelude::*;



pub struct RustExtension;




#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}