use godot::{
    builtin::{Callable, Color, GString, Variant},
    classes::{
        Button, CheckButton, Control, FileDialog, IControl, ItemList, LineEdit, PackedScene,
    },
    obj::{Base, Gd, WithBaseField},
    prelude::{godot_api, GodotClass},
    tools::load,
};

use crate::{
    ban_phim_midi::BanPhimMidi,
    doc_sheet_json::{doc_file_nghe_tim_json, luu_file_nghe_tim_json, NgheTimNotJson},
};

#[derive(GodotClass)]
#[class(base = Control)]
pub struct TaoTimNot {
    #[base]
    base: Base<Control>,

    ten_bai_hoc: GString,
    nguoi_tao: GString,
    che_do_sua: bool,
    vi_tri_dang_sua: i32,

    danh_sach_bai: Vec<String>,
    cac_ghi_chu: Vec<String>,
    phim_dang_giu: Vec<i32>,
}

#[godot_api]
impl TaoTimNot {
    // ==================== GETTERS & SETTERS ====================
    #[func]
    pub fn set_ten_bai_hoc(&mut self, ten: GString) {
        self.ten_bai_hoc = ten.clone();
        if let Some(mut input) = self
            .base_mut()
            .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangThongTin/TenBai_Input")
        {
            input.set_text(&ten);
        }
    }

    #[func]
    pub fn get_ten_bai_hoc(&self) -> GString {
        if let Some(input) = self
            .base()
            .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangThongTin/TenBai_Input")
        {
            return input.get_text();
        }
        self.ten_bai_hoc.clone()
    }

    #[func]
    pub fn set_nguoi_tao(&mut self, nguoi: GString) {
        self.nguoi_tao = nguoi.clone();
        if let Some(mut input) = self
            .base_mut()
            .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangThongTin/NguoiTao_Input")
        {
            input.set_text(&nguoi);
        }
    }

    #[func]
    pub fn get_nguoi_tao(&self) -> GString {
        if let Some(input) = self
            .base()
            .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangThongTin/NguoiTao_Input")
        {
            return input.get_text();
        }
        self.nguoi_tao.clone()
    }

    // ==================== CHẾ ĐỘ BẬT/TẮT THÊM HOẶC SỬA ====================
    #[func]
    pub fn set_che_do_sua(&mut self, bat: bool) {
        self.che_do_sua = bat;
        if !bat {
            self.vi_tri_dang_sua = -1;
        }
        if let Some(mut btn) = self
            .base_mut()
            .try_get_node_as::<Button>("KhungSoan/VBox/HangGhiChu/ThemCau_Btn")
        {
            btn.set_text(if bat { "✎ Lưu sửa bước" } else { "➕ Thêm bước" });
        }
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

    // ==================== LẤY THÔNG TIN CƠ BẢN ====================
    #[func]
    pub fn lay_so_luong_buoc(&self) -> i32 {
        self.danh_sach_bai.len() as i32
    }

    #[func]
    pub fn kiem_tra_hop_le(&self) -> bool {
        !self.get_ten_bai_hoc().to_string().trim().is_empty() && !self.danh_sach_bai.is_empty()
    }

    #[func]
    pub fn lay_thong_tin_co_ban(&self) -> GString {
        GString::from(format!(
            "Bài Tìm Nốt: {} | Tác giả: {} | Số bước: {} | Chế độ: {}",
            self.get_ten_bai_hoc(),
            self.get_nguoi_tao(),
            self.danh_sach_bai.len(),
            if self.che_do_sua { "SỬA" } else { "THÊM" }
        ))
    }

    // ==================== NHẬN PHÍM MIDI & SOẠN BƯỚC TÌM NỐT ====================
    #[func]
    fn _khi_bam_midi(&mut self, so_phim: i32) {
        let gop_hop_am = self
            .base()
            .try_get_node_as::<CheckButton>("KhungSoan/VBox/HangSoanCau/GopHopAm_Check")
            .map(|c| c.is_pressed())
            .unwrap_or(true);
        let dang_giu_phim = !self.phim_dang_giu.is_empty();

        if let Some(mut input) = self
            .base_mut()
            .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangSoanCau/ChuoiNot_Input")
        {
            let mut txt = input.get_text().to_string().trim().to_string();
            if gop_hop_am && dang_giu_phim && !txt.is_empty() {
                txt = format!("{}_{}", txt, so_phim);
            } else if txt.is_empty() {
                txt = so_phim.to_string();
            } else {
                txt = format!("{} {}", txt, so_phim);
            }
            input.set_text(&txt);
        }
        if !self.phim_dang_giu.contains(&so_phim) {
            self.phim_dang_giu.push(so_phim);
        }
    }

    #[func]
    fn _khi_nha_midi(&mut self, so_phim: i32) {
        self.phim_dang_giu.retain(|&x| x != so_phim);
    }

    #[func]
    pub fn xoa_not_cuoi(&mut self) {
        if let Some(mut input) = self
            .base_mut()
            .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangSoanCau/ChuoiNot_Input")
        {
            let txt = input.get_text().to_string();
            let mut parts: Vec<&str> = txt.split_whitespace().collect();
            parts.pop();
            input.set_text(&parts.join(" "));
        }
    }

    #[func]
    pub fn them_hoac_sua_buoc(&mut self, chuoi_not: GString, ghi_chu: GString) {
        let cn = chuoi_not.to_string().trim().to_string();
        if cn.is_empty() {
            return;
        }
        let gc = ghi_chu.to_string().trim().to_string();

        if self.che_do_sua
            && self.vi_tri_dang_sua >= 0
            && (self.vi_tri_dang_sua as usize) < self.danh_sach_bai.len()
        {
            let idx = self.vi_tri_dang_sua as usize;
            self.danh_sach_bai[idx] = cn;
            if idx < self.cac_ghi_chu.len() {
                self.cac_ghi_chu[idx] = gc;
            }
            self.set_che_do_sua(false);
        } else {
            self.danh_sach_bai.push(cn);
            self.cac_ghi_chu.push(gc);
        }
        self.lam_moi_danh_sach_ui();
    }

    #[func]
    fn _khi_bam_them_buoc(&mut self) {
        let cn = self
            .base()
            .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangSoanCau/ChuoiNot_Input")
            .map(|l| l.get_text())
            .unwrap_or_default();
        let gc = self
            .base()
            .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangGhiChu/GhiChu_Input")
            .map(|l| l.get_text())
            .unwrap_or_default();

        self.them_hoac_sua_buoc(cn, gc);

        if let Some(mut l1) = self
            .base_mut()
            .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangSoanCau/ChuoiNot_Input")
        {
            l1.set_text("");
        }
        if let Some(mut l2) = self
            .base_mut()
            .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangGhiChu/GhiChu_Input")
        {
            l2.set_text("");
        }
    }

    #[func]
    fn _khi_chon_buoc(&mut self, idx: i64) {
        let i = idx as usize;
        if i < self.danh_sach_bai.len() {
            let cn = self.danh_sach_bai[i].clone();
            let gc = self.cac_ghi_chu.get(i).cloned().unwrap_or_default();
            self.vi_tri_dang_sua = idx as i32;

            if let Some(mut l1) = self
                .base_mut()
                .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangSoanCau/ChuoiNot_Input")
            {
                l1.set_text(&cn);
            }
            if let Some(mut l2) = self
                .base_mut()
                .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangGhiChu/GhiChu_Input")
            {
                l2.set_text(&gc);
            }
        }
    }

    #[func]
    pub fn cap_nhat_buoc_dang_chon(&mut self) {
        self.set_che_do_sua(true);
        self._khi_bam_them_buoc();
    }

    #[func]
    pub fn xoa_buoc_dang_chon(&mut self) {
        let chon = self
            .base()
            .try_get_node_as::<ItemList>("KhungSoan/VBox/DanhSachCau")
            .map(|mut l| l.get_selected_items())
            .unwrap_or_default();
        if let Some(idx) = chon.get(0) {
            let i = idx as usize;
            if i < self.danh_sach_bai.len() {
                self.danh_sach_bai.remove(i);
                if i < self.cac_ghi_chu.len() {
                    self.cac_ghi_chu.remove(i);
                }
                self.set_che_do_sua(false);
                self.lam_moi_danh_sach_ui();
            }
        }
    }

    #[func]
    pub fn lam_moi_danh_sach_ui(&mut self) {
        let danh_sach_bai = self.danh_sach_bai.clone();
        let cac_ghi_chu = self.cac_ghi_chu.clone();
        if let Some(mut list) = self
            .base_mut()
            .try_get_node_as::<ItemList>("KhungSoan/VBox/DanhSachCau")
        {
            list.clear();
            for (i, cn) in danh_sach_bai.iter().enumerate() {
                let gc = cac_ghi_chu.get(i).map(|s| s.as_str()).unwrap_or("");
                list.add_item(&format!("Bước {}:  [{}]   —   Gợi ý: {}", i + 1, cn, gc));
            }
        }
    }

    // ==================== NGHE THỬ & CHƠI THỬ ====================
    fn phat_chuoi_not(mut ban_phim: Gd<BanPhimMidi>, chuoi: &str) {
        let mau_to = Color::from_rgba(0.2, 0.95, 0.45, 0.85);
        for buoc in chuoi.split_whitespace() {
            for n_str in buoc.split('_') {
                if let Ok(so_phim) = n_str.trim().parse::<i32>() {
                    let mut bp = ban_phim.bind_mut();
                    bp.phat_am_thanh_not(so_phim, 100, 480);
                    bp.to_phim(so_phim, mau_to);
                }
            }
        }
    }

    #[func]
    pub fn nghe_thu_buoc_hien_tai(&mut self) {
        let mut chuoi = self
            .base()
            .try_get_node_as::<LineEdit>("KhungSoan/VBox/HangSoanCau/ChuoiNot_Input")
            .map(|l| l.get_text().to_string())
            .unwrap_or_default();

        if chuoi.trim().is_empty() && self.vi_tri_dang_sua >= 0 {
            if let Some(c) = self.danh_sach_bai.get(self.vi_tri_dang_sua as usize) {
                chuoi = c.clone();
            }
        }
        if let Some(bp) = self.base().try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano") {
            Self::phat_chuoi_not(bp, &chuoi);
        }
    }

    #[func]
    pub fn luu_bai_hoc_json(&mut self, path: GString) -> bool {
        let data = NgheTimNotJson {
            ten_bai_hoc: self.get_ten_bai_hoc().to_string().trim().to_string(),
            nguoi_tao: self.get_nguoi_tao().to_string().trim().to_string(),
            danh_sach_bai: self.danh_sach_bai.clone(),
            cac_ghi_chu: self.cac_ghi_chu.clone(),
        };
        luu_file_nghe_tim_json(&path.to_string(), &data)
    }

    #[func]
    pub fn mo_bai_hoc_json(&mut self, path: GString) -> bool {
        if let Some(data) = doc_file_nghe_tim_json(&path.to_string()) {
            self.set_ten_bai_hoc(GString::from(data.ten_bai_hoc));
            self.set_nguoi_tao(GString::from(data.nguoi_tao));
            self.danh_sach_bai = data.danh_sach_bai;
            self.cac_ghi_chu = data.cac_ghi_chu;
            self.set_che_do_sua(false);
            self.lam_moi_danh_sach_ui();
            return true;
        }
        false
    }

    #[func]
    pub fn choi_thu_bai_hoc(&mut self) {
        if self.danh_sach_bai.is_empty() {
            self._khi_bam_them_buoc();
        }
        if self.danh_sach_bai.is_empty() {
            return;
        }
        let temp_path = GString::from("user://temp_tim_not_test.json");
        self.luu_bai_hoc_json(temp_path.clone());

        let scene_mau = load::<PackedScene>("res://scene/phanchoi/HocTimNot.tscn");
        {
            if let Some(mut node_moi) = scene_mau.instantiate() {
                node_moi.set("duong_dan_bai_hoc", &Variant::from(temp_path));
                if let Some(mut tree) = self.base().get_tree() {
                    if let Some(mut root) = tree.get_root() {
                        let cur = tree.get_current_scene();
                        root.add_child(&node_moi);
                        tree.set_current_scene(&node_moi);
                        if let Some(mut c) = cur {
                            c.queue_free();
                        }
                    }
                }
            }
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
impl IControl for TaoTimNot {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            ten_bai_hoc: GString::new(),
            nguoi_tao: GString::from("timekp"),
            che_do_sua: false,
            vi_tri_dang_sua: -1,
            danh_sach_bai: Vec::new(),
            cac_ghi_chu: Vec::new(),
            phim_dang_giu: Vec::new(),
        }
    }

    fn ready(&mut self) {
        let obj = self.to_gd();
        if let Some(mut bp) = self.base_mut().try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano") {
            bp.connect("phim_vua_duoc_bam", &Callable::from_object_method(&obj, "_khi_bam_midi"));
            bp.connect("phim_vua_duoc_nha", &Callable::from_object_method(&obj, "_khi_nha_midi"));
        }
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/NgheThu_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "nghe_thu_buoc_hien_tai"));
        }
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/ChoiThuBai_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "choi_thu_bai_hoc"));
        }
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/MoJson_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "_mo_popup_mo"));
        }
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/LuuJson_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "_mo_popup_luu"));
        }
        if let Some(mut b) = self.base_mut().try_get_node_as::<Button>("ThanhTren/HBox/VeMenu_Btn") {
            b.connect("pressed", &Callable::from_object_method(&obj, "_ve_menu_chinh"));
        }
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("KhungSoan/VBox/HangSoanCau/XoaCuoi_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "xoa_not_cuoi"));
        }
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("KhungSoan/VBox/HangGhiChu/ThemCau_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "_khi_bam_them_buoc"));
        }
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("KhungSoan/VBox/HangGhiChu/CapNhatCau_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "cap_nhat_buoc_dang_chon"));
        }
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("KhungSoan/VBox/HangGhiChu/XoaCau_Btn")
        {
            b.connect("pressed", &Callable::from_object_method(&obj, "xoa_buoc_dang_chon"));
        }
        if let Some(mut list) = self
            .base_mut()
            .try_get_node_as::<ItemList>("KhungSoan/VBox/DanhSachCau")
        {
            list.connect("item_selected", &Callable::from_object_method(&obj, "_khi_chon_buoc"));
        }
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiLuuJson") {
            d.connect("file_selected", &Callable::from_object_method(&obj, "luu_bai_hoc_json"));
        }
        if let Some(mut d) = self.base_mut().try_get_node_as::<FileDialog>("HopThoaiMoJson") {
            d.connect("file_selected", &Callable::from_object_method(&obj, "mo_bai_hoc_json"));
        }
    }
}