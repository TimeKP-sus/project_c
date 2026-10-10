use godot::builtin::{Array, Dictionary, GString};
use godot::classes::{Control, FileAccess, IControl, file_access::ModeFlags};
use godot::global::godot_print;
use godot::{
    obj::Base, prelude::{GodotClass, godot_api}
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
    #[export]
    duong_dan_bai_hoc: GString,
    #[export]
    duong_dan_video: GString,
    ten_bai_hoc: GString,
    nguoi_tao: GString,
    cac_moc_thoi_gian: Option<Vec<MocThoiGian>>,
}

#[godot_api]
impl HocVideo {
    /// Hàm này được GDScript gọi để truyền đường dẫn file JSON vào
    #[func]
    pub fn doc_file_json(&mut self, duong_dan_file: GString) -> bool {
        let file_access = FileAccess::open(&duong_dan_file.clone(), ModeFlags::READ);
        
        if let Some(file) = file_access {
            let noi_dung = file.get_as_text().to_string();
            
            match serde_json::from_str::<VideoJson>(&noi_dung) {
                Ok(data) => {
                    self.ten_bai_hoc = GString::from(data.ten_bai_hoc);
                    self.nguoi_tao = GString::from(data.nguoi_tao);
                    self.duong_dan_video = GString::from(data.duong_dan_video);
                    self.cac_moc_thoi_gian = data.cac_moc_thoi_gian;
                    return true;
                }
                Err(e) => {
                    godot_print!("Lỗi parse JSON: {}", e);
                    return false;
                }
            }
        }
        godot_print!("Không thể mở file: {}", duong_dan_file);
        false
    }

    #[func]
    pub fn get_ten_bai_hoc(&self) -> GString {
        self.ten_bai_hoc.clone()
    }

    #[func]
    pub fn get_nguoi_tao(&self) -> GString {
        self.nguoi_tao.clone()
    }

    /// Trả về Array chứa Dictionary để GDScript có thể loop và lấy data
    #[func]
    pub fn get_cac_moc_thoi_gian(&self) -> Array<Dictionary> {
        let mut array: Array<Dictionary> = Array::new();
        if let Some(ref mocs) = self.cac_moc_thoi_gian {
            for moc in mocs {
                let mut dict: Dictionary = Dictionary::new();
                let _ = dict.insert("thoi_gian", moc.thoi_gian as f64);
                let _ = dict.insert("tieu_de", GString::from(&moc.tieu_de));
                array.push(&dict);
            }
        }
        
        array
    }
    #[func]
    pub fn khoi_dong_bai_hoc(&mut self) {
        let path: GString = self.duong_dan_bai_hoc.clone();
        if !path.is_empty() {
            self.doc_file_json(path);
        }
    }
}

#[godot_api]
impl IControl for HocVideo {
    fn ready(&mut self) {
        godot_print!("Ready HocVideo (Rust)");
    }
}