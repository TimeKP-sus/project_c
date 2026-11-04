use std::time::SystemTime;

use godot::{
    builtin::{Callable, GString, Variant},
    classes::{
        Button, Control, DirAccess, FileAccess, FileDialog, IControl, ItemList, LineEdit,
        OptionButton, ProjectSettings,
    },
    obj::{Base, WithBaseField},
    prelude::{godot_api, GodotClass},
};
use serde_json::Value;

use crate::doc_sheet_json::{
    doc_file_chuong_json, luu_file_chuong_json, ChuongJson, MucBaiHocChuongJson,
};

const CAC_LOAI_BAI: [&str; 5] = [
    "choi_sheet",
    "tap_luyen_sheet",
    "nghe_va_doan_not",
    "tim_not",
    "xem_video",
];

#[derive(GodotClass)]
#[class(base = Control)]
pub struct TaoDanhSach {
    #[base]
    base: Base<Control>,

    ten_chuong: GString,
    che_do_sua: bool,
    vi_tri_dang_sua: i32,

    kho_bai_hoc: Vec<MucBaiHocChuongJson>,
    bai_trong_chuong: Vec<MucBaiHocChuongJson>,
}

#[godot_api]
impl TaoDanhSach {
    fn hien_thong_bao(&self, noi_dung: &str, loai: &str) {
        if let Some(mut tb) = self.base().try_get_node_as::<crate::thong_bao::ThongBao>("ThongBao") {
            let noi_dung: GString = GString::from(noi_dung);
            match loai {
                "loi" => tb.bind_mut().thong_bao_loi(noi_dung),
                "canh_bao" => tb.bind_mut().thong_bao_canh_bao(noi_dung),
                _ => tb.bind_mut().thong_bao_thanh_cong(noi_dung),
            }
        }
    }

    // ==================== GETTERS & SETTERS ====================
    #[func]
    pub fn set_ten_chuong(&mut self, ten: GString) {
        self.ten_chuong = ten.clone();
        if let Some(mut input) = self
            .base_mut()
            .try_get_node_as::<LineEdit>("CotPhai/VBox/HangTenChuong/TenChuong_Input")
        {
            input.set_text(&ten);
        }
    }

    #[func]
    pub fn get_ten_chuong(&self) -> GString {
        if let Some(input) = self
            .base()
            .try_get_node_as::<LineEdit>("CotPhai/VBox/HangTenChuong/TenChuong_Input")
        {
            return input.get_text();
        }
        self.ten_chuong.clone()
    }

    #[func]
    pub fn set_che_do_sua(&mut self, bat: bool) {
        self.che_do_sua = bat;
        if !bat {
            self.vi_tri_dang_sua = -1;
        }
    }

    #[func]
    pub fn get_che_do_sua(&self) -> bool {
        self.che_do_sua
    }

    // ==================== LẤY THÔNG TIN CƠ BẢN ====================
    #[func]
    pub fn lay_so_bai_trong_chuong(&self) -> i32 {
        self.bai_trong_chuong.len() as i32
    }

    #[func]
    pub fn lay_thong_tin_chuong(&self) -> GString {
        GString::from(format!(
            "Chương: {} | Tổng số bài: {} | Kho bài hiện có: {}",
            self.get_ten_chuong(),
            self.bai_trong_chuong.len(),
            self.kho_bai_hoc.len()
        ))
    }

    // ==================== QUÉT & NHẬN DIỆN KHO BÀI HỌC ====================
    fn nhan_dien_file_bai_hoc(path: &str) -> Option<MucBaiHocChuongJson> {
        let txt = FileAccess::get_file_as_string(path).to_string();
        let val: Value = serde_json::from_str(&txt).ok()?;
        let obj = val.as_object()?;

        // Bỏ qua nếu đây là file danh sách chương
        if obj.contains_key("ten_chuong") && obj.contains_key("bai_hoc") {
            return None;
        }

        let ten = obj
            .get("ten_bai_hoc")
            .or_else(|| obj.get("ten_bai_hat"))
            .and_then(|v| v.as_str())
            .unwrap_or("Bài học")
            .to_string();

        let loai = if obj.contains_key("duong_dan_video") || path.contains("video") {
            "xem_video"
        } else if obj.contains_key("duong_dan_sheet")
            || path.contains("luyen_tap")
            || path.contains("tap_luyen")
        {
            "tap_luyen_sheet"
        } else if obj.contains_key("danh_sach_bai") {
            if path.contains("tim_not") {
                "tim_not"
            } else {
                "nghe_va_doan_not"
            }
        } else {
            "choi_sheet"
        };

        Some(MucBaiHocChuongJson {
            ten_bai_hoc: ten,
            loai_bai_hoc: loai.to_string(),
            duong_dan: path.to_string(),
        })
    }

    fn quet_de_quy(&mut self, dir_path: &str) {
        let Some(mut dir) = DirAccess::open(dir_path) else {
            return;
        };
        dir.list_dir_begin();
        loop {
            let fname = dir.get_next().to_string();
            if fname.is_empty() {
                break;
            }
            if fname == "." || fname == ".." {
                continue;
            }
            let full_path = format!("{}/{}", dir_path.trim_end_matches('/'), fname);
            if dir.current_is_dir() {
                if fname != "danh_sach" {
                    self.quet_de_quy(&full_path);
                }
            } else if fname.ends_with(".json") {
                if let Some(info) = Self::nhan_dien_file_bai_hoc(&full_path) {
                    self.kho_bai_hoc.push(info);
                }
            }
        }
    }

    #[func]
    pub fn quet_kho_bai_hoc(&mut self) {
        self.kho_bai_hoc.clear();
        self.quet_de_quy("res://bai_hoc");
        self.hien_thi_kho_bai_hoc();
    }
    #[func]
    pub fn hien_thi_kho_bai_hoc(&mut self) {
        let chon_loc = self
            .base()
            .try_get_node_as::<OptionButton>("CotTrai/VBox/HangLoc/LocLoai_Option")
            .map(|o| o.get_selected())
            .unwrap_or(0);
            
        // LẤY TỪ KHÓA TÌM KIẾM (Chuyển về chữ thường để so sánh không phân biệt hoa thường)
        let tu_khoa = self
            .base()
            .try_get_node_as::<LineEdit>("CotTrai/VBox/HangLoc/TimKiem_Input")
            .map(|l| l.get_text().to_string().to_lowercase())
            .unwrap_or_default();

        let kho_bai_hoc = self.kho_bai_hoc.clone();

        if let Some(mut list) = self
            .base_mut()
            .try_get_node_as::<ItemList>("CotTrai/VBox/KhoBaiHoc_List")
        {
            list.clear();
            let mut item_idx = 0;
            for (real_idx, b) in kho_bai_hoc.iter().enumerate() {
                // 1. Lọc theo Option (Loại bài học)
                if chon_loc > 0 {
                    let loai_loc = CAC_LOAI_BAI.get((chon_loc - 1) as usize).copied().unwrap_or("");
                    if b.loai_bai_hoc != loai_loc {
                        continue;
                    }
                }
                
                // 2. Lọc theo từ khóa (Tên bài học)
                if !tu_khoa.is_empty() && !b.ten_bai_hoc.to_lowercase().contains(&tu_khoa) {
                    continue;
                }

                let ten_file = b.duong_dan.rsplit('/').next().unwrap_or(&b.duong_dan);
                list.add_item(&format!(
                    "[{}]  {}  ({})",
                    b.loai_bai_hoc, b.ten_bai_hoc, ten_file
                ));
                list.set_item_metadata(item_idx, &Variant::from(real_idx as i32));
                item_idx += 1;
            }
        }
    }

    #[func]
    fn _khi_doi_bo_loc(&mut self, _idx: i64) {
        self.hien_thi_kho_bai_hoc();
    }
    #[func]
    fn _khi_go_tim_kiem(&mut self, _text: GString) {
        self.hien_thi_kho_bai_hoc();
    }

    // ==================== QUẢN LÝ BÀI HỌC TRONG CHƯƠNG ====================
    #[func]
    pub fn them_bai_tu_kho_vao_chuong(&mut self) {
        let mut cac_index_kho: Vec<usize> = Vec::new();
        if let Some(mut list) = self
            .base()
            .try_get_node_as::<ItemList>("CotTrai/VBox/KhoBaiHoc_List")
        {
            let sel = list.get_selected_items();
            for i in 0..sel.len() {
                if let Some(ui_idx) = sel.get(i) {
                    let real_idx = list.get_item_metadata(ui_idx).to::<i32>() as usize;
                    cac_index_kho.push(real_idx);
                }
            }
        }
        let so_bai_them = cac_index_kho.len();
        for idx in cac_index_kho {
            if let Some(b) = self.kho_bai_hoc.get(idx).cloned() {
                self.bai_trong_chuong.push(b);
            }
        }
        self.cap_nhat_ui_chuong();
        if so_bai_them > 0 {
            self.hien_thong_bao("Đã thêm bài học vào chương.", "thanh_cong");
        }
    }

    #[func]
    pub fn them_bai_hoc_thu_cong(
        &mut self,
        ten_bai: GString,
        loai_bai: GString,
        duong_dan: GString,
    ) {
        self.bai_trong_chuong.push(MucBaiHocChuongJson {
            ten_bai_hoc: ten_bai.to_string(),
            loai_bai_hoc: loai_bai.to_string(),
            duong_dan: duong_dan.to_string(),
        });
        self.cap_nhat_ui_chuong();
    }

    #[func]
    fn _khi_chon_file_bai_le(&mut self, path: GString) {
        let local = ProjectSettings::singleton().localize_path(&path).to_string();
        if let Some(info) = Self::nhan_dien_file_bai_hoc(&local) {
            self.bai_trong_chuong.push(info);
            self.cap_nhat_ui_chuong();
        }
    }

    #[func]
    pub fn cap_nhat_ui_chuong(&mut self) {
        let bai_trong_chuong = self.bai_trong_chuong.clone();
        if let Some(mut list) = self
            .base_mut()
            .try_get_node_as::<ItemList>("CotPhai/VBox/BaiTrongChuong_List")
        {
            list.clear();
            for (i, b) in bai_trong_chuong.iter().enumerate() {
                list.add_item(&format!(
                    "{}. [{}]  {}  ➔  {}",
                    i + 1,
                    b.loai_bai_hoc,
                    b.ten_bai_hoc,
                    b.duong_dan
                ));
            }
        }
    }

    #[func]
    fn _khi_chon_bai_trong_chuong(&mut self, idx: i64) {
        let i = idx as usize;
        if let Some(b) = self.bai_trong_chuong.get(i).cloned() {
            self.vi_tri_dang_sua = idx as i32;
            self.che_do_sua = true;

            if let Some(mut input) = self
                .base_mut()
                .try_get_node_as::<LineEdit>("CotPhai/VBox/HangSuaNhanh/TenBaiTuyChinh_Input")
            {
                input.set_text(&b.ten_bai_hoc);
            }
            if let Some(mut opt) = self
                .base_mut()
                .try_get_node_as::<OptionButton>("CotPhai/VBox/HangSuaNhanh/LoaiBai_Option")
            {
                let pos = CAC_LOAI_BAI
                    .iter()
                    .position(|&x| x == b.loai_bai_hoc)
                    .unwrap_or(0);
                opt.select(pos as i32);
            }
        }
    }

    #[func]
    pub fn cap_nhat_bai_dang_chon(&mut self) {
        if self.vi_tri_dang_sua < 0
            || (self.vi_tri_dang_sua as usize) >= self.bai_trong_chuong.len()
        {
            return;
        }
        let ten_moi = self
            .base()
            .try_get_node_as::<LineEdit>("CotPhai/VBox/HangSuaNhanh/TenBaiTuyChinh_Input")
            .map(|l| l.get_text().to_string().trim().to_string())
            .unwrap_or_default();
        let loai_idx = self
            .base()
            .try_get_node_as::<OptionButton>("CotPhai/VBox/HangSuaNhanh/LoaiBai_Option")
            .map(|o| o.get_selected() as usize)
            .unwrap_or(0);

        let i = self.vi_tri_dang_sua as usize;
        if !ten_moi.is_empty() {
            self.bai_trong_chuong[i].ten_bai_hoc = ten_moi;
        }
        self.bai_trong_chuong[i].loai_bai_hoc =
            CAC_LOAI_BAI.get(loai_idx).copied().unwrap_or("choi_sheet").to_string();

        self.cap_nhat_ui_chuong();
        let vi_tri_dang_sua = self.vi_tri_dang_sua;
        if let Some(mut list) = self
            .base_mut()
            .try_get_node_as::<ItemList>("CotPhai/VBox/BaiTrongChuong_List")
        {
            list.select(vi_tri_dang_sua);
        }
    }

    #[func]
    pub fn day_bai_len(&mut self) {
        self.di_chuyen_bai(-1);
    }

    #[func]
    pub fn day_bai_xuong(&mut self) {
        self.di_chuyen_bai(1);
    }

    fn di_chuyen_bai(&mut self, huong: i32) {
        let chon = self
            .base()
            .try_get_node_as::<ItemList>("CotPhai/VBox/BaiTrongChuong_List")
            .map(|mut l| l.get_selected_items())
            .unwrap_or_default();
        if let Some(idx) = chon.get(0) {
            let moi = idx + huong;
            if moi >= 0 && (moi as usize) < self.bai_trong_chuong.len() {
                self.bai_trong_chuong.swap(idx as usize, moi as usize);
                self.vi_tri_dang_sua = moi;
                self.cap_nhat_ui_chuong();
                if let Some(mut list) = self
                    .base_mut()
                    .try_get_node_as::<ItemList>("CotPhai/VBox/BaiTrongChuong_List")
                {
                    list.select(moi);
                }
            }
        }
    }

    #[func]
    pub fn xoa_bai_khoi_chuong(&mut self) {
        let chon = self
            .base()
            .try_get_node_as::<ItemList>("CotPhai/VBox/BaiTrongChuong_List")
            .map(|mut l| l.get_selected_items())
            .unwrap_or_default();
        if let Some(idx) = chon.get(0) {
            let i = idx as usize;
            if i < self.bai_trong_chuong.len() {
                self.bai_trong_chuong.remove(i);
                self.set_che_do_sua(false);
                self.cap_nhat_ui_chuong();
            }
        }
    }

    // ==================== LƯU & MỞ FILE CHƯƠNG JSON ====================
    #[func]
    pub fn luu_chuong_json(&mut self, path: GString) -> bool {
        let data = ChuongJson {
            ten_chuong: self.get_ten_chuong().to_string().trim().to_string(),
            bai_hoc: self.bai_trong_chuong.clone(),
        };
        luu_file_chuong_json(&path.to_string(), &data)
    }

    #[func]
    pub fn mo_chuong_json(&mut self, path: GString) -> bool {
        if let Some(data) = doc_file_chuong_json(&path.to_string()) {
            self.set_ten_chuong(GString::from(data.ten_chuong));
            self.bai_trong_chuong = data.bai_hoc;
            self.set_che_do_sua(false);
            self.cap_nhat_ui_chuong();
            return true;
        }
        false
    }

    #[func]
    fn _mo_popup_chon_le(&mut self) {
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiChonBaiLe") {
            d.popup_centered();
        }
    }
    #[func]
    fn _mo_popup_luu(&mut self) {
        // Lấy tên trước khi mượn mutable self qua base_mut().
        let ten_chuong: String = self.get_ten_chuong().to_string().trim().replace(" ", "_").to_lowercase();

        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiLuuChuong") {
            d.set_current_dir("res://bai_hoc");
            
            let id_ngau_nhien: u64 = SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
            
            let ten_file_moi = if ten_chuong.is_empty() {
                format!("chuong_{}.json", id_ngau_nhien)
            } else {
                format!("{}_{}.json", ten_chuong, id_ngau_nhien)
            };

            d.set_current_file(&GString::from(&ten_file_moi));
            d.popup_centered();
        }
    }
    #[func]
    fn _mo_popup_mo(&mut self) {
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiMoChuong") {
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
impl IControl for TaoDanhSach {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            ten_chuong: GString::from("Chương 1"),
            che_do_sua: false,
            vi_tri_dang_sua: -1,
            kho_bai_hoc: Vec::new(),
            bai_trong_chuong: Vec::new(),
        }
    }

    fn ready(&mut self) {
        let obj = self.to_gd();
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/QuetLai_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "quet_kho_bai_hoc"));
        }
        if let Some(mut input) = self
            .base_mut()
            .try_get_node_as::<LineEdit>("CotTrai/VBox/HangLoc/TimKiem_Input")
        {
            input.connect("text_changed", &Callable::from_object_method(&obj, "_khi_go_tim_kiem"));
        }
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/ChonFileLe_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "_mo_popup_chon_le"));
        }
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/MoChuong_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "_mo_popup_mo"));
        }
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/LuuChuong_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "_mo_popup_luu"));
        }
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/VeMenu_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "_ve_menu_chinh"));
        }
        if let Some(mut opt) = self
            .base_mut()
            .try_get_node_as::<OptionButton>("CotTrai/VBox/HangLoc/LocLoai_Option")
        {
            opt.connect("item_selected", &Callable::from_object_method(&obj, "_khi_doi_bo_loc"));
        }
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("CotTrai/VBox/ThemVaoChuong_Btn")
        {
            b.connect(
                "pressed",
                &Callable::from_object_method(&obj, "them_bai_tu_kho_vao_chuong"),
            );
        }
        if let Some(mut list) = self
            .base_mut()
            .try_get_node_as::<ItemList>("CotPhai/VBox/BaiTrongChuong_List")
        {
            list.connect(
                "item_selected",
                &Callable::from_object_method(&obj, "_khi_chon_bai_trong_chuong"),
            );
        }
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("CotPhai/VBox/HangSuaNhanh/CapNhatBai_Btn")
        {
            b.connect(
                "pressed",
                &Callable::from_object_method(&obj, "cap_nhat_bai_dang_chon"),
            );
        }
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("CotPhai/VBox/HangDieuKhien/Len_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "day_bai_len"));
        }
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("CotPhai/VBox/HangDieuKhien/Xuong_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "day_bai_xuong"));
        }
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("CotPhai/VBox/HangDieuKhien/XoaBai_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "xoa_bai_khoi_chuong"));
        }
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiChonBaiLe") {
            d.connect("file_selected", &Callable::from_object_method(&obj, "_khi_chon_file_bai_le"));
        }
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiLuuChuong") {
            d.connect("file_selected", &Callable::from_object_method(&obj, "luu_chuong_json"));
        }
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiMoChuong") {
            d.connect("file_selected", &Callable::from_object_method(&obj, "mo_chuong_json"));
        }

        self.quet_kho_bai_hoc();
    }
}