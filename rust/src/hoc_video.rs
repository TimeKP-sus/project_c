use godot::builtin::GString;
use godot::classes::{Control, IControl};
use godot::global::godot_print;
use godot::{
    obj::{Base}, prelude::{GodotClass, godot_api}
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MocThoiGian {
    pub thoi_gian: f32,
    pub tieu_de: String,
}

#[derive(Deserialize, Debug)]
pub struct VideoJson {
    pub ten_bai_hoc: String,
    pub nguoi_tao: String,
    pub duong_dan_video: String,
    pub cac_moc_thoi_gian: Option<Vec<MocThoiGian>>
}

#[derive(GodotClass)]
#[class(init, base=Control)] 
pub struct HocVideo {
    #[base]
    base: Base<Control>,
    duong_dan_bai_hoc: GString,
    #[export]
    duong_dan_video: GString,
    ten_bai_hoc: GString,
    nguoi_tao: GString,
}

#[godot_api]
impl HocVideo {

    pub fn get_duong_dan_bai_hoc(&self) -> GString {
        self.duong_dan_bai_hoc.clone()
    }
}

#[godot_api]
impl IControl for HocVideo {
    fn ready(&mut self) {
        godot_print!("Ready HocVideo");
    }
}