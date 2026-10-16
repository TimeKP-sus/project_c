use godot::builtin::{ Color, GString, Vector2 };
use godot::classes::{ Area2D, Control, IControl, PackedScene, Panel };
use godot::global::godot_print;
use godot::meta::ToGodot;
use godot::obj::{ Gd, GdMut, WithBaseField };
use godot::tools::try_load;
use godot::{ obj::Base, prelude::{ GodotClass, godot_api } };

use crate::check_ten_not;
use crate::not_nhac::NotNhac;
use crate::quan_ly_sheet::QuanLySheet;

pub struct DuLieuNot {
    pub nhip_dich: f32,
    pub not_node: Gd<NotNhac>,
}

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct KhuongNhac {
    #[base]
    base: Base<Control>,
    nut_scene: Option<Gd<PackedScene>>,
    area_nhan_not: Option<Gd<Area2D>>,
    quan_ly_sheet: Option<Gd<QuanLySheet>>,

    cac_not_trong_area: Vec<Gd<NotNhac>>,
    cac_not_trong_khuong: Vec<DuLieuNot>,

    #[export]
    pub cho_phep_xoa_not: bool,

    #[export]
    giu_lai_not: bool,

    thoi_gian_hien_tai: f32,
    toc_do_chung: f32,
    pub nhip_moi_o: i32,

    pub mien_goc: i32,
    pub mien_hien_tai: i32,
    cac_moc_chuyen_mien: Vec<(f32, i32)>,
}

#[godot_api]
impl KhuongNhac {
    #[signal]
    fn khuong_da_het_not();

    #[signal]
    fn not_bi_truot();

    #[signal]
    fn cap_nhat_mien_not(gia_tri_chuyen: i32);

    #[func]
    pub fn da_het_not(&self) -> bool {
        self.cac_not_trong_khuong.is_empty()
    }

    pub fn get_cac_not_trong_area(&mut self) -> Vec<Gd<NotNhac>> {
        self.cac_not_trong_area.retain(|not| not.is_instance_valid());
        self.cac_not_trong_area.clone()
    }

    pub fn get_tat_ca_not(&mut self) -> Vec<Gd<NotNhac>> {
        self.cac_not_trong_khuong
            .iter()
            .map(|du_lieu| du_lieu.not_node.clone())
            .collect()
    }

    #[func]
    pub fn get_tong_so_not(&self) -> i32 {
        self.cac_not_trong_khuong
            .iter()
            .filter(|du_lieu| du_lieu.not_node.bind().get_id_not() != 100)
            .count() as i32
    }

    fn phan_tich_not(id_not: i32, mien_bat_dau: i32, chuyen_not: i32) -> (String, f32) {
        if id_not == 100 {
            let ten: String = if chuyen_not > 0 {
                format!("{}⬈", chuyen_not.abs())
            } else if chuyen_not < 0 {
                format!("{}⬊", chuyen_not.abs())
            } else {
                String::from("")
            };
            return (ten, -6.0);
        }

        // 2. Xử lý nốt bình thường
        let ten_not = check_ten_not::chuyen_id_thanh_ten(id_not, false).to_string();

        // 3. Tính toán vị trí và ÉP CHẶT vào khoảng 0 -> 16
        let vi_tri_so = (16 - (id_not - mien_bat_dau)).clamp(0, 16);

        let toa_do_y = match vi_tri_so {
            0 => -10.0,
            1 => 1.0,
            2 => 14.0,
            3 => 27.0,
            4 => 40.0,
            5 => 53.0,
            6 => 65.0,
            7 => 79.0,
            8 => 90.0,
            9 => 105.0,
            10 => 117.0,
            11 => 131.0,
            12 => 142.0,
            13 => 157.0,
            14 => 169.0,
            15 => 184.0,
            16 => 196.0,
            _ => 210.0,
        };

        (ten_not, toa_do_y)
    }
    #[func]
    pub fn tao_not(
        &mut self,
        id_not: i32,
        mau_not: String,
        nhip_dich: f32,
        nhip_giu: f32,
        chuyen_not: i32,
        mien_bat_dau: i32,
        toc_do_chung: f32
    ) {
        let Some(scene) = &self.nut_scene else {
            return;
        };
        let Some(mut not_nhac) = scene.try_instantiate_as::<NotNhac>() else {
            return;
        };

        let (ten_not, toa_do_y) = Self::phan_tich_not(id_not, mien_bat_dau, chuyen_not);
        let vt: Vector2 = Vector2::new(2040.0, toa_do_y);

        {
            let mut not_bind: GdMut<'_, NotNhac> = not_nhac.bind_mut();
            not_bind.set_id_not(id_not);
            not_bind.set_ten_not(ten_not.into());

            // XỬ LÝ NỐT CHUYỂN
            if id_not == 100 {
                not_bind.bien_thanh_vach_canh_bao();
            } else {
                not_bind.set_mau_not(&mau_not);
                if nhip_giu > 0.0 {
                    not_bind.set_chieu_dai_duoi(nhip_giu * toc_do_chung);
                }
            }
        }
        // Thêm điều kiện id_not != 100 để vạch cảnh báo KHÔNG BỊ kéo dài đuôi nhip_giu
        if nhip_giu > 0.0 && id_not != 100 {
            if let Some(mut panel) = not_nhac.try_get_node_as::<Panel>("nen") {
                let mut size: Vector2 = panel.get_size();
                size.x += nhip_giu * toc_do_chung;
                panel.set_size(size);
            }
        }

        not_nhac.set_position(vt);
        self.base_mut().add_child(&not_nhac);
        self.cac_not_trong_khuong.push(DuLieuNot { nhip_dich, not_node: not_nhac });
    }

    #[func]
    pub fn xu_ly_di_chuyen_not(&mut self, thoi_gian_hien_tai: f32, toc_do_chung: f32) {
        const TOA_DO_X_DICH: f32 = 130.0;

        self.thoi_gian_hien_tai = thoi_gian_hien_tai;
        self.toc_do_chung = toc_do_chung;
        self.base_mut().queue_redraw();

        let so_luong_truoc = self.cac_not_trong_khuong.len();
        let cho_phep_xoa = self.cho_phep_xoa_not;

        self.cac_not_trong_khuong.retain_mut(|du_lieu| {
            let tg_dich: f32 = du_lieu.nhip_dich;
            let vi_tri_x: f32 = TOA_DO_X_DICH + (tg_dich - thoi_gian_hien_tai) * toc_do_chung;
            let toa_do_y: f32 = du_lieu.not_node.get_position().y;

            du_lieu.not_node.set_position(Vector2::new(vi_tri_x.round(), toa_do_y));

            // CHỈ XÓA NỐT KHI CHẾ ĐỘ XÓA ĐANG BẬT
            if cho_phep_xoa {
                let chieu_dai: f32 = du_lieu.not_node.bind().get_chieu_dai_duoi();
                let toa_do_xoa: f32 = vi_tri_x + chieu_dai;

                if toa_do_xoa < -50.0 {
                    du_lieu.not_node.queue_free();
                    return false;
                }
            }
            true
        });

        if cho_phep_xoa && so_luong_truoc > 0 && self.cac_not_trong_khuong.is_empty() {
            godot_print!("Khuông nhạc đã hết nốt!");
            self.base_mut().emit_signal("khuong_da_het_not", &[]);
        }
    }

    #[func]
    pub fn kiem_tra_not_vao(&mut self, area: Gd<Area2D>) {
        let Some(not_nhac) = area.try_get_node_as::<NotNhac>(".") else {
            return;
        };

        // KIỂM TRA NẾU LÀ NỐT CHUYỂN (ID 100)
        let id: i32 = not_nhac.bind().get_id_not();
        if id == 100 && self.cho_phep_xoa_not {
            // Lấy tên nốt (VD: "5⬈" hoặc "3⬊")
            let ten = not_nhac.bind().get_ten_not().to_string();

            // Lọc ra các ký tự số từ chuỗi
            let so_str: String = ten
                .chars()
                .filter(|c| c.is_digit(10))
                .collect();
            if let Ok(gia_tri_tuyet_doi) = so_str.parse::<i32>() {
                // Xác định dấu (lên hay xuống)
                let gia_tri_chuyen: i32 = if ten.contains('⬈') {
                    gia_tri_tuyet_doi
                } else if ten.contains('⬊') {
                    -gia_tri_tuyet_doi
                } else {
                    0
                };

                // Phát tín hiệu lên QuanLySheet
                self.base_mut().emit_signal("cap_nhat_mien_not", &[gia_tri_chuyen.to_variant()]);
            }
        }

        self.cac_not_trong_area.push(not_nhac);
    }

    #[func]
    pub fn kiem_tra_not_ra(&mut self, area: Gd<Area2D>) {
        let Some(mut not_nhac) = area.try_get_node_as::<NotNhac>(".") else {
            return;
        };
        if self.cho_phep_xoa_not && !not_nhac.bind().get_da_duoc_danh() {
            not_nhac.bind_mut().da_danh_trat();
            self.base_mut().emit_signal("not_bi_truot", &[]);
        }
        if !self.giu_lai_not {
            if let Some(index) = self.cac_not_trong_area.iter().position(|node| node == &not_nhac) {
                self.cac_not_trong_area.swap_remove(index);
            }
        }
    }
    // 1. Xóa sạch các node nốt nhạc cũ trên khuông trước khi vẽ lại
    #[func]
    pub fn xoa_tat_ca_not(&mut self) {
        for mut du_lieu in self.cac_not_trong_khuong.drain(..) {
            if du_lieu.not_node.is_instance_valid() {
                du_lieu.not_node.queue_free();
            }
        }
        self.cac_not_trong_area.clear();
        self.base_mut().queue_redraw();
    }

    // 2. Di chuyển nốt khi tua thanh trượt trong TaoSheet (KHÔNG xóa nốt khi đi qua vạch đích)
    #[func]
    pub fn cap_nhat_vi_tri_soan(&mut self, nhip_hien_tai: f32, toc_do_chung: f32) {
        const TOA_DO_X_DICH: f32 = 130.0;
        self.thoi_gian_hien_tai = nhip_hien_tai;
        self.toc_do_chung = toc_do_chung;
        self.base_mut().queue_redraw();

        for du_lieu in &mut self.cac_not_trong_khuong {
            let vi_tri_x = TOA_DO_X_DICH + (du_lieu.nhip_dich - nhip_hien_tai) * toc_do_chung;
            let toa_do_y = du_lieu.not_node.get_position().y;
            du_lieu.not_node.set_position(Vector2::new(vi_tri_x.round(), toa_do_y));
        }
    }
    #[func]
    pub fn het_not_trong_khuong(&mut self) {
        self.base_mut().emit_signal("khuong_da_het_not", &[]);
    }
    ///!chua on
    #[func]
    pub fn get_khoang_ten_not_hien_tai(&self) -> GString {
        // Nếu khuông nhạc không có nốt nào
        if self.cac_not_trong_khuong.is_empty() {
            return "Trống".into();
        }

        // Lấy nốt đầu tiên
        let not_dau: &DuLieuNot = self.cac_not_trong_khuong.first().unwrap();
        let ten_dau: GString = not_dau.not_node.bind().get_ten_not();

        // Lấy nốt cuối cùng
        let not_cuoi: &DuLieuNot = self.cac_not_trong_khuong.last().unwrap();
        let ten_cuoi: GString = not_cuoi.not_node.bind().get_ten_not();

        format!("{} -> {}", ten_dau, ten_cuoi).into()
    }
}

#[godot_api]
impl IControl for KhuongNhac {
    fn ready(&mut self) {
        self.nut_scene = try_load::<PackedScene>("res://scene/not.tscn").ok();
        self.quan_ly_sheet = self.base().try_get_node_as::<QuanLySheet>("..");
        self.area_nhan_not = self.base().try_get_node_as::<Area2D>("NhanDien");
    }

    fn draw(&mut self) {
        if self.toc_do_chung <= 0.0 || self.nhip_moi_o <= 0 {
            return;
        }
        let toa_do_x_dich: f32 = 130.0;
        let nhip_min: f32 = self.thoi_gian_hien_tai + (-50.0 - toa_do_x_dich) / self.toc_do_chung;
        let nhip_max: f32 = self.thoi_gian_hien_tai + (2040.0 - toa_do_x_dich) / self.toc_do_chung;

        let o_bat_dau = (nhip_min / (self.nhip_moi_o as f32)).floor() as i32;
        let o_ket_thuc = (nhip_max / (self.nhip_moi_o as f32)).ceil() as i32;
        let o_bat_dau = o_bat_dau.max(0);

        for i in o_bat_dau..=o_ket_thuc {
            let nhip = (i * self.nhip_moi_o) as f32;
            let vi_tri_x = toa_do_x_dich + (nhip - self.thoi_gian_hien_tai) * self.toc_do_chung;

            let diem_bat_dau: Vector2 = Vector2::new(vi_tri_x, 0.0);
            let diem_ket_thuc: Vector2 = Vector2::new(vi_tri_x, 213.0);
            let mau_sac: Color = Color::from_rgba(0.7, 0.7, 0.7, 0.7);
            let do_day: f32 = 2.0;
            self.base_mut().draw_line_ex(diem_bat_dau, diem_ket_thuc, mau_sac).width(do_day).done();
        }
    }
}
