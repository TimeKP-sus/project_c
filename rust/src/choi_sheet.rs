use godot::builtin::{ Callable, Color, GString, NodePath, Variant };
use godot::classes::{
    AudioStream, AudioStreamMp3, AudioStreamOggVorbis, AudioStreamPlayer2D,
    Button, CheckButton, Control, FileAccess, IControl, Label, Object, ResourceLoader,
};
use godot::obj::{ Gd, GdRef, NewGd, WithBaseField };
use godot::{ obj::Base, prelude::{ GodotClass, godot_api } };

use crate::ban_phim_midi::BanPhimMidi;
use crate::doc_sheet_json;
use crate::du_lieu_lich_su::DuLieuLichSu;
use crate::khuong_nhac::KhuongNhac;
use crate::not_nhac::NotNhac;
use crate::quan_ly_sheet::QuanLySheet;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct ChoiSheet {
    #[base]
    base: Base<Control>,
    diem: i32,
    diem_ky_luc: i32,
    ban_phim_midi_node: Option<Gd<BanPhimMidi>>,
    quan_ly_sheet_node: Option<Gd<QuanLySheet>>,
    nhac_player: Option<Gd<AudioStreamPlayer2D>>,
    loi_player: Option<Gd<AudioStreamPlayer2D>>,

    #[export]
    ten_bai_hat_label: Option<Gd<Label>>,
    #[export]
    tac_gia_label: Option<Gd<Label>>,
    #[export]
    nguoi_tao_label: Option<Gd<Label>>,
    #[export]
    che_do_choi_thu: bool,
    #[export]
    phan_luyen_tap: bool,
    #[export]
    duong_dan_bai_hoc: GString,

    cho_phep_phat_nhac: bool,
    cho_phep_phat_loi: bool,
    co_stream_nhac: bool,
    co_stream_loi: bool,
    da_bat_dau_audio: bool,

    so_phim_tot: i32,
    so_phim_kha: i32,
    so_phim_truot: i32,
}

#[godot_api]
impl ChoiSheet {
    #[func]
    pub fn get_diem(&self) -> i32 {
        self.diem
    }
    #[func]
    pub fn get_so_phim_tot(&self) -> i32 {
        self.so_phim_tot
    }
    #[func]
    pub fn get_so_phim_kha(&self) -> i32 {
        self.so_phim_kha
    }
    #[func]
    pub fn get_so_phim_truot(&self) -> i32 {
        self.so_phim_truot
    }

    /// Tải file âm thanh hỗ trợ cả file trong gói res:// và file nằm ngoài thư mục khi xuất game
    fn tai_audio_stream(duong_dan: &str) -> Option<Gd<AudioStream>> {
        let p = duong_dan.trim();
        if p.is_empty() {
            return None;
        }
        let lower = p.to_lowercase();
        if lower.ends_with(".mp3") {
            let bytes = FileAccess::get_file_as_bytes(p);
            if !bytes.is_empty() {
                let mut stream = AudioStreamMp3::new_gd();
                stream.set_data(&bytes);
                return Some(stream.upcast::<AudioStream>());
            }
        } else if lower.ends_with(".ogg") {
            if let Some(stream) = AudioStreamOggVorbis::load_from_file(p) {
                return Some(stream.upcast::<AudioStream>());
            }
        }
        ResourceLoader::singleton()
            .load(p)
            .and_then(|res| res.try_cast::<AudioStream>().ok())
    }

    fn thiet_lap_am_thanh_bai_hat(&mut self) {
        self.co_stream_nhac = false;
        self.co_stream_loi = false;
        self.da_bat_dau_audio = false;

        let (path_nhac, path_loi, bpm_ht, bpm_goc) = if let Some(qls) = self.quan_ly_sheet_node.as_ref() {
            let b = qls.bind();
            (
                b.get_duong_dan_nhac().to_string(),
                b.get_duong_dan_loi().to_string(),
                b.bpm,
                b.get_bpm_goc(),
            )
        } else {
            return;
        };

        let ti_le_toc_do = if bpm_goc > 0.0 && bpm_ht > 0.0 {
            (bpm_ht / bpm_goc).clamp(0.25, 4.0)
        } else {
            1.0
        };

        if let Some(mut np) = self.nhac_player.clone() {
            np.stop();
            if let Some(stream) = Self::tai_audio_stream(&path_nhac) {
                np.set_stream(&stream);
                np.set_pitch_scale(ti_le_toc_do);
                self.co_stream_nhac = true;
            }
        }

        if let Some(mut lp) = self.loi_player.clone() {
            lp.stop();
            if let Some(stream) = Self::tai_audio_stream(&path_loi) {
                lp.set_stream(&stream);
                lp.set_pitch_scale(ti_le_toc_do);
                self.co_stream_loi = true;
            }
        }
    }

    fn tinh_vi_tri_audio_hien_tai(&self) -> f32 {
        if let Some(qls) = self.quan_ly_sheet_node.as_ref() {
            let b = qls.bind();
            let tg = b.get_thoi_gian().max(0.0);
            let bpm_ht = b.bpm;
            let bpm_goc = b.get_bpm_goc();
            let nhip_bd = b.get_nhip_bat_dau_luyen_tap();
            let ti_le = if bpm_goc > 0.0 && bpm_ht > 0.0 { bpm_ht / bpm_goc } else { 1.0 };
            let giay_bat_dau_goc = if bpm_goc > 0.0 { (nhip_bd * 60.0) / bpm_goc } else { 0.0 };
            giay_bat_dau_goc + tg * ti_le
        } else {
            0.0
        }
    }

    #[func]
    pub fn bat_tat_phat_nhac(&mut self, bat: bool) {
        self.cho_phep_phat_nhac = bat;
        let vi_tri = self.tinh_vi_tri_audio_hien_tai();
        let dang_dung = self.quan_ly_sheet_node.as_ref().map(|q| q.bind().get_da_dung()).unwrap_or(false);

        if let Some(mut np) = self.nhac_player.clone() {
            if !bat {
                np.stop();
            } else if self.co_stream_nhac && self.da_bat_dau_audio {
                np.play_ex().from_position(vi_tri).done();
                np.set_stream_paused(dang_dung);
            }
        }
    }

    #[func]
    pub fn bat_tat_phat_loi(&mut self, bat: bool) {
        self.cho_phep_phat_loi = bat;
        let vi_tri = self.tinh_vi_tri_audio_hien_tai();
        let dang_dung = self.quan_ly_sheet_node.as_ref().map(|q| q.bind().get_da_dung()).unwrap_or(false);

        if let Some(mut lp) = self.loi_player.clone() {
            if !bat {
                lp.stop();
            } else if self.co_stream_loi && self.da_bat_dau_audio {
                lp.play_ex().from_position(vi_tri).done();
                lp.set_stream_paused(dang_dung);
            }
        }
    }

    #[func]
    pub fn bat_tat_che_do_cho(&mut self, bat: bool) {
        if let Some(mut node) = self.quan_ly_sheet_node.clone() {
            node.bind_mut().set_cho_phep_cho(bat);
        }
    }

    pub fn set_thong_tin_bai_hat(&mut self) {
        let ten_bai_hat: Option<GString> = self.quan_ly_sheet_node.as_ref().map(|node| {
            let sheet_bind: GdRef<'_, QuanLySheet> = node.bind();
            sheet_bind.get_ten_bai_hat()
        });

        if let Some(ten_bai_hat) = ten_bai_hat {
            if let Some(mut label) = self.ten_bai_hat_label.clone() {
                label.set_text(&ten_bai_hat);
                self.hieu_ung_xuat_hien_label(&mut label);
            }
        }
    }

    pub fn hieu_ung_xuat_hien_label(&mut self, label: &mut Gd<Label>) {
        label.set_visible(true);
        let mut mau_goc: Color = label.get_modulate();
        mau_goc.a = 0.0;
        label.set_modulate(mau_goc);
        let mut mau_hien: Color = mau_goc;
        mau_hien.a = 1.0;

        let mut mau_an: Color = mau_goc;
        mau_an.a = 0.0;

        let doi_tuong: Gd<Object> = label.clone().upcast::<Object>();

        if let Some(mut tween) = self.base_mut().create_tween() {
            tween.tween_property(
                &doi_tuong,
                &NodePath::from("modulate"),
                &Variant::from(mau_hien),
                0.3,
            );
            tween.tween_interval(4.0);
            tween.tween_property(
                &doi_tuong,
                &NodePath::from("modulate"),
                &Variant::from(mau_an),
                0.5,
            );
        }
    }

    pub fn cat_nhat_diem_label(&mut self, diem_moi: i32) {
        self.diem = diem_moi;
        if let Some(mut label) = self.base().try_get_node_as::<Label>("diem") {
            label.set_text(&format!("Điểm: {}", self.diem));
        }
    }

    #[func]
    pub fn kiem_tra_phim(&mut self, so_phim: i32) {
        let cac_not_trong_area: Vec<Gd<NotNhac>> = self
            .quan_ly_sheet_node
            .as_mut()
            .unwrap()
            .bind_mut()
            .get_danh_sach_not_trong_area();

        for mut not_node in cac_not_trong_area {
            let id_not: i32 = not_node.bind().get_id_not();
            let da_danh: bool = not_node.bind().get_da_duoc_danh();

            if id_not == so_phim && !da_danh {
                not_node.bind_mut().bat_dau_giu();
                self.xu_ly_not_ra_area(so_phim);

                let vi_tri_x: f32 = not_node.get_position().x;
                let chieu_dai_duoi: f32 = not_node.bind().get_chieu_dai_duoi();

                if chieu_dai_duoi < 0.0 {
                    let do_lech: f32 = (vi_tri_x - 130.0).abs();

                    if do_lech <= 15.0 {
                        self.diem += 100;
                        self.so_phim_tot += 1;
                    } else if do_lech <= 40.0 {
                        self.diem += 50;
                        self.so_phim_kha += 1;
                    }
                    self.cat_nhat_diem_label(self.diem);
                }
                break;
            }
        }
    }

    #[func]
    pub fn xu_ly_nha_phim(&mut self, so_phim: i32) {
        let tat_ca_not = self
            .quan_ly_sheet_node
            .as_mut()
            .unwrap()
            .bind_mut()
            .get_tat_ca_not_tren_sheet();

        for mut not_node in tat_ca_not {
            let id_not: i32 = not_node.bind().get_id_not();
            let dang_giu: bool = not_node.bind().get_dang_giu();

            if id_not == so_phim && dang_giu {
                not_node.bind_mut().ket_thuc_giu();

                let chieu_dai_duoi: f32 = not_node.bind().get_chieu_dai_duoi();

                if chieu_dai_duoi >= 0.0 {
                    let vi_tri_x: f32 = not_node.get_position().x;
                    let toa_do_ket_thuc: f32 = vi_tri_x + chieu_dai_duoi;

                    if toa_do_ket_thuc <= 150.0 {
                        self.diem += 200;
                        self.so_phim_tot += 1;
                    } else if toa_do_ket_thuc <= 250.0 {
                        self.diem += 50;
                        self.so_phim_kha += 1;
                    }
                    self.cat_nhat_diem_label(self.diem);
                }
                break;
            }
        }
    }

    #[func]
    pub fn xu_ly_not_bi_truot(&mut self) {
        self.so_phim_truot += 1;
        self.cat_nhat_diem_label(self.diem);
    }

    #[func]
    pub fn reset_diem(&mut self) {
        self.diem = 0;
        self.so_phim_tot = 0;
        self.so_phim_kha = 0;
        self.so_phim_truot = 0;
        self.da_bat_dau_audio = false;
        if let Some(mut np) = self.nhac_player.clone() {
            np.stop();
        }
        if let Some(mut lp) = self.loi_player.clone() {
            lp.stop();
        }
        self.cat_nhat_diem_label(self.diem);
    }

    pub fn lay_ten_che_do_luu(&self) -> &'static str {
        if self.phan_luyen_tap { "luyen_tap_sheet" } else { "choi_sheet" }
    }

    #[func]
    pub fn lay_ti_le_hoan_thanh(&self) -> f32 {
        let tong_not = self
            .quan_ly_sheet_node
            .as_ref()
            .map(|q| q.bind().get_tong_so_not_sheet())
            .unwrap_or(0);
        let da_danh_trung = self.so_phim_tot + self.so_phim_kha;
        let mau_so = tong_not.max(da_danh_trung + self.so_phim_truot);
        if mau_so <= 0 {
            return 0.0;
        }
        (((da_danh_trung as f32) / (mau_so as f32)) * 100.0).clamp(0.0, 100.0)
    }

    #[func]
    pub fn lay_diem_cao_nhat_cu(&self) -> i32 {
        self.diem_ky_luc
    }

    #[func]
    pub fn thoat_choi_thu(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    pub fn tong_ket(&mut self) {
        if let Some(mut np) = self.nhac_player.clone() {
            np.stop();
        }
        if let Some(mut lp) = self.loi_player.clone() {
            lp.stop();
        }

        if self.duong_dan_bai_hoc.is_empty() || self.che_do_choi_thu {
            return;
        }
        let ti_le = self.lay_ti_le_hoan_thanh();
        let hang = DuLieuLichSu::tinh_hang(ti_le);
        let che_do = self.lay_ten_che_do_luu();

        let mut ls: DuLieuLichSu = DuLieuLichSu::doc_lich_su();
        self.diem_ky_luc = ls
            .lay_ket_qua(che_do, &self.duong_dan_bai_hoc.to_string())
            .map(|kq| kq.diem_cao_nhat)
            .unwrap_or(0);

        ls.cap_nhat_ket_qua(che_do, &self.duong_dan_bai_hoc.to_string(), self.diem, ti_le, hang);
    }

    fn lay_mau_to_phim(mau: &str) -> Color {
        match mau {
            "1" => Color::from_rgba(0.85, 0.25, 0.25, 0.75),
            "2" => Color::from_rgba(0.2, 0.75, 0.35, 0.75),
            "3" => Color::from_rgba(0.25, 0.55, 0.9, 0.75),
            "4" => Color::from_rgba(0.95, 0.75, 0.15, 0.75),
            "5" => Color::from_rgba(0.6, 0.3, 0.8, 0.75),
            _ => Color::from_rgba(0.2, 0.85, 1.0, 0.75),
        }
    }

    #[func]
    pub fn bat_tat_to_mau_phim(&mut self, bat: bool) {
        let cho_phep = bat && self.phan_luyen_tap;
        if let Some(mut qls) = self.quan_ly_sheet_node.clone() {
            qls.bind_mut().bat_tat_to_mau_phim(cho_phep);
        }

        if let Some(mut midi) = self.ban_phim_midi_node.clone() {
            if !cho_phep {
                for id in 0..84 {
                    midi.bind_mut().dung_to_phim(id);
                }
            } else if let Some(mut qls) = self.quan_ly_sheet_node.clone() {
                let ds = qls.bind_mut().get_danh_sach_not_trong_area();
                for not in ds {
                    let b = not.bind();
                    if b.get_id_not() != 100 && !b.get_da_duoc_danh() {
                        let mau: Color = Self::lay_mau_to_phim(&b.get_mau_not());
                        midi.bind_mut().to_phim(b.get_id_not(), mau);
                    }
                }
            }
        }
    }

    #[func]
    pub fn xu_ly_not_vao_area(&mut self, id_not: i32, mau_not: GString) {
        let cho_phep = self
            .quan_ly_sheet_node
            .as_ref()
            .map(|q| q.bind().get_cho_phep_to_mau_phim())
            .unwrap_or(false);
        if !cho_phep || id_not == 100 {
            return;
        }
        if let Some(mut midi) = self.ban_phim_midi_node.clone() {
            let mau = Self::lay_mau_to_phim(&mau_not.to_string());
            midi.bind_mut().to_phim(id_not, mau);
        }
    }

    #[func]
    pub fn xu_ly_not_ra_area(&mut self, id_not: i32) {
        if id_not == 100 {
            return;
        }
        let mut con_not_cung_phim = false;
        let mut mau_con_lai = String::new();
        if let Some(mut qls) = self.quan_ly_sheet_node.clone() {
            let ds = qls.bind_mut().get_danh_sach_not_trong_area();
            for not in ds {
                let b = not.bind();
                if b.get_id_not() == id_not && !b.get_da_duoc_danh() {
                    con_not_cung_phim = true;
                    mau_con_lai = b.get_mau_not();
                    break;
                }
            }
        }

        if let Some(mut midi) = self.ban_phim_midi_node.clone() {
            let cho_phep = self
                .quan_ly_sheet_node
                .as_ref()
                .map(|q| q.bind().get_cho_phep_to_mau_phim())
                .unwrap_or(false);
            if con_not_cung_phim && cho_phep {
                midi.bind_mut().to_phim(id_not, Self::lay_mau_to_phim(&mau_con_lai));
            } else {
                midi.bind_mut().dung_to_phim(id_not);
            }
        }
    }
}

#[godot_api]
impl IControl for ChoiSheet {
    fn ready(&mut self) {
        self.ban_phim_midi_node = self.base().try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano");
        self.quan_ly_sheet_node = self.base().try_get_node_as::<QuanLySheet>("QuanLySheet");
        self.nhac_player = self.base().try_get_node_as::<AudioStreamPlayer2D>("nhac");
        self.loi_player = self.base().try_get_node_as::<AudioStreamPlayer2D>("loi");
        self.cho_phep_phat_nhac = true;
        self.cho_phep_phat_loi = true;

        self.quan_ly_sheet_node.as_mut().unwrap().bind_mut().bat_tat_cho_phep_xoa_not(true);
        let check_phim_midi: Callable = self.base().callable("kiem_tra_phim");
        let nha_phim_midi: Callable = self.base().callable("xu_ly_nha_phim");

        if let Some(mut midi_node) = self.ban_phim_midi_node.clone() {
            midi_node.connect("phim_vua_duoc_bam", &check_phim_midi);
            midi_node.connect("phim_vua_duoc_nha", &nha_phim_midi);
        }

        let callable_truot = self.base().callable("xu_ly_not_bi_truot");
        let callable_vao = self.base().callable("xu_ly_not_vao_area");
        let callable_ra = self.base().callable("xu_ly_not_ra_area");

        let khuong_1 = self.base().try_get_node_as::<KhuongNhac>("QuanLySheet/K1");
        let khuong_2 = self.base().try_get_node_as::<KhuongNhac>("QuanLySheet/K2");

        if let Some(mut k1) = khuong_1 {
            k1.connect("not_bi_truot", &callable_truot);
            k1.connect("not_vao_area", &callable_vao);
            k1.connect("not_ra_area", &callable_ra);
        }
        if let Some(mut k2) = khuong_2 {
            k2.connect("not_bi_truot", &callable_truot);
            k2.connect("not_vao_area", &callable_vao);
            k2.connect("not_ra_area", &callable_ra);
        }

        // Kết nối các nút bật/tắt trên thanh Menu của ChoiSheet (nếu có)
        let call_to_phim = self.base().callable("bat_tat_to_mau_phim");
        if let Some(mut chk) = self.base().try_get_node_as::<CheckButton>("NutMenu/Menu/to_mau_phim") {
            if !chk.is_connected("toggled", &call_to_phim) {
                chk.connect("toggled", &call_to_phim);
            }
        }
        let call_nhac = self.base().callable("bat_tat_phat_nhac");
        if let Some(mut chk) = self.base().try_get_node_as::<CheckButton>("NutMenu/Menu/bat_tat_nhac") {
            if !chk.is_connected("toggled", &call_nhac) {
                chk.connect("toggled", &call_nhac);
            }
        }
        let call_loi = self.base().callable("bat_tat_phat_loi");
        if let Some(mut chk) = self.base().try_get_node_as::<CheckButton>("NutMenu/Menu/bat_tat_loi") {
            if !chk.is_connected("toggled", &call_loi) {
                chk.connect("toggled", &call_loi);
            }
        }

        let duong_dan: GString = self.duong_dan_bai_hoc.clone();
        if !duong_dan.is_empty() {
            if self.phan_luyen_tap {
                if let Some(du_lieu_lt) = doc_sheet_json::doc_file_luyen_tap(&duong_dan.to_string()) {
                    self.cho_phep_phat_nhac = du_lieu_lt.phat_nhac;
                    self.cho_phep_phat_loi = du_lieu_lt.phat_loi;

                    if let Some(mut quan_ly_sheet) = self.quan_ly_sheet_node.clone() {
                        quan_ly_sheet.bind_mut().set_che_do_luyen_tap(true);
                        quan_ly_sheet.bind_mut().set_cho_phep_cho(du_lieu_lt.cho_phep_cho);
                        quan_ly_sheet.bind_mut().bat_tat_am_thanh_nhip(du_lieu_lt.cho_phep_am_thanh_nhip);
                        quan_ly_sheet.bind_mut().bat_tat_hien_thi_ten_khac(du_lieu_lt.hien_thi_ten_khac);
                        quan_ly_sheet.bind_mut().bat_tat_to_mau_phim(du_lieu_lt.to_mau_phim);

                        quan_ly_sheet.bind_mut().khoi_tao_sheet(
                            du_lieu_lt.duong_dan_sheet.into(),
                            du_lieu_lt.khuong.into(),
                            du_lieu_lt.phan_thoi_gian.into(),
                        );
                        if let Some(bpm_moi) = du_lieu_lt.bpm_tuy_chinh {
                            if bpm_moi > 0.0 {
                                quan_ly_sheet.bind_mut().bpm = bpm_moi;
                            }
                        }
                    }

                    if let Some(mut chk) = self.base().try_get_node_as::<CheckButton>("NutMenu/Menu/bat_tat_nhip") {
                        chk.set_pressed_no_signal(du_lieu_lt.cho_phep_am_thanh_nhip);
                    }
                    if let Some(mut chk) = self.base().try_get_node_as::<CheckButton>("NutMenu/Menu/che_do_cho") {
                        chk.set_pressed_no_signal(du_lieu_lt.cho_phep_cho);
                    }
                    if let Some(mut chk) = self.base().try_get_node_as::<CheckButton>("NutMenu/Menu/hien_ten_khac") {
                        chk.set_pressed_no_signal(du_lieu_lt.hien_thi_ten_khac);
                    }
                    if let Some(mut chk) = self.base().try_get_node_as::<CheckButton>("NutMenu/Menu/to_mau_phim") {
                        chk.set_pressed_no_signal(du_lieu_lt.to_mau_phim);
                    }
                    if let Some(mut chk) = self.base().try_get_node_as::<CheckButton>("NutMenu/Menu/bat_tat_nhac") {
                        chk.set_pressed_no_signal(du_lieu_lt.phat_nhac);
                    }
                    if let Some(mut chk) = self.base().try_get_node_as::<CheckButton>("NutMenu/Menu/bat_tat_loi") {
                        chk.set_pressed_no_signal(du_lieu_lt.phat_loi);
                    }

                    self.thiet_lap_am_thanh_bai_hat();
                    self.set_thong_tin_bai_hat();

                    if let Some(mut label) = self.ten_bai_hat_label.clone() {
                        label.set_text(&GString::from(du_lieu_lt.ten_bai_hoc));
                    }
                }
            } else {
                // Chế độ chơi bình thường: luôn bật nhạc và lời nếu file sheet có gắn
                self.cho_phep_phat_nhac = true;
                self.cho_phep_phat_loi = true;
                if let Some(mut quan_ly_sheet) = self.quan_ly_sheet_node.clone() {
                    quan_ly_sheet.bind_mut().khoi_tao_sheet(duong_dan, "".into(), "".into());
                }
                self.thiet_lap_am_thanh_bai_hat();
                self.set_thong_tin_bai_hat();
            }
        }

        if let Some(mut btn_thoat) = self.base().try_get_node_as::<Button>("thoat_choi_thu") {
            btn_thoat.set_visible(self.che_do_choi_thu);
            let callable_thoat = self.base().callable("thoat_choi_thu");
            if !btn_thoat.is_connected("button_down", &callable_thoat) {
                btn_thoat.connect("button_down", &callable_thoat);
            }
        }
    }

    fn process(&mut self, _delta: f64) {
        let Some(qls) = self.quan_ly_sheet_node.as_ref() else {
            return;
        };
        let (thoi_gian, da_dung) = {
            let b = qls.bind();
            (b.get_thoi_gian(), b.get_da_dung())
        };

        // 1. Nếu vẫn đang trong 10 giây đếm ngược đầu bài (thoi_gian < 0.0)
        if thoi_gian < 0.0 {
            if self.da_bat_dau_audio {
                self.da_bat_dau_audio = false;
                if let Some(mut np) = self.nhac_player.clone() {
                    np.stop();
                }
                if let Some(mut lp) = self.loi_player.clone() {
                    lp.stop();
                }
            }
            return;
        }

        // 2. Ngay khi chạm mốc 0.0 giây -> Bắt đầu phát từ đúng vị trí nhịp
        if !self.da_bat_dau_audio {
            self.da_bat_dau_audio = true;
            let vi_tri_bd = self.tinh_vi_tri_audio_hien_tai();
            if self.co_stream_nhac && self.cho_phep_phat_nhac {
                if let Some(mut np) = self.nhac_player.clone() {
                    np.play_ex().from_position(vi_tri_bd).done();
                }
            }
            if self.co_stream_loi && self.cho_phep_phat_loi {
                if let Some(mut lp) = self.loi_player.clone() {
                    lp.play_ex().from_position(vi_tri_bd).done();
                }
            }
        }

        // 3. Đồng bộ Tạm dừng / Tiếp tục khi bấm nút Menu hoặc khi dừng chờ phím
        if self.co_stream_nhac && self.cho_phep_phat_nhac {
            if let Some(mut np) = self.nhac_player.clone() {
                if np.get_stream_paused() != da_dung {
                    np.set_stream_paused(da_dung);
                }
            }
        }
        if self.co_stream_loi && self.cho_phep_phat_loi {
            if let Some(mut lp) = self.loi_player.clone() {
                if lp.get_stream_paused() != da_dung {
                    lp.set_stream_paused(da_dung);
                }
            }
        }
    }
}