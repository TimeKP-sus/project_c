use std::collections::BTreeMap;

use godot::{
    builtin::{ Callable, GString, Variant }, classes::{
        Button,
        CheckButton,
        Control,
        FileDialog,
        IControl,
        ItemList,
        Label,
        LineEdit,
        OptionButton,
        PackedScene,
        ProjectSettings,
        SpinBox,
    }, obj::{ Base, Gd, WithBaseField }, prelude::{ GodotClass, godot_api }, tools::load,
};

use crate::doc_sheet_json::{ doc_file_json, doc_file_luyen_tap, luu_file_luyen_tap, LuyenTapJson };

#[derive(GodotClass)]
#[class(base = Control)]
pub struct TaoTapLuyenSheet {
    #[base]
    base: Base<Control>,

    ten_bai_hoc: GString,
    duong_dan_sheet: GString,
    khuong: GString,
    cho_phep_cho: bool,
    nhip_bat_dau: f32,
    nhip_ket_thuc: f32,

    /// Bật/tắt chế độ sửa ghi chú đang chọn
    che_do_sua: bool,
    khoa_dang_sua: String,

    cac_ghi_chu: BTreeMap<String, String>,
    bpm_tuy_chinh: f32,
    #[export]
    tu_choi_thu_quay_ve: bool,
}

#[godot_api]
impl TaoTapLuyenSheet {
    #[signal]
    fn mo_file();
    // ==================== GETTERS & SETTERS ====================
    #[func]
    pub fn get_bpm_tuy_chinh(&self) -> f32 {
        if
            let Some(spin) = self
                .base()
                .try_get_node_as::<SpinBox>("KhungChinh/VBox/HangCauHinh/Bpm_Spin")
        {
            return spin.get_value() as f32;
        }
        self.bpm_tuy_chinh
    }

    #[func]
    pub fn set_bpm_tuy_chinh(&mut self, bpm: f32) {
        self.bpm_tuy_chinh = bpm;
        if
            let Some(mut spin) = self
                .base_mut()
                .try_get_node_as::<SpinBox>("KhungChinh/VBox/HangCauHinh/Bpm_Spin")
        {
            spin.set_value(bpm as f64);
        }
    }
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
    pub fn set_duong_dan_sheet(&mut self, path: GString) {
        self.duong_dan_sheet = path.clone();
        if
            let Some(mut input) = self
                .base_mut()
                .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangSheet/DuongDanSheet_Input")
        {
            input.set_text(&path);
        }
        self.doc_thong_tin_sheet_goc(path);
    }

    #[func]
    pub fn get_duong_dan_sheet(&self) -> GString {
        if
            let Some(input) = self
                .base()
                .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangSheet/DuongDanSheet_Input")
        {
            return input.get_text();
        }
        self.duong_dan_sheet.clone()
    }

    #[func]
    pub fn set_khuong(&mut self, kh: GString) {
        self.khuong = kh.clone();
        let idx = match kh.to_string().as_str() {
            "1" => 1,
            "2" => 2,
            _ => 0,
        };
        if
            let Some(mut opt) = self
                .base_mut()
                .try_get_node_as::<OptionButton>("KhungChinh/VBox/HangCauHinh/Khuong_Option")
        {
            opt.select(idx);
        }
    }

    #[func]
    pub fn get_khuong(&self) -> GString {
        if
            let Some(opt) = self
                .base()
                .try_get_node_as::<OptionButton>("KhungChinh/VBox/HangCauHinh/Khuong_Option")
        {
            return match opt.get_selected() {
                1 => GString::from("1"),
                2 => GString::from("2"),
                _ => GString::from("12"),
            };
        }
        self.khuong.clone()
    }

    #[func]
    pub fn set_cho_phep_cho(&mut self, cho: bool) {
        self.cho_phep_cho = cho;
        if
            let Some(mut chk) = self
                .base_mut()
                .try_get_node_as::<CheckButton>("KhungChinh/VBox/HangCauHinh/ChoPhepCho_Check")
        {
            chk.set_pressed(cho);
        }
    }

    #[func]
    pub fn get_cho_phep_cho(&self) -> bool {
        if
            let Some(chk) = self
                .base()
                .try_get_node_as::<CheckButton>("KhungChinh/VBox/HangCauHinh/ChoPhepCho_Check")
        {
            return chk.is_pressed();
        }
        self.cho_phep_cho
    }

    #[func]
    pub fn set_khoang_nhip(&mut self, bat_dau: f32, ket_thuc: f32) {
        self.nhip_bat_dau = bat_dau;
        self.nhip_ket_thuc = ket_thuc;
        if
            let Some(mut s1) = self
                .base_mut()
                .try_get_node_as::<SpinBox>("KhungChinh/VBox/HangCauHinh/NhipBatDau_Spin")
        {
            s1.set_value(bat_dau as f64);
        }
        if
            let Some(mut s2) = self
                .base_mut()
                .try_get_node_as::<SpinBox>("KhungChinh/VBox/HangCauHinh/NhipKetThuc_Spin")
        {
            s2.set_value(ket_thuc as f64);
        }
    }

    #[func]
    pub fn get_phan_thoi_gian(&self) -> GString {
        let bd = self
            .base()
            .try_get_node_as::<SpinBox>("KhungChinh/VBox/HangCauHinh/NhipBatDau_Spin")
            .map(|s| s.get_value() as f32)
            .unwrap_or(self.nhip_bat_dau);
        let kt = self
            .base()
            .try_get_node_as::<SpinBox>("KhungChinh/VBox/HangCauHinh/NhipKetThuc_Spin")
            .map(|s| s.get_value() as f32)
            .unwrap_or(self.nhip_ket_thuc);
        GString::from(format!("{}-{}", bd, kt))
    }

    // ==================== CHẾ ĐỘ BẬT/TẮT THÊM HOẶC SỬA ====================
    #[func]
    pub fn set_che_do_sua(&mut self, bat: bool) {
        self.che_do_sua = bat;
        if !bat {
            self.khoa_dang_sua.clear();
        }
        if
            let Some(mut btn) = self
                .base_mut()
                .try_get_node_as::<Button>("KhungChinh/VBox/HangGhiChu/ThemGhiChu_Btn")
        {
            btn.set_text(if bat { "✎ Lưu sửa nhắc" } else { "➕ Thêm nhắc" });
        }
    }

    #[func]
    pub fn get_che_do_sua(&self) -> bool {
        self.che_do_sua
    }
    // ==================== LẤY THÔNG TIN CƠ BẢN CỦA SHEET GỐC ====================
    #[func]
    pub fn doc_thong_tin_sheet_goc(&mut self, path: GString) -> GString {
        let p_str: String = path.to_string().trim().to_string();
        let mut bpm_goc: f32 = 0.0;
        let mut max_nhip: f32 = 0.0;

        // Thử đọc bằng đường dẫn truyền vào, nếu không được thì thử globalize_path
        let sheet_opt = doc_file_json(&p_str).or_else(|| {
            let global_p = ProjectSettings::singleton().globalize_path(&path).to_string();
            doc_file_json(&global_p)
        });

        let thong_bao: String = if let Some(sheet) = sheet_opt {
            for n in sheet.khuong_1.iter().chain(sheet.khuong_2.iter()) {
                let kt = n.nhip_dich + n.nhip_giu;
                if kt > max_nhip {
                    max_nhip = kt;
                }
            }
            bpm_goc = sheet.bpm;

            format!(
                "🎼 Bài gốc: {} | Tác giả: {} | BPM gốc: {:.0} | Tổng độ dài: {:.2} nhịp",
                sheet.ten_bai_hat,
                sheet.tac_gia,
                sheet.bpm,
                max_nhip
            )
        } else {
            format!("⚠ Không đọc được thông tin Sheet tại: {}", p_str)
        };

        if
            let Some(mut lbl) = self
                .base_mut()
                .try_get_node_as::<Label>("KhungChinh/VBox/ThongTinSheet_Label")
        {
            lbl.set_text(&thong_bao);
        }

        // Tự động gán BPM gốc vào ô SpinBox làm chuẩn
        if bpm_goc > 0.0 {
            self.set_bpm_tuy_chinh(bpm_goc);
        }

        GString::from(thong_bao)
    }

    // ==================== THÊM / SỬA / XÓA GHI CHÚ THEO NHỊP ====================
    fn dinh_dang_khoa_nhip(nhip: f32) -> String {
        if (nhip - nhip.round()).abs() < 0.001 {
            format!("{}", nhip.round() as i32)
        } else {
            format!("{}", (nhip * 100.0).round() / 100.0)
        }
    }

    #[func]
    pub fn them_hoac_sua_ghi_chu(&mut self, nhip: f32, noi_dung: GString) {
        let nd = noi_dung.to_string().trim().to_string();
        if nd.is_empty() {
            return;
        }
        if self.che_do_sua && !self.khoa_dang_sua.is_empty() {
            self.cac_ghi_chu.remove(&self.khoa_dang_sua);
        }
        let key = Self::dinh_dang_khoa_nhip(nhip);
        self.cac_ghi_chu.insert(key, nd);
        self.set_che_do_sua(false);
        self.cap_nhat_ui_ghi_chu();
    }

    #[func]
    pub fn cap_nhat_ui_ghi_chu(&mut self) {
        let mut danh_sach: Vec<(f32, String, String)> = self.cac_ghi_chu
            .iter()
            .map(|(k, v)| (k.parse::<f32>().unwrap_or(0.0), k.clone(), v.clone()))
            .collect();
        danh_sach.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        if
            let Some(mut list) = self
                .base_mut()
                .try_get_node_as::<ItemList>("KhungChinh/VBox/DanhSachGhiChu")
        {
            list.clear();
            for (idx, (_, k, v)) in danh_sach.iter().enumerate() {
                list.add_item(&format!("Nhịp {}  ➔  {}", k, v));
                list.set_item_metadata(idx as i32, &Variant::from(GString::from(k.clone())));
            }
        }
    }

    #[func]
    fn _khi_bam_them_ghi_chu(&mut self) {
        let nhip = self
            .base()
            .try_get_node_as::<SpinBox>("KhungChinh/VBox/HangGhiChu/NhipGhiChu_Spin")
            .map(|s| s.get_value() as f32)
            .unwrap_or(0.0);
        let nd = self
            .base()
            .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangGhiChu/NoiDungGhiChu_Input")
            .map(|l| l.get_text())
            .unwrap_or_default();

        self.them_hoac_sua_ghi_chu(nhip, nd);

        if
            let Some(mut l) = self
                .base_mut()
                .try_get_node_as::<LineEdit>("KhungChinh/VBox/HangGhiChu/NoiDungGhiChu_Input")
        {
            l.set_text("");
        }
    }

    #[func]
    fn _khi_chon_ghi_chu(&mut self, idx: i64) {
        if
            let Some(list) = self
                .base()
                .try_get_node_as::<ItemList>("KhungChinh/VBox/DanhSachGhiChu")
        {
            let key = list.get_item_metadata(idx as i32).to_string();
            if let Some(nd) = self.cac_ghi_chu.get(&key).cloned() {
                self.khoa_dang_sua = key.clone();
                self.set_che_do_sua(true);
                if
                    let Some(mut spin) = self
                        .base_mut()
                        .try_get_node_as::<SpinBox>("KhungChinh/VBox/HangGhiChu/NhipGhiChu_Spin")
                {
                    spin.set_value(key.parse::<f64>().unwrap_or(0.0));
                }
                if
                    let Some(mut input) = self
                        .base_mut()
                        .try_get_node_as::<LineEdit>(
                            "KhungChinh/VBox/HangGhiChu/NoiDungGhiChu_Input"
                        )
                {
                    input.set_text(&nd);
                }
            }
        }
    }

    #[func]
    fn _khi_bam_xoa_ghi_chu(&mut self) {
        let key_opt = self
            .base()
            .try_get_node_as::<ItemList>("KhungChinh/VBox/DanhSachGhiChu")
            .and_then(|mut l| {
                let sel = l.get_selected_items();
                sel.get(0).map(|idx| l.get_item_metadata(idx).to_string())
            });
        if let Some(k) = key_opt {
            self.cac_ghi_chu.remove(&k);
            self.set_che_do_sua(false);
            self.cap_nhat_ui_ghi_chu();
        }
    }

    // ==================== LƯU / MỞ / CHƠI THỬ ĐOẠN LUYỆN TẬP ====================
    #[func]
    pub fn luu_bai_luyen_tap(&mut self, path: GString) -> bool {
        let bpm = self.get_bpm_tuy_chinh();
        let bpm_opt = if bpm > 0.0 { Some(bpm) } else { None };

        let data = LuyenTapJson {
            ten_bai_hoc: self.get_ten_bai_hoc().to_string().trim().to_string(),
            duong_dan_sheet: self.get_duong_dan_sheet().to_string().trim().to_string(),
            khuong: self.get_khuong().to_string(),
            cho_phep_cho: self.get_cho_phep_cho(),
            phan_thoi_gian: self.get_phan_thoi_gian().to_string(),
            bpm_tuy_chinh: bpm_opt,
            cac_ghi_chu: self.cac_ghi_chu.clone(),
        };
        luu_file_luyen_tap(&path.to_string(), &data)
    }

    #[func]
    pub fn mo_bai_luyen_tap(&mut self, path: GString) -> bool {
        if let Some(data) = doc_file_luyen_tap(&path.to_string()) {
            self.set_ten_bai_hoc(GString::from(data.ten_bai_hoc));
            // Hàm này đã tự động đọc sheet gốc và gán BPM gốc vào Bpm_Spin
            self.set_duong_dan_sheet(GString::from(data.duong_dan_sheet));
            self.set_khuong(GString::from(data.khuong));
            self.set_cho_phep_cho(data.cho_phep_cho);

            let parts: Vec<&str> = data.phan_thoi_gian.split('-').collect();
            if parts.len() == 2 {
                let bd = parts[0].parse::<f32>().unwrap_or(0.0);
                let kt = parts[1].parse::<f32>().unwrap_or(10.0);
                self.set_khoang_nhip(bd, kt);
            }

            // Chỉ ghi đè BPM nếu file luyện tập có lưu bpm_tuy_chinh > 0, ngược lại giữ nguyên BPM gốc của sheet
            if let Some(bpm_luu) = data.bpm_tuy_chinh {
                if bpm_luu > 0.0 {
                    self.set_bpm_tuy_chinh(bpm_luu);
                }
            }

            self.cac_ghi_chu = data.cac_ghi_chu;
            self.set_che_do_sua(false);
            self.cap_nhat_ui_ghi_chu();

            // Gọi hàm GDScript và phát signal để cập nhật giao diện
            if self.base().has_method("cap_nhat_thong_tin_giao_dien") {
                self.base_mut().call("cap_nhat_thong_tin_giao_dien", &[]);
            }
            self.base_mut().emit_signal("mo_file", &[]);
            return true;
        }
        false
    }

    #[func]
    fn _khi_chon_file_sheet(&mut self, path: GString) {
        let local: GString = ProjectSettings::singleton().localize_path(&path);
        self.set_duong_dan_sheet(local);

        if self.base().has_method("cap_nhat_thong_tin_giao_dien") {
            self.base_mut().call("cap_nhat_thong_tin_giao_dien", &[]);
        }
        self.base_mut().emit_signal("mo_file", &[]);
    }

    #[func]
    pub fn choi_thu_doan_luyen_tap(&mut self) {
        let temp_path = GString::from("user://temp_tap_luyen_test.json");
        self.luu_bai_luyen_tap(temp_path.clone());

        let scene_mau = load::<PackedScene>("res://scene/phanchoi/ChoiSheet.tscn");
        if let Some(mut node_moi) = scene_mau.instantiate() {
            node_moi.set("duong_dan_bai_hoc", &Variant::from(temp_path));
            node_moi.set("phan_luyen_tap", &Variant::from(true));
            node_moi.set("che_do_choi_thu", &Variant::from(true));

            // Đẩy z_index lên cao để đè lên toàn bộ giao diện Tạo Tập Sheet
            if let Ok(mut ctrl_moi) = node_moi.clone().try_cast::<Control>() {
                ctrl_moi.set_z_index(100);
            }

            // Khi ChoiSheet bị đóng (queue_free), tự động gọi hàm hiện lại giao diện
            let obj = self.to_gd();
            node_moi.connect(
                "tree_exited",
                &Callable::from_object_method(&obj, "_khi_dong_choi_thu"),
            );

            // Thêm ChoiSheet vào cây thư mục gốc (Root) để chạy đè lên trên
            if let Some(tree) = self.base().get_tree() {
                if let Some(mut root) = tree.get_root() {
                    root.add_child(&node_moi);
                }
            }

            // Ẩn màn hình Tạo Tập Sheet đi trong lúc đang chơi thử
            self.base_mut().set_visible(false);
        }
    }

    #[func]
    fn _khi_dong_choi_thu(&mut self) {
        // Hiện lại màn hình Tạo Tập Sheet ngay khi đóng ChoiSheet
        self.base_mut().set_visible(true);
    }

    #[func]
    fn _mo_popup_sheet(&mut self) {
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiChonSheet") {
            d.popup_centered();
        }
    }
    #[func]
    fn _mo_popup_luu(&mut self) {
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiLuuJson") {
            d.popup_centered();
        }
    }
    #[func]
    fn _mo_popup_mo(&mut self) {
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiMoJson") {
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
impl IControl for TaoTapLuyenSheet {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            ten_bai_hoc: GString::new(),
            duong_dan_sheet: GString::from("res://bai_hoc/choi_sheet/test.json"),
            khuong: GString::from("12"),
            cho_phep_cho: true,
            nhip_bat_dau: 1.5,
            nhip_ket_thuc: 10.0,
            che_do_sua: false,
            khoa_dang_sua: String::new(),
            cac_ghi_chu: BTreeMap::new(),
            bpm_tuy_chinh: 0.0,
            tu_choi_thu_quay_ve: false,
        }
    }

    fn ready(&mut self) {
        let obj: Gd<TaoTapLuyenSheet> = self.to_gd();
        if
            let Some(mut b) = self
                .base_mut()
                .try_get_node_as::<Button>("ThanhTren/HBox/ChoiThu_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "choi_thu_doan_luyen_tap"));
        }
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
                .try_get_node_as::<Button>("KhungChinh/VBox/HangSheet/ChonSheet_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "_mo_popup_sheet"));
        }
        if
            let Some(mut b) = self
                .base_mut()
                .try_get_node_as::<Button>("KhungChinh/VBox/HangGhiChu/ThemGhiChu_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "_khi_bam_them_ghi_chu"));
        }
        if
            let Some(mut b) = self
                .base_mut()
                .try_get_node_as::<Button>("KhungChinh/VBox/HangGhiChu/XoaGhiChu_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "_khi_bam_xoa_ghi_chu"));
        }
        if
            let Some(mut list) = self
                .base_mut()
                .try_get_node_as::<ItemList>("KhungChinh/VBox/DanhSachGhiChu")
        {
            list.connect("item_selected", &Callable::from_object_method(&obj, "_khi_chon_ghi_chu"));
        }
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiChonSheet") {
            d.connect("file_selected", &Callable::from_object_method(&obj, "_khi_chon_file_sheet"));
        }
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiLuuJson") {
            d.connect("file_selected", &Callable::from_object_method(&obj, "luu_bai_luyen_tap"));
        }
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiMoJson") {
            d.connect("file_selected", &Callable::from_object_method(&obj, "mo_bai_luyen_tap"));
        }
        // Nếu quay về từ chế độ Chơi thử -> Khôi phục lại toàn bộ dữ liệu đang soạn
        if self.tu_choi_thu_quay_ve {
            self.mo_bai_luyen_tap(GString::from("user://temp_tap_luyen_test.json"));
        } else {
            let path_mac_dinh: GString = self.get_duong_dan_sheet();
            if !path_mac_dinh.is_empty() {
                self.doc_thong_tin_sheet_goc(path_mac_dinh);
            }
        }
    }
}
