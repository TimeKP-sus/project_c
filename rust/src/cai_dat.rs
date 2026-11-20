use godot::{
    builtin::{varray, Callable, GString, StringName, Variant, Vector2i},
    classes::{
        AudioServer, Button, CheckButton, Control, DisplayServer, Engine, HSlider, IControl,
        InputEvent, InputEventKey, Label, OptionButton, Os, Panel, VBoxContainer, HBoxContainer,
        control::SizeFlags,
    },
    global::Key,
    obj::{Base, EngineEnum, Gd, NewAlloc, WithBaseField},
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
    not_dang_cho_gan: Option<i32>,
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
            s.set_value_no_signal(am_luong_chung as f64);
        }
        if let Some(mut lbl) = self
            .base_mut()
            .try_get_node_as::<Label>("KhungChinh/HBox/CotChoiNhac/VBox/HangAmLuong/AmLuong_Val")
        {
            lbl.set_text(&format!("{:.0}%", am_luong_chung));
        }
        self.ap_dung_am_luong();
    }

    #[func]
    pub fn get_am_luong_chung(&self) -> f32 {
        self.cai_dat.am_luong_chung
    }

    fn ap_dung_bus_am_luong(audio: &mut Gd<AudioServer>, bus_name: &str, volume_pct: f32) {
        let ten_bus = StringName::from(bus_name);
        let bus_idx = audio.get_bus_index(&ten_bus);
        if bus_idx < 0 {
            return;
        }
        if volume_pct <= 1.0 {
            audio.set_bus_mute(bus_idx, true);
        } else {
            audio.set_bus_mute(bus_idx, false);
            audio.set_bus_volume_db(bus_idx, (volume_pct - 100.0) * 0.4);
        }
    }

    fn ap_dung_am_luong(&self) {
        let mut audio = AudioServer::singleton();
        Self::ap_dung_bus_am_luong(&mut audio, "Master", self.cai_dat.am_luong_chung);
        Self::ap_dung_bus_am_luong(&mut audio, "Nhac", self.cai_dat.am_luong_nhac);
        Self::ap_dung_bus_am_luong(&mut audio, "Loi", self.cai_dat.am_luong_loi);
        crate::sf2_read::cap_nhat_am_luong_piano(self.cai_dat.am_luong_piano);
    }

    fn cap_nhat_thanh_am_luong(&mut self, ten_hang: &str, ten_slider: &str, ten_label: &str, am_luong: f32) {
        let am_luong = am_luong.clamp(0.0, 100.0);
        match ten_hang {
            "piano" => self.cai_dat.am_luong_piano = am_luong,
            "nhac" => self.cai_dat.am_luong_nhac = am_luong,
            "loi" => self.cai_dat.am_luong_loi = am_luong,
            _ => return,
        }
        if let Some(mut slider) = self.base_mut().try_get_node_as::<HSlider>(ten_slider) {
            slider.set_value_no_signal(am_luong as f64);
        }
        if let Some(mut label) = self.base_mut().try_get_node_as::<Label>(ten_label) {
            label.set_text(&format!("{:.0}%", am_luong));
        }
    }

    #[func]
    pub fn set_am_luong_piano(&mut self, am_luong: f32) {
        self.cap_nhat_thanh_am_luong(
            "piano",
            "KhungChinh/HBox/CotChoiNhac/VBox/HangAmPiano/AmPiano_Slider",
            "KhungChinh/HBox/CotChoiNhac/VBox/HangAmPiano/AmPiano_Val",
            am_luong,
        );
        self.ap_dung_am_luong();
    }

    #[func]
    pub fn get_am_luong_piano(&self) -> f32 {
        self.cai_dat.am_luong_piano
    }

    #[func]
    pub fn set_am_luong_nhac(&mut self, am_luong: f32) {
        self.cap_nhat_thanh_am_luong(
            "nhac",
            "KhungChinh/HBox/CotChoiNhac/VBox/HangAmNhac/AmNhac_Slider",
            "KhungChinh/HBox/CotChoiNhac/VBox/HangAmNhac/AmNhac_Val",
            am_luong,
        );
        self.ap_dung_am_luong();
    }

    #[func]
    pub fn get_am_luong_nhac(&self) -> f32 {
        self.cai_dat.am_luong_nhac
    }

    #[func]
    pub fn set_am_luong_loi(&mut self, am_luong: f32) {
        self.cap_nhat_thanh_am_luong(
            "loi",
            "KhungChinh/HBox/CotChoiNhac/VBox/HangAmLoi/AmLoi_Slider",
            "KhungChinh/HBox/CotChoiNhac/VBox/HangAmLoi/AmLoi_Val",
            am_luong,
        );
        self.ap_dung_am_luong();
    }

    #[func]
    pub fn get_am_luong_loi(&self) -> f32 {
        self.cai_dat.am_luong_loi
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

        // 2. Áp dụng âm lượng Godot và âm lượng Piano của Rustysynth
        self.ap_dung_am_luong();

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
        self.set_am_luong_piano(cfg.am_luong_piano);
        self.set_am_luong_nhac(cfg.am_luong_nhac);
        self.set_am_luong_loi(cfg.am_luong_loi);
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
    fn _khi_keo_am_piano(&mut self, val: f64) {
        self.set_am_luong_piano(val as f32);
    }
    #[func]
    fn _khi_keo_am_nhac(&mut self, val: f64) {
        self.set_am_luong_nhac(val as f32);
    }
    #[func]
    fn _khi_keo_am_loi(&mut self, val: f64) {
        self.set_am_luong_loi(val as f32);
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

    fn ten_not_tu_id(so_phim: i32) -> String {
        const TEN: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        format!("{}{} (ID: {})", TEN[(so_phim % 12) as usize], so_phim / 12 + 1, so_phim)
    }

    fn tao_danh_sach_gan_phim(&mut self) {
        let obj = self.to_gd();
        let ban_do = self.cai_dat.gan_phim_midi.clone();
        let dang_cho = self.not_dang_cho_gan;
        let Some(mut vbox) = self.base_mut().try_get_node_as::<VBoxContainer>(
            "PanelGanPhim/VBox/Scroll/DanhSachPhim_VBox",
        ) else {
            return;
        };
        for mut child in vbox.get_children().iter_shared() {
            vbox.remove_child(&child);
            child.queue_free();
        }
        for so_phim in 0..84 {
            let mut hang = HBoxContainer::new_alloc();
            hang.add_theme_constant_override("separation", 12);
            let mut lbl = Label::new_alloc();
            lbl.set_text(&Self::ten_not_tu_id(so_phim));
            lbl.set_h_size_flags(SizeFlags::EXPAND_FILL);
            lbl.add_theme_font_size_override("font_size", 18);
            hang.add_child(&lbl);

            let mut nut_gan = Button::new_alloc();
            nut_gan.set_custom_minimum_size(godot::builtin::Vector2::new(200.0, 38.0));
            let text_nut = if dang_cho == Some(so_phim) {
                "Nhấn phím bất kỳ...".to_string()
            } else if let Some(&keycode) = ban_do.get(&so_phim) {
                Os::singleton().get_keycode_string(Key::from_ord(keycode as i32)).to_string()
            } else {
                "— Chưa gắn —".to_string()
            };
            nut_gan.set_text(&text_nut);
            nut_gan.connect(
                "pressed",
                &Callable::from_object_method(&obj, "_khi_bam_gan_not")
                    .bindv(&varray![so_phim]),
            );
            hang.add_child(&nut_gan);

            let mut nut_xoa = Button::new_alloc();
            nut_xoa.set_custom_minimum_size(godot::builtin::Vector2::new(46.0, 38.0));
            nut_xoa.set_text("X");
            nut_xoa.connect(
                "pressed",
                &Callable::from_object_method(&obj, "_khi_xoa_gan_not")
                    .bindv(&varray![so_phim]),
            );
            hang.add_child(&nut_xoa);
            vbox.add_child(&hang);
        }
    }

    #[func]
    fn _mo_panel_gan_phim(&mut self) {
        self.not_dang_cho_gan = None;
        let bat_ban_phim = self.cai_dat.bam_bang_ban_phim;
        if let Some(mut chk) = self.base_mut().try_get_node_as::<CheckButton>(
            "PanelGanPhim/VBox/BatBanPhim_Check",
        ) {
            chk.set_pressed_no_signal(bat_ban_phim);
        }
        self.tao_danh_sach_gan_phim();
        if let Some(mut panel) = self.base_mut().try_get_node_as::<Panel>("PanelGanPhim") {
            panel.set_visible(true);
        }
    }

    #[func]
    fn _dong_panel_gan_phim(&mut self) {
        self.not_dang_cho_gan = None;
        if let Some(mut panel) = self.base_mut().try_get_node_as::<Panel>("PanelGanPhim") {
            panel.set_visible(false);
        }
    }

    #[func]
    fn _khi_bat_tat_ban_phim(&mut self, pressed: bool) {
        self.cai_dat.bam_bang_ban_phim = pressed;
    }

    #[func]
    fn _khi_bam_gan_not(&mut self, so_phim: i32) {
        self.not_dang_cho_gan = Some(so_phim);
        self.tao_danh_sach_gan_phim();
    }

    #[func]
    fn _khi_xoa_gan_not(&mut self, so_phim: i32) {
        self.cai_dat.gan_phim_midi.remove(&so_phim);
        self.not_dang_cho_gan = None;
        self.tao_danh_sach_gan_phim();
    }

    #[func]
    fn _khi_dat_lai_phim_mac_dinh(&mut self) {
        self.cai_dat.gan_phim_midi = CaiDatGameJson::default().gan_phim_midi;
        self.not_dang_cho_gan = None;
        self.tao_danh_sach_gan_phim();
    }

    #[func]
    fn _khi_xoa_het_phim(&mut self) {
        self.cai_dat.gan_phim_midi.clear();
        self.not_dang_cho_gan = None;
        self.tao_danh_sach_gan_phim();
    }
}

#[godot_api]
impl IControl for ManHinhCaiDat {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            cai_dat: CaiDatGameJson::default(),
            not_dang_cho_gan: None,
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
        for (path, method) in [
            ("KhungChinh/HBox/CotChoiNhac/VBox/HangAmPiano/AmPiano_Slider", "_khi_keo_am_piano"),
            ("KhungChinh/HBox/CotChoiNhac/VBox/HangAmNhac/AmNhac_Slider", "_khi_keo_am_nhac"),
            ("KhungChinh/HBox/CotChoiNhac/VBox/HangAmLoi/AmLoi_Slider", "_khi_keo_am_loi"),
        ] {
            if let Some(mut slider) = self.base_mut().try_get_node_as::<HSlider>(path) {
                slider.connect("value_changed", &Callable::from_object_method(&obj, method));
            }
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

        for (path, method) in [
            ("KhungChinh/HBox/CotChoiNhac/VBox/GanPhimMidi_Btn", "_mo_panel_gan_phim"),
            ("PanelGanPhim/VBox/HangNut/DongPanelPhim_Btn", "_dong_panel_gan_phim"),
            ("PanelGanPhim/VBox/HangNut/MacDinhPhim_Btn", "_khi_dat_lai_phim_mac_dinh"),
            ("PanelGanPhim/VBox/HangNut/XoaHetPhim_Btn", "_khi_xoa_het_phim"),
        ] {
            if let Some(mut button) = self.base_mut().try_get_node_as::<Button>(path) {
                button.connect("pressed", &Callable::from_object_method(&obj, method));
            }
        }
        if let Some(mut check) = self.base_mut().try_get_node_as::<CheckButton>(
            "PanelGanPhim/VBox/BatBanPhim_Check",
        ) {
            check.connect(
                "toggled",
                &Callable::from_object_method(&obj, "_khi_bat_tat_ban_phim"),
            );
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let Some(so_phim) = self.not_dang_cho_gan else {
            return;
        };
        if let Ok(key_event) = event.try_cast::<InputEventKey>() {
            if !key_event.is_pressed() || key_event.is_echo() {
                return;
            }
            let key = key_event.get_physical_keycode();
            if key == Key::ESCAPE {
                self.not_dang_cho_gan = None;
                self.tao_danh_sach_gan_phim();
            } else if key != Key::NONE {
                let code = key.ord() as i64;
                self.cai_dat.gan_phim_midi.retain(|_, value| *value != code);
                self.cai_dat.gan_phim_midi.insert(so_phim, code);
                self.not_dang_cho_gan = None;
                self.tao_danh_sach_gan_phim();
            }
            if let Some(mut viewport) = self.base().get_viewport() {
                viewport.set_input_as_handled();
            }
        }
    }
}