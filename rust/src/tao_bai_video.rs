use std::time::SystemTime;

use godot::{
    builtin::{ Callable, GString, PackedStringArray },
    classes::{
        Button,
        Control,
        DirAccess,
        FileDialog,
        IControl,
        ItemList,
        LineEdit,
        ProjectSettings,
        SpinBox,
    },
    global::{ Error, godot_print },
    meta::ToGodot,
    obj::{ Base, WithBaseField },
    prelude::{ GodotClass, godot_api },
};

use crate::{
    doc_sheet_json::{ BaiHocVideoJson, MocThoiGianJson, doc_file_video_json, luu_file_video_json },
    thong_bao,
};

#[derive(GodotClass)]
#[class(base = Control)]
pub struct TaoVideo {
    #[base]
    base: Base<Control>,

    ten_bai_hoc: GString,
    nguoi_tao: GString,
    duong_dan_video: GString,

    /// Bật/tắt chế độ Sửa trực tiếp mục đang chọn (false = Chế độ Thêm mới, true = Chế độ Sửa)
    che_do_sua: bool,
    vi_tri_dang_sua: i32,

    cac_moc: Vec<MocThoiGianJson>,
}

#[godot_api]
impl TaoVideo {
    fn hien_thong_bao(&self, noi_dung: &str, loai: &str) {
        if let Some(mut tb) = self.base().try_get_node_as::<thong_bao::ThongBao>("ThongBao") {
            let noi_dung = GString::from(noi_dung);
            match loai {
                "loi" => tb.bind_mut().thong_bao_loi(noi_dung),
                "canh_bao" => tb.bind_mut().thong_bao_canh_bao(noi_dung),
                "thong_tin" => tb.bind_mut().thong_bao_thong_tin(noi_dung),
                _ => tb.bind_mut().thong_bao_thanh_cong(noi_dung),
            }
        }
    }

    // ==================== GETTERS & SETTERS ====================
    #[func]
    pub fn set_ten_bai_hoc(&mut self, ten: GString) {
        self.ten_bai_hoc = ten.clone();
        if
            let Some(mut input) = self
                .base_mut()
                .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangTen/TenBai_Input")
        {
            input.set_text(&ten);
        }
    }

    #[func]
    pub fn get_ten_bai_hoc(&self) -> GString {
        if
            let Some(input) = self
                .base()
                .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangTen/TenBai_Input")
        {
            return input.get_text();
        }
        self.ten_bai_hoc.clone()
    }

    #[func]
    pub fn set_nguoi_tao(&mut self, nguoi: GString) {
        self.nguoi_tao = nguoi.clone();
        if
            let Some(mut input) = self
                .base_mut()
                .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangTen/NguoiTao_Input")
        {
            input.set_text(&nguoi);
        }
    }

    #[func]
    pub fn get_nguoi_tao(&self) -> GString {
        if
            let Some(input) = self
                .base()
                .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangTen/NguoiTao_Input")
        {
            return input.get_text();
        }
        self.nguoi_tao.clone()
    }

    #[func]
    pub fn set_duong_dan_video(&mut self, path: GString) {
        self.duong_dan_video = path.clone();
        if
            let Some(mut input) = self
                .base_mut()
                .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangVideo/DuongDanVideo_Input")
        {
            input.set_text(&path);
        }
    }

    #[func]
    pub fn get_duong_dan_video(&self) -> GString {
        if
            let Some(input) = self
                .base()
                .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangVideo/DuongDanVideo_Input")
        {
            return input.get_text();
        }
        self.duong_dan_video.clone()
    }

    // ==================== CHẾ ĐỘ BẬT/TẮT THÊM HOẶC SỬA ====================
    #[func]
    pub fn set_che_do_sua(&mut self, bat: bool) {
        self.che_do_sua = bat;
        if !bat {
            self.vi_tri_dang_sua = -1;
        }
        self.cap_nhat_nhan_nut_che_do();
    }

    #[func]
    pub fn get_che_do_sua(&self) -> bool {
        self.che_do_sua
    }

    #[func]
    pub fn dao_che_do_sua(&mut self) {
        let moi = !self.che_do_sua;
        self.set_che_do_sua(moi);
    }

    fn cap_nhat_nhan_nut_che_do(&mut self) {
        let dang_sua = self.che_do_sua && self.vi_tri_dang_sua >= 0;
        if
            let Some(mut btn) = self
                .base_mut()
                .try_get_node_as::<Button>("KhungChinh/VBox/HangThemMoc/ThemMoc_Btn")
        {
            if dang_sua {
                btn.set_text("✎ Lưu sửa mốc");
            } else {
                btn.set_text("➕ Thêm mốc");
            }
        }
    }

    // ==================== LẤY THÔNG TIN CƠ BẢN ====================
    #[func]
    pub fn lay_so_luong_moc(&self) -> i32 {
        self.cac_moc.len() as i32
    }

    #[func]
    pub fn kiem_tra_hop_le(&self) -> bool {
        !self.get_ten_bai_hoc().to_string().trim().is_empty() &&
            !self.get_duong_dan_video().to_string().trim().is_empty()
    }

    #[func]
    pub fn lay_thong_tin_co_ban(&self) -> GString {
        GString::from(
            format!(
                "Bài Video: {} | Tác giả: {} | Số mốc: {} | Chế độ: {}",
                self.get_ten_bai_hoc(),
                self.get_nguoi_tao(),
                self.cac_moc.len(),
                if self.che_do_sua {
                    "SỬA"
                } else {
                    "THÊM"
                }
            )
        )
    }

    // ==================== THÊM / SỬA / XÓA MỐC THỜI GIAN ====================
    #[func]
    pub fn them_hoac_sua_moc(&mut self, thoi_gian: i32, tieu_de: GString) {
        let td = tieu_de.to_string().trim().to_string();
        if td.is_empty() {
            return;
        }

        if
            self.che_do_sua &&
            self.vi_tri_dang_sua >= 0 &&
            (self.vi_tri_dang_sua as usize) < self.cac_moc.len()
        {
            let idx = self.vi_tri_dang_sua as usize;
            self.cac_moc[idx].thoi_gian = thoi_gian;
            self.cac_moc[idx].tieu_de = td;
            self.set_che_do_sua(false);
        } else if let Some(moc) = self.cac_moc.iter_mut().find(|m| m.thoi_gian == thoi_gian) {
            moc.tieu_de = td;
        } else {
            self.cac_moc.push(MocThoiGianJson {
                thoi_gian,
                tieu_de: td,
            });
        }
        self.cap_nhat_ui_danh_sach_moc();
    }

    #[func]
    pub fn xoa_moc_tai(&mut self, vi_tri: i32) {
        if vi_tri >= 0 && (vi_tri as usize) < self.cac_moc.len() {
            self.cac_moc.remove(vi_tri as usize);
            self.set_che_do_sua(false);
            self.cap_nhat_ui_danh_sach_moc();
        }
    }

    #[func]
    pub fn cap_nhat_ui_danh_sach_moc(&mut self) {
        self.cac_moc.sort_by_key(|m| m.thoi_gian);
        let cac_moc = self.cac_moc.clone();
        if
            let Some(mut list) = self
                .base_mut()
                .try_get_node_as::<ItemList>("KhungChinh/VBox/DanhSachMoc")
        {
            list.clear();
            for m in &cac_moc {
                let hien_thi = format!("⏱ {} giây  —  {}", m.thoi_gian, m.tieu_de);
                list.add_item(&hien_thi);
            }
        }
    }

    // ==================== CALLBACKS TỪ GIAO DIỆN ====================
    #[func]
    fn _khi_bam_them_moc(&mut self) {
        let tg = self
            .base()
            .try_get_node_as::<SpinBox>("KhungChinh/VBox/HangThemMoc/ThoiGian_Spin")
            .map(|s| s.get_value() as i32)
            .unwrap_or(0);
        let td = self
            .base()
            .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangThemMoc/TieuDeMoc_Input")
            .map(|l| l.get_text())
            .unwrap_or_default();

        self.them_hoac_sua_moc(tg, td);

        if
            let Some(mut l) = self
                .base_mut()
                .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangThemMoc/TieuDeMoc_Input")
        {
            l.set_text("");
        }
    }

    #[func]
    fn _khi_chon_moc_trong_list(&mut self, idx: i64) {
        let i = idx as usize;
        if i < self.cac_moc.len() {
            let tg = self.cac_moc[i].thoi_gian;
            let td = self.cac_moc[i].tieu_de.clone();
            self.vi_tri_dang_sua = idx as i32;
            self.che_do_sua = true;
            self.cap_nhat_nhan_nut_che_do();

            if
                let Some(mut spin) = self
                    .base_mut()
                    .try_get_node_as::<SpinBox>("KhungChinh/VBox/HangThemMoc/ThoiGian_Spin")
            {
                spin.set_value(tg as f64);
            }
            if
                let Some(mut input) = self
                    .base_mut()
                    .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangThemMoc/TieuDeMoc_Input")
            {
                input.set_text(&td);
            }
        }
    }
    #[func]
    pub fn khi_keo_tha_file(&mut self, danh_sach_file: PackedStringArray) {
        if danh_sach_file.is_empty() {
            return;
        }

        if let Some(duong_dan) = danh_sach_file.get(0) {
            let duong_dan_str: String = duong_dan.to_string().to_lowercase();
            if
                duong_dan_str.ends_with(".mp4") ||
                duong_dan_str.ends_with(".ogv") ||
                duong_dan_str.ends_with(".webm")
            {
                godot_print!("Đã kéo thả file Video: {}", duong_dan);
                self.hien_thong_bao("Đã chọn file video.", "thanh_cong");
                // Cập nhật lên ô Input
                self.set_duong_dan_video(ProjectSettings::singleton().localize_path(&duong_dan));
                // Gọi hàm GDScript để bắt đầu phát video
                self.base_mut().call(
                    "load_va_phat_video",
                    &[ProjectSettings::singleton().localize_path(&duong_dan).to_variant()]
                );
            } else if
                // Xử lý nếu file kéo vào là file JSON bài học
                duong_dan_str.ends_with(".json")
            {
                godot_print!("Đã kéo thả file JSON bài học video: {}", duong_dan);
                self.hien_thong_bao("Đang mở bài học video từ file JSON...", "thong_tin");
                self.mo_bai_hoc_json(duong_dan);
            } else {
                godot_print!(
                    "Định dạng file không hỗ trợ! Vui lòng thả file Video (.mp4) hoặc JSON (.json)."
                );
                self.hien_thong_bao(
                    "Định dạng file không hỗ trợ. Hãy chọn Video hoặc JSON.",
                    "canh_bao"
                );
            }
        }
    }

    #[func]
    fn _khi_bam_xoa_moc(&mut self) {
        let chon = self
            .base()
            .try_get_node_as::<ItemList>("KhungChinh/VBox/DanhSachMoc")
            .map(|mut l| l.get_selected_items())
            .unwrap_or_default();
        if !chon.is_empty() {
            if let Some(idx) = chon.get(0) {
                self.xoa_moc_tai(idx);
            }
        }
    }

    #[func]
    fn _khi_chon_file_video(&mut self, path: GString) {
        self.set_duong_dan_video(ProjectSettings::singleton().localize_path(&path));
        self.base_mut().call(
            "load_va_phat_video",
            &[ProjectSettings::singleton().localize_path(&path).to_variant()]
        );
    }

    #[func]
    pub fn luu_bai_hoc_json(&mut self, path: GString) -> bool {
        let duong_dan_goc: String = self.get_duong_dan_video().to_string().trim().to_string();

        // 1. NẾU VIDEO ĐÃ NẰM SẴN TRONG THƯ MỤC CHUẨN: Giữ nguyên đường dẫn, không copy
        if duong_dan_goc.is_empty() || duong_dan_goc.starts_with("res://bai_hoc/video") {
            return self.thuc_hien_luu_json(path, GString::from(duong_dan_goc));
        }

        // 2. NẾU VIDEO TỪ BÊN NGOÀI: Copy vào và đổi tên thành mã random
        self.hien_thong_bao("Đang tiến hành sao chép video, vui lòng đợi...", "thong_tin");

        let thu_muc_dich = "res://bai_hoc/video";
        
        // Tạo mã ID ngẫu nhiên từ thời gian thực
        let id_ngau_nhien: u64 = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Lấy đuôi file gốc (vd: mp4, webm)
        let duoi_file = duong_dan_goc.rsplit('.').next().unwrap_or("mp4");

        // Chuẩn hóa tên bài học (vd: "Bài 1" thành "bai_1")
        let ten_bai = self.get_ten_bai_hoc().to_string().trim().replace(" ", "_").to_lowercase();
        
        // Ghép thành tên file hoàn chỉnh (Tên_ID.đuôi hoặc video_ID.đuôi)
        let ten_file = if ten_bai.is_empty() {
            format!("video_{}.{}", id_ngau_nhien, duoi_file)
        } else {
            format!("{}_{}.{}", ten_bai, id_ngau_nhien, duoi_file)
        };

        let duong_dan_dich = format!("{}/{}", thu_muc_dich, ten_file);
        
        // Thực hiện lệnh copy
        let err: Error = DirAccess::copy_absolute(
            &GString::from(&duong_dan_goc),
            &GString::from(&duong_dan_dich)
        );

        // Xử lý sau khi copy
        if err == Error::OK {
            self._xu_ly_copy_thanh_cong(path, GString::from(duong_dan_dich));
            true
        } else {
            self._xu_ly_copy_that_bai(path, GString::from(duong_dan_goc));
            false
        }
    }


    /// Hàm nội bộ được gọi từ Thread sau khi copy video thành công
    #[func]
    fn _xu_ly_copy_thanh_cong(&mut self, path_json: GString, duong_dan_video_moi: GString) {
        self.set_duong_dan_video(duong_dan_video_moi.clone());
        self.thuc_hien_luu_json(path_json, duong_dan_video_moi);
    }

    /// Hàm nội bộ được gọi từ Thread nếu copy video thất bại (Sẽ lưu JSON với đường dẫn cũ)
    #[func]
    fn _xu_ly_copy_that_bai(&mut self, path_json: GString, duong_dan_video_goc: GString) {
        godot_print!("Lỗi sao chép video! Vẫn tiếp tục lưu JSON với đường dẫn video cũ.");
        self.hien_thong_bao("Copy video thất bại! Giữ nguyên đường dẫn cũ.", "canh_bao");
        self.thuc_hien_luu_json(path_json, duong_dan_video_goc);
    }

    /// Hàm lưu file JSON cuối cùng (Được dùng chung)
    fn thuc_hien_luu_json(
        &mut self,
        path_json: GString,
        duong_dan_video_chinh_thuc: GString
    ) -> bool {
        let data = BaiHocVideoJson {
            ten_bai_hoc: self.get_ten_bai_hoc().to_string().trim().to_string(),
            nguoi_tao: self.get_nguoi_tao().to_string().trim().to_string(),
            duong_dan_video: duong_dan_video_chinh_thuc.to_string(),
            cac_moc_thoi_gian: self.cac_moc.clone(),
        };

        let ket_qua = luu_file_video_json(&path_json.to_string(), &data);
        self.hien_thong_bao(
            if ket_qua {
                "Đã lưu bài học video thành công."
            } else {
                "Lỗi không thể lưu file JSON bài học video!"
            },
            if ket_qua {
                "thanh_cong"
            } else {
                "loi"
            }
        );
        ket_qua
    }

    #[func]
    pub fn mo_bai_hoc_json(&mut self, path: GString) -> bool {
        if let Some(data) = doc_file_video_json(&path.to_string()) {
            self.set_ten_bai_hoc(GString::from(&data.ten_bai_hoc));
            self.set_nguoi_tao(GString::from(&data.nguoi_tao));

            let path_video: GString = GString::from(&data.duong_dan_video);
            self.set_duong_dan_video(path_video.clone());

            self.cac_moc = data.cac_moc_thoi_gian;
            self.set_che_do_sua(false);
            self.cap_nhat_ui_danh_sach_moc();

            self.base_mut().call("load_va_phat_video", &[path_video.to_variant()]);
            godot_print!("Đã mở bài học từ file JSON: {}", path);
            self.hien_thong_bao("Đã mở bài học video thành công.", "thanh_cong");
            return true;
        }
        self.hien_thong_bao("Không thể mở bài học video từ file JSON.", "loi");
        false
    }

    #[func]
    fn _mo_popup_video(&mut self) {
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiChonVideo") {
            d.popup_centered();
        }
    }
    #[func]
    fn _mo_popup_luu(&mut self) {
        let id_ngau_nhien = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let ten_bai: String = self
            .get_ten_bai_hoc()
            .to_string()
            .trim()
            .replace(" ", "_")
            .to_lowercase();

        let ten_file_moi: String = if ten_bai.is_empty() {
            format!("video_{}.json", id_ngau_nhien)
        } else {
            format!("{}_{}.json", ten_bai, id_ngau_nhien)
        };

        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiLuuJson") {
            // Đảm bảo thư mục đích TỒN TẠI trước khi mở hộp thoại
            let thu_muc_json = "res://bai_hoc/xem_video";
            let _ = DirAccess::make_dir_recursive_absolute(&GString::from(thu_muc_json));
            d.set_current_dir(thu_muc_json);
            d.set_current_file(&GString::from(&ten_file_moi));
            d.popup_centered();
        }
    }

    #[func]
    fn _mo_popup_mo(&mut self) {
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiMoJson") {

            let thu_muc_json = "res://bai_hoc/xem_video";
            let _ = DirAccess::make_dir_recursive_absolute(&GString::from(thu_muc_json));
            d.set_current_dir(thu_muc_json);
            d.popup_centered();
        }
    }

    #[func]
    fn _ve_menu_chinh(&mut self) {
        if let Some(mut tree) = self.base().get_tree() {
            tree.change_scene_to_file("res://scene/menu/MenuChinh.tscn");
        }
    }
}

#[godot_api]
impl IControl for TaoVideo {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            ten_bai_hoc: GString::new(),
            nguoi_tao: GString::from("timekp"),
            duong_dan_video: GString::from("res://bai_hoc/video/v1.mp4"),
            che_do_sua: false,
            vi_tri_dang_sua: -1,
            cac_moc: Vec::new(),
        }
    }

    fn ready(&mut self) {
        let obj = self.to_gd();
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/MoJson_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "_mo_popup_mo"));
        }
        if
            let Some(mut b) = self
                .base_mut()
                .try_get_node_as::<Button>("ThanhTren/HBox/LuuJson_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "_mo_popup_luu"));
        }
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/VeMenu_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "_ve_menu_chinh"));
        }
        if
            let Some(mut b) = self
                .base_mut()
                .try_get_node_as::<Button>("KhungChinh/VBox/HangVideo/ChonVideo_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "_mo_popup_video"));
        }
        if
            let Some(mut b) = self
                .base_mut()
                .try_get_node_as::<Button>("KhungChinh/VBox/HangThemMoc/ThemMoc_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "_khi_bam_them_moc"));
        }
        if
            let Some(mut b) = self
                .base_mut()
                .try_get_node_as::<Button>("KhungChinh/VBox/HangThemMoc/XoaMoc_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "_khi_bam_xoa_moc"));
        }
        if
            let Some(mut list) = self
                .base_mut()
                .try_get_node_as::<ItemList>("KhungChinh/VBox/DanhSachMoc")
        {
            list.connect(
                "item_selected",
                &Callable::from_object_method(&obj, "_khi_chon_moc_trong_list")
            );
        }
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiChonVideo") {
            d.connect("file_selected", &Callable::from_object_method(&obj, "_khi_chon_file_video"));
        }
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiLuuJson") {
            d.connect("file_selected", &Callable::from_object_method(&obj, "luu_bai_hoc_json"));
        }
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiMoJson") {
            d.connect("file_selected", &Callable::from_object_method(&obj, "mo_bai_hoc_json"));
        }
        // Bắt sự kiện kéo thả file từ hệ điều hành vào cửa sổ Godot
        if let Some(mut window) = self.base().get_window() {
            window.connect(
                "files_dropped",
                &Callable::from_object_method(&obj, "khi_keo_tha_file")
            );
        }
    }
}
