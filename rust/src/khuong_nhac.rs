use godot::builtin::{ Color, GString, Vector2 };
use godot::classes::{ Area2D, Control, IControl, PackedScene };
use godot::global::godot_print;
use godot::obj::{ Gd, WithBaseField };
use godot::tools::try_load;
use godot::{ obj::Base, prelude::{ GodotClass, godot_api } };

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
    giu_lai_not: bool,

    // CÁC BIẾN PHỤC VỤ VẼ VẠCH NHỊP
    thoi_gian_hien_tai: f32,
    toc_do_chung: f32,
    pub nhip_moi_o: i32,
}

#[godot_api]
impl KhuongNhac {
    #[signal]
    fn khuong_da_het_not();

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
        self.cac_not_trong_khuong.len() as i32
    }

    pub fn tao_not(
        &mut self,
        id_not: i32,
        ten_not: &str,
        mau_not: &str,
        vi_tri_so: i32,
        nhip_dich: f32,
        nhip_giu: f32,
        toc_do_chung: f32
    ) {
        let Some(scene) = &self.nut_scene else {
            return;
        };
        let Some(mut not_nhac) = scene.try_instantiate_as::<NotNhac>() else {
            return;
        };

        let vt: Vector2 = match vi_tri_so {
            0 => Vector2::new(2040.0, -10.0),
            1 => Vector2::new(2040.0, 1.0),
            2 => Vector2::new(2040.0, 14.0),
            3 => Vector2::new(2040.0, 27.0),
            4 => Vector2::new(2040.0, 40.0),
            5 => Vector2::new(2040.0, 53.0),
            6 => Vector2::new(2040.0, 65.0),
            7 => Vector2::new(2040.0, 79.0),
            8 => Vector2::new(2040.0, 90.0),
            9 => Vector2::new(2040.0, 105.0),
            10 => Vector2::new(2040.0, 117.0),
            11 => Vector2::new(2040.0, 131.0),
            12 => Vector2::new(2040.0, 142.0),
            13 => Vector2::new(2040.0, 157.0),
            14 => Vector2::new(2040.0, 169.0),
            15 => Vector2::new(2040.0, 184.0),
            16 => Vector2::new(2040.0, 196.0),
            _ => {
                return;
            }
        };

        {
            let mut not_bind = not_nhac.bind_mut();
            not_bind.set_id_not(id_not);
            not_bind.set_ten_not(ten_not.into());
            not_bind.set_mau_not(mau_not.into());
            if nhip_giu > 0.0 {
                not_bind.set_chieu_dai_duoi(nhip_giu * toc_do_chung);
            }
        }

        if nhip_giu > 0.0 {
            if let Some(mut panel) = not_nhac.try_get_node_as::<godot::classes::Panel>("nen") {
                let mut size = panel.get_size();
                size.x += nhip_giu * toc_do_chung;
                panel.set_size(size);
            }
        }

        not_nhac.set_position(Vector2::new(2040.0, vt.y));
        self.base_mut().add_child(&not_nhac);
        self.cac_not_trong_khuong.push(DuLieuNot {
            nhip_dich,
            not_node: not_nhac,
        });
    }

    #[func]
    pub fn xu_ly_di_chuyen_not(&mut self, thoi_gian_hien_tai: f32, toc_do_chung: f32) {
        const TOA_DO_X_DICH: f32 = 130.0;

        // Lưu thông số và yêu cầu vẽ lại vạch nhịp mỗi frame
        self.thoi_gian_hien_tai = thoi_gian_hien_tai;
        self.toc_do_chung = toc_do_chung;
        self.base_mut().queue_redraw();

        let so_luong_truoc = self.cac_not_trong_khuong.len();

        self.cac_not_trong_khuong.retain_mut(|du_lieu| {
            let tg_dich = du_lieu.nhip_dich;
            let vi_tri_x = TOA_DO_X_DICH + (tg_dich - thoi_gian_hien_tai) * toc_do_chung;
            let toa_do_y = du_lieu.not_node.get_position().y;

            du_lieu.not_node.set_position(Vector2::new(vi_tri_x, toa_do_y));

            let chieu_dai = du_lieu.not_node.bind().get_chieu_dai_duoi();
            let toa_do_xoa = vi_tri_x + chieu_dai;

            if toa_do_xoa < -50.0 {
                du_lieu.not_node.queue_free();
                false
            } else {
                true
            }
        });

        if so_luong_truoc > 0 && self.cac_not_trong_khuong.is_empty() {
            godot_print!("Khuông nhạc đã hết nốt!");
            self.base_mut().emit_signal("khuong_da_het_not", &[]);
        }
    }

    #[func]
    pub fn kiem_tra_not_vao(&mut self, area: Gd<Area2D>) {
        let Some(not_nhac) = area.try_get_node_as::<NotNhac>(".") else {
            return;
        };
        self.cac_not_trong_area.push(not_nhac);
    }

    #[func]
    pub fn kiem_tra_not_ra(&mut self, area: Gd<Area2D>) {
        let Some(mut not_nhac) = area.try_get_node_as::<NotNhac>(".") else {
            return;
        };
        if !not_nhac.bind().get_da_duoc_danh() {
            not_nhac.bind_mut().da_danh_trat();
        }
        if !self.giu_lai_not {
            if let Some(index) = self.cac_not_trong_area.iter().position(|node| node == &not_nhac) {
                self.cac_not_trong_area.swap_remove(index);
            }
        }
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

        let toa_do_x_dich = 130.0;
        let nhip_min: f32 = self.thoi_gian_hien_tai + (-50.0 - toa_do_x_dich) / self.toc_do_chung;
        let nhip_max: f32 = self.thoi_gian_hien_tai + (2040.0 - toa_do_x_dich) / self.toc_do_chung;

        let o_bat_dau = (nhip_min / (self.nhip_moi_o as f32)).floor() as i32;
        let o_ket_thuc = (nhip_max / (self.nhip_moi_o as f32)).ceil() as i32;
        let o_bat_dau = o_bat_dau.max(0);

        for i in o_bat_dau..=o_ket_thuc {
            let nhip = (i * self.nhip_moi_o) as f32;
            let vi_tri_x = toa_do_x_dich + (nhip - self.thoi_gian_hien_tai) * self.toc_do_chung;

            let diem_bat_dau = Vector2::new(vi_tri_x, -2.0);
            let diem_ket_thuc = Vector2::new(vi_tri_x, 214.0); 
            let mau_sac = Color::from_rgba(0.7, 0.7, 0.7, 0.7);
            let do_day = 2.0;
            self.base_mut()
            .draw_line_ex(diem_bat_dau, diem_ket_thuc, mau_sac)
            .width(do_day).done();
        }
    }
}
