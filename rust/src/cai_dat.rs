use godot::{
    builtin::{Callable, GString, Variant, Vector2i},
    classes::{
        AudioServer, Button, CheckButton, Control, DisplayServer, Engine, HSlider, IControl,
        Label, OptionButton,
    },
    obj::{Base, WithBaseField},
    prelude::{godot_api, GodotClass},
};

use crate::du_lieu_nguoi_dung::{CaiDatGameJson, duong_dan_cai_dat};

const DANH_SACH_FPS: [i32; 6] = [0, 60, 90,120, 144, 240];

#[derive(GodotClass)]
#[class(base = Control)]
pub struct ManHinhCaiDat {
    #[base]
    base: Base<Control>,
    cai_dat: CaiDatGameJson,
}

#[godot_api]
impl ManHinhCaiDat {
    // ==================== GETTERS & SETTERS ====================
    #[func]
    pub fn set_toc_do_choi(&mut self, toc_do: f32) {
        self.cai_dat.toc_do_choi = toc_do.clamp(0.25, 2.5);
        let toc_do_choi = self.cai_dat.toc_do_choi;
        if let Some(mut s) = self
            .base_mut()
            .try_get_node_as::<HSlider>("KhungChinh/HBox/CotChoiNhac/VBox/HangTocDo/TocDo_Slider")
        {
            s.set_value(toc_do_choi as f64);
        }
        if let Some(mut lbl) = self
            .base_mut()
            .try_get_node_as::<Label>("KhungChinh/HBox/CotChoiNhac/VBox/HangTocDo/TocDo_Val")
        {
            lbl.set_text(&format!("{:.2}x", toc_do_choi));
        }
    }

    #[func]
    pub fn get_toc_do_choi(&self) -> f32 {
        self.cai_dat.toc_do_choi
    }

    #[func]
    pub fn set_do_tre_phim_ms(&mut self, do_tre_ms: f32) {
        self.cai_dat.do_tre_phim_ms = do_tre_ms.clamp(-200.0, 200.0);
        let do_tre_phim_ms = self.cai_dat.do_tre_phim_ms;
        if let Some(mut s) = self
            .base_mut()
            .try_get_node_as::<HSlider>("KhungChinh/HBox/CotChoiNhac/VBox/HangDoTre/DoTre_Slider")
        {
            s.set_value(do_tre_phim_ms as f64);
        }
        if let Some(mut lbl) = self
            .base_mut()
            .try_get_node_as::<Label>("KhungChinh/HBox/CotChoiNhac/VBox/HangDoTre/DoTre_Val")
        {
            let dau = if do_tre_phim_ms > 0.0 { "+" } else { "" };
            lbl.set_text(&format!("{}{:.0} ms", dau, do_tre_phim_ms));
        }
    }

    #[func]
    pub fn get_do_tre_phim_ms(&self) -> f32 {
        self.cai_dat.do_tre_phim_ms
    }

    #[func]
    pub fn set_am_luong_chung(&mut self, am_luong: f32) {
        self.cai_dat.am_luong_chung = am_luong.clamp(0.0, 100.0);
        let am_luong_chung = self.cai_dat.am_luong_chung;
        if let Some(mut s) = self.base_mut().try_get_node_as::<HSlider>(
            "KhungChinh/HBox/CotChoiNhac/VBox/HangAmLuong/AmLuong_Slider",
        ) {
            s.set_value(am_luong_chung as f64);
        }
        if let Some(mut lbl) = self
            .base_mut()
            .try_get_node_as::<Label>("KhungChinh/HBox/CotChoiNhac/VBox/HangAmLuong/AmLuong_Val")
        {
            lbl.set_text(&format!("{:.0}%", am_luong_chung));
        }
    }

    #[func]
    pub fn get_am_luong_chung(&self) -> f32 {
        self.cai_dat.am_luong_chung
    }

    #[func]
    pub fn set_toan_man_hinh(&mut self, bat: bool) {
        self.cai_dat.toan_man_hinh = bat;
        if let Some(mut chk) = self
            .base_mut()
            .try_get_node_as::<CheckButton>("KhungChinh/HBox/CotDoHoa/VBox/ToanManHinh_Check")
        {
            chk.set_pressed(bat);
        }
    }

    #[func]
    pub fn get_toan_man_hinh(&self) -> bool {
        self.cai_dat.toan_man_hinh
    }

    #[func]
    pub fn set_bat_vsync(&mut self, bat: bool) {
        self.cai_dat.bat_vsync = bat;
        if let Some(mut chk) = self
            .base_mut()
            .try_get_node_as::<CheckButton>("KhungChinh/HBox/CotDoHoa/VBox/VSync_Check")
        {
            chk.set_pressed(bat);
        }
    }

    #[func]
    pub fn get_bat_vsync(&self) -> bool {
        self.cai_dat.bat_vsync
    }

    #[func]
    pub fn set_gioi_han_fps(&mut self, fps: i32) {
        self.cai_dat.gioi_han_fps = fps;
        let idx = DANH_SACH_FPS.iter().position(|&x| x == fps).unwrap_or(1) as i32;
        if let Some(mut opt) = self.base_mut().try_get_node_as::<OptionButton>(
            "KhungChinh/HBox/CotDoHoa/VBox/HangFps/GioiHanFps_Option",
        ) {
            opt.select(idx);
        }
    }

    #[func]
    pub fn get_gioi_han_fps(&self) -> i32 {
        self.cai_dat.gioi_han_fps
    }

    #[func]
    pub fn set_do_phan_giai_idx(&mut self, idx: i32) {
        self.cai_dat.do_phan_giai_idx = idx.clamp(0, 3);
        let do_phan_giai_idx = self.cai_dat.do_phan_giai_idx;
        if let Some(mut opt) = self.base_mut().try_get_node_as::<OptionButton>(
            "KhungChinh/HBox/CotDoHoa/VBox/HangPhanGiai/DoPhanGiai_Option",
        ) {
            opt.select(do_phan_giai_idx);
        }
    }

    #[func]
    pub fn get_do_phan_giai_idx(&self) -> i32 {
        self.cai_dat.do_phan_giai_idx
    }

    #[func]
    pub fn set_hien_thi_fps(&mut self, bat: bool) {
        self.cai_dat.hien_thi_fps = bat;
        if let Some(mut chk) = self
            .base_mut()
            .try_get_node_as::<CheckButton>("KhungChinh/HBox/CotDoHoa/VBox/HienThiFps_Check")
        {
            chk.set_pressed(bat);
        }
    }

    #[func]
    pub fn get_hien_thi_fps(&self) -> bool {
        self.cai_dat.hien_thi_fps
    }

    // ==================== ÁP DỤNG CÀI ĐẶT VÀO ENGINE ====================
    #[func]
    pub fn ap_dung_cai_dat_he_thong(&mut self) {
        // 1. Áp dụng giới hạn FPS
        Engine::singleton().set_max_fps(self.cai_dat.gioi_han_fps);

        // 2. Áp dụng Âm lượng tổng (Bus 0 - Master)
        let mut audio = AudioServer::singleton();
        if self.cai_dat.am_luong_chung <= 0.5 {
            audio.set_bus_mute(0, true);
        } else {
            audio.set_bus_mute(0, false);
            let db = (self.cai_dat.am_luong_chung - 80.0) * 0.4;
            audio.set_bus_volume_db(0, db);
        }

        // 3. Áp dụng VSync (0 = Disabled, 1 = Enabled)
        let vsync_val = if self.cai_dat.bat_vsync { 1 } else { 0 };
        DisplayServer::singleton().call("window_set_vsync_mode", &[Variant::from(vsync_val)]);

        // 4. Áp dụng Toàn màn hình (3 = Fullscreen, 0 = Windowed) & Độ phân giải
        if let Some(tree) = self.base().get_tree() {
            if let Some(mut root_win) = tree.get_root() {
                let mode_val = if self.cai_dat.toan_man_hinh { 3 } else { 0 };
                root_win.set("mode", &Variant::from(mode_val));

                if !self.cai_dat.toan_man_hinh {
                    let (w, h) = self.cai_dat.lay_kich_thuoc_cua_so();
                    root_win.set("size", &Variant::from(Vector2i::new(w, h)));
                }
            }
        }
    }

    #[func]
    pub fn dong_bo_len_ui(&mut self) {
        let cfg = self.cai_dat.clone();
        self.set_toc_do_choi(cfg.toc_do_choi);
        self.set_do_tre_phim_ms(cfg.do_tre_phim_ms);
        self.set_am_luong_chung(cfg.am_luong_chung);
        self.set_toan_man_hinh(cfg.toan_man_hinh);
        self.set_bat_vsync(cfg.bat_vsync);
        self.set_gioi_han_fps(cfg.gioi_han_fps);
        self.set_do_phan_giai_idx(cfg.do_phan_giai_idx);
        self.set_hien_thi_fps(cfg.hien_thi_fps);
    }

    #[func]
    pub fn luu_va_ap_dung(&mut self) -> bool {
        self.ap_dung_cai_dat_he_thong();

        let ok = self.cai_dat.luu_cai_dat();
        let toc_do_choi = self.cai_dat.toc_do_choi;
        let do_tre_phim_ms = self.cai_dat.do_tre_phim_ms;
        if let Some(mut lbl) = self
            .base_mut()
            .try_get_node_as::<Label>("KhungChinh/TrangThai_Label")
        {
            if ok {
                lbl.set_text(&format!(
                    "✔ Đã lưu & áp dụng thành công vào: {} (Tốc độ: {:.2}x | Độ trễ: {:.0}ms)",
                    duong_dan_cai_dat(), toc_do_choi, do_tre_phim_ms
                ));
            } else {
                lbl.set_text("❌ Lỗi: Không thể lưu file cài đặt!");
            }
        }
        ok
    }

    #[func]
    pub fn dat_lai_mac_dinh(&mut self) {
        self.cai_dat = CaiDatGameJson::default();
        self.dong_bo_len_ui();
        self.luu_va_ap_dung();
    }

    #[func]
    pub fn lay_thong_tin_co_ban(&self) -> GString {
        let (w, h) = self.cai_dat.lay_kich_thuoc_cua_so();
        GString::from(format!(
            "Tốc độ: {:.2}x | Độ trễ: {:.0}ms | Màn hình: {}x{} | FPS tối đa: {}",
            self.cai_dat.toc_do_choi,
            self.cai_dat.do_tre_phim_ms,
            w,
            h,
            self.cai_dat.gioi_han_fps
        ))
    }

    // ==================== CALLBACKS TỪ GIAO DIỆN ====================
    #[func]
    fn _khi_keo_toc_do(&mut self, val: f64) {
        self.set_toc_do_choi(val as f32);
    }
    #[func]
    fn _khi_keo_do_tre(&mut self, val: f64) {
        self.set_do_tre_phim_ms(val as f32);
    }
    #[func]
    fn _khi_keo_am_luong(&mut self, val: f64) {
        self.set_am_luong_chung(val as f32);
    }
    #[func]
    fn _khi_bam_toan_man_hinh(&mut self, pressed: bool) {
        self.cai_dat.toan_man_hinh = pressed;
    }
    #[func]
    fn _khi_bam_vsync(&mut self, pressed: bool) {
        self.cai_dat.bat_vsync = pressed;
    }
    #[func]
    fn _khi_bam_hien_fps(&mut self, pressed: bool) {
        self.cai_dat.hien_thi_fps = pressed;
    }
    #[func]
    fn _khi_chon_phan_giai(&mut self, idx: i64) {
        self.cai_dat.do_phan_giai_idx = idx as i32;
    }
    #[func]
    fn _khi_chon_fps(&mut self, idx: i64) {
        self.cai_dat.gioi_han_fps = DANH_SACH_FPS.get(idx as usize).copied().unwrap_or(60);
    }
    #[func]
    fn _ve_menu_chinh(&mut self) {
        if let Some(mut tree) = self.base().get_tree() {
            tree.change_scene_to_file("res://scene/menu/MenuChinh.tscn");
        }
    }
}

#[godot_api]
impl IControl for ManHinhCaiDat {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            cai_dat: CaiDatGameJson::default(),
        }
    }

    fn ready(&mut self) {
        self.cai_dat = CaiDatGameJson::doc_cai_dat();
        self.dong_bo_len_ui();
        self.ap_dung_cai_dat_he_thong();

        let obj = self.to_gd();
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("ThanhTren/HBox/MacDinh_Btn")
        {
            b.connect(
                "pressed",
                &Callable::from_object_method(&obj, "dat_lai_mac_dinh"),
            );
        }
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("ThanhTren/HBox/LuuCaiDat_Btn")
        {
            b.connect(
                "pressed",
                &Callable::from_object_method(&obj, "luu_va_ap_dung"),
            );
        }
        if let Some(mut b) = self
            .base_mut()
            .try_get_node_as::<Button>("ThanhTren/HBox/VeMenu_Btn")
        {
            b.connect(
                "pressed",
                &Callable::from_object_method(&obj, "_ve_menu_chinh"),
            );
        }

        if let Some(mut s) = self
            .base_mut()
            .try_get_node_as::<HSlider>("KhungChinh/HBox/CotChoiNhac/VBox/HangTocDo/TocDo_Slider")
        {
            s.connect(
                "value_changed",
                &Callable::from_object_method(&obj, "_khi_keo_toc_do"),
            );
        }
        if let Some(mut s) = self
            .base_mut()
            .try_get_node_as::<HSlider>("KhungChinh/HBox/CotChoiNhac/VBox/HangDoTre/DoTre_Slider")
        {
            s.connect(
                "value_changed",
                &Callable::from_object_method(&obj, "_khi_keo_do_tre"),
            );
        }
        if let Some(mut s) = self.base_mut().try_get_node_as::<HSlider>(
            "KhungChinh/HBox/CotChoiNhac/VBox/HangAmLuong/AmLuong_Slider",
        ) {
            s.connect(
                "value_changed",
                &Callable::from_object_method(&obj, "_khi_keo_am_luong"),
            );
        }

        if let Some(mut chk) = self
            .base_mut()
            .try_get_node_as::<CheckButton>("KhungChinh/HBox/CotDoHoa/VBox/ToanManHinh_Check")
        {
            chk.connect(
                "toggled",
                &Callable::from_object_method(&obj, "_khi_bam_toan_man_hinh"),
            );
        }
        if let Some(mut chk) = self
            .base_mut()
            .try_get_node_as::<CheckButton>("KhungChinh/HBox/CotDoHoa/VBox/VSync_Check")
        {
            chk.connect(
                "toggled",
                &Callable::from_object_method(&obj, "_khi_bam_vsync"),
            );
        }
        if let Some(mut chk) = self
            .base_mut()
            .try_get_node_as::<CheckButton>("KhungChinh/HBox/CotDoHoa/VBox/HienThiFps_Check")
        {
            chk.connect(
                "toggled",
                &Callable::from_object_method(&obj, "_khi_bam_hien_fps"),
            );
        }

        if let Some(mut opt) = self.base_mut().try_get_node_as::<OptionButton>(
            "KhungChinh/HBox/CotDoHoa/VBox/HangPhanGiai/DoPhanGiai_Option",
        ) {
            opt.connect(
                "item_selected",
                &Callable::from_object_method(&obj, "_khi_chon_phan_giai"),
            );
        }
        if let Some(mut opt) = self.base_mut().try_get_node_as::<OptionButton>(
            "KhungChinh/HBox/CotDoHoa/VBox/HangFps/GioiHanFps_Option",
        ) {
            opt.connect(
                "item_selected",
                &Callable::from_object_method(&obj, "_khi_chon_fps"),
            );
        }
    }
}