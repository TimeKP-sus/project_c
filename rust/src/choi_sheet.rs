use godot::builtin::{ Callable, Color, GString, NodePath, Variant };
use godot::classes::{ Control, IControl, Label, Object };

use godot::obj::{ Gd, GdRef, WithBaseField };
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
    #[export]
    ten_bai_hat_label: Option<Gd<Label>>,
    #[export]
    tac_gia_label: Option<Gd<Label>>,
    #[export]
    nguoi_tao_label: Option<Gd<Label>>,

    #[export]
    phan_luyen_tap: bool,

    #[export]
    duong_dan_bai_hoc: GString,

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
            // let tac_gia = sheet_bind.get_tac_gia();
            // let nguoi_tao_sheet = sheet_bind.get_nguoi_tao_sheet();

            if let Some(mut label) = self.ten_bai_hat_label.clone() {
                label.set_text(&ten_bai_hat);
                self.hieu_ung_xuat_hien_label(&mut label);
            }
        }
    }
    pub fn hieu_ung_xuat_hien_label(&mut self, label: &mut Gd<Label>) {
        // 1. Bật hiển thị và đặt độ trong suốt (alpha) ban đầu về 0.0
        label.set_visible(true);
        let mut mau_goc: Color = label.get_modulate();
        mau_goc.a = 0.0;
        label.set_modulate(mau_goc);
        let mut mau_hien: Color = mau_goc;
        mau_hien.a = 1.0;

        let mut mau_an: Color = mau_goc;
        mau_an.a = 0.0;

        // Ép kiểu sang Object để truyền vào Tween
        let doi_tuong: Gd<Object> = label.clone().upcast::<Object>();

        // 2. Tạo chuỗi hiệu ứng chạy nối tiếp nhau
        if let Some(mut tween) = self.base_mut().create_tween() {
            // Bước 1: Hiện rõ dần lên trong 0.3 giây
            tween.tween_property(
                &doi_tuong,
                &NodePath::from("modulate"),
                &Variant::from(mau_hien),
                0.3
            );

            // Bước 2: Đứng yên hiển thị trong đúng 4 giây
            tween.tween_interval(4.0);

            // Bước 3: Mờ dần và biến mất trong 0.5 giây
            tween.tween_property(
                &doi_tuong,
                &NodePath::from("modulate"),
                &Variant::from(mau_an),
                0.5
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
        let cac_not_trong_area: Vec<Gd<NotNhac>> = self.quan_ly_sheet_node
            .as_mut()
            .unwrap()
            .bind_mut()
            .get_danh_sach_not_trong_area();

        for mut not_node in cac_not_trong_area {
            let id_not: i32 = not_node.bind().get_id_not();
            let da_danh: bool = not_node.bind().get_da_duoc_danh();

            if id_not == so_phim && !da_danh {
                not_node.bind_mut().bat_dau_giu();

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
        let tat_ca_not = self.quan_ly_sheet_node
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
        // godot_print!("Bỏ lỡ nốt! Tổng trượt: {}", self.so_phim_truot);
        self.cat_nhat_diem_label(self.diem);
    }
    #[func]
    pub fn reset_diem(&mut self) {
        self.diem = 0;
        self.so_phim_tot = 0;
        self.so_phim_kha = 0;
        self.so_phim_truot = 0;
        self.cat_nhat_diem_label(self.diem);
    }
    pub fn lay_ten_che_do_luu(&self) -> &'static str {
        if self.phan_luyen_tap {
            "luyen_tap_sheet"
        } else {
            "choi_sheet"
        }
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
        ((da_danh_trung as f32 / mau_so as f32) * 100.0).clamp(0.0, 100.0)
    }

    #[func]
    pub fn lay_diem_cao_nhat_cu(&self) -> i32 {
        self.diem_ky_luc
    }

    #[func]
    pub fn tong_ket(&mut self) {
        if self.duong_dan_bai_hoc.is_empty() {
            return;
        }
        let ti_le = self.lay_ti_le_hoan_thanh();
        let hang = DuLieuLichSu::tinh_hang(ti_le);
        let che_do = self.lay_ten_che_do_luu();

        let mut ls: DuLieuLichSu = DuLieuLichSu::doc_lich_su();

        // 1. Lưu lại điểm kỷ lục cũ TRƯỚC KHI cập nhật kết quả mới
        self.diem_ky_luc = ls
            .lay_ket_qua(che_do, &self.duong_dan_bai_hoc.to_string())
            .map(|kq| kq.diem_cao_nhat)
            .unwrap_or(0);

        // 2. Cập nhật kết quả mới vào file lich_su.json
        ls.cap_nhat_ket_qua(
            che_do,
            &self.duong_dan_bai_hoc.to_string(),
            self.diem,
            ti_le,
            hang,
        );
    }
}

#[godot_api]
impl IControl for ChoiSheet {
    fn ready(&mut self) {
        self.ban_phim_midi_node = self.base().try_get_node_as::<BanPhimMidi>("MIDI/BanPhimPiano");
        self.quan_ly_sheet_node = self.base().try_get_node_as::<QuanLySheet>("QuanLySheet");

        self.quan_ly_sheet_node.as_mut().unwrap().bind_mut().bat_tat_cho_phep_xoa_not(true);
        let check_phim_midi: Callable = self.base().callable("kiem_tra_phim");
        let nha_phim_midi: Callable = self.base().callable("xu_ly_nha_phim");

        if let Some(mut midi_node) = self.ban_phim_midi_node.clone() {
            midi_node.connect("phim_vua_duoc_bam", &check_phim_midi);
            midi_node.connect("phim_vua_duoc_nha", &nha_phim_midi);
        }

        let callable_truot = self.base().callable("xu_ly_not_bi_truot");

        // Trỏ thẳng đường dẫn xuyên qua QuanLySheet để lấy K1 và K2
        let khuong_1 = self.base().try_get_node_as::<KhuongNhac>("QuanLySheet/K1");
        let khuong_2 = self.base().try_get_node_as::<KhuongNhac>("QuanLySheet/K2");

        if let Some(mut k1) = khuong_1 {
            k1.connect("not_bi_truot", &callable_truot);
        }
        if let Some(mut k2) = khuong_2 {
            k2.connect("not_bi_truot", &callable_truot);
        }

        let duong_dan: GString = self.duong_dan_bai_hoc.clone();
        if !duong_dan.is_empty() {
            if self.phan_luyen_tap {
                if self.quan_ly_sheet_node.as_mut().unwrap().bind().get_cho_phep_cho() == true {
                    self.quan_ly_sheet_node
                        .as_mut()
                        .unwrap()
                        .bind_mut()
                        .bat_tat_am_thanh_nhip(false);
                }

                // Đọc file luyen_tap.json
                if
                    let Some(du_lieu_lt) = doc_sheet_json::doc_file_luyen_tap(
                        &duong_dan.to_string()
                    )
                {
                    if let Some(mut quan_ly_sheet) = self.quan_ly_sheet_node.clone() {
                        quan_ly_sheet.bind_mut().set_cho_phep_cho(du_lieu_lt.cho_phep_cho);

                        quan_ly_sheet
                            .bind_mut()
                            .khoi_tao_sheet(
                                du_lieu_lt.duong_dan_sheet.into(),
                                du_lieu_lt.khuong.into(),
                                du_lieu_lt.phan_thoi_gian.into()
                            );
                            if let Some(bpm_moi) = du_lieu_lt.bpm_tuy_chinh {
                            if bpm_moi > 0.0 {
                                // Ghi đè biến bpm nội bộ của QuanLySheet
                                quan_ly_sheet.bind_mut().bpm = bpm_moi;
                            }
                        }
                    }

                    // 1. Lấy thông tin Tác giả, Người tạo sheet từ file test.json gốc
                    self.set_thong_tin_bai_hat();

                    // 2. GHI ĐÈ tên bài hát trên UI bằng tên bài học luyện tập
                    if let Some(mut label) = self.ten_bai_hat_label.clone() {
                        label.set_text(&GString::from(du_lieu_lt.ten_bai_hoc));
                    }
                }
            } else {
                // Chế độ chơi bình thường
                if let Some(mut quan_ly_sheet) = self.quan_ly_sheet_node.clone() {
                    quan_ly_sheet.bind_mut().khoi_tao_sheet(duong_dan, "".into(), "".into());
                }
                // Lấy trọn bộ thông tin từ file test.json gốc
                self.set_thong_tin_bai_hat();
            }
        }

    }
}
