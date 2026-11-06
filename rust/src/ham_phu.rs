use godot::classes::Os;
use godot::builtin::GString;
use std::path::Path;

/// Lấy đường dẫn gốc linh hoạt cho cả Editor và file .exe đã xuất
pub fn lay_duong_dan_goc() -> String {
    let os = Os::singleton();
    if os.has_feature(&GString::from("editor")) {
        String::from("res://")
    } else {
        let executable_path: String = os.get_executable_path().to_string();
        let base_dir = Path::new(&executable_path)
            .parent()
            .map(|path| path.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();
        format!("{}/", base_dir)
    }
}