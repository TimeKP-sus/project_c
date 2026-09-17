use godot::builtin::Vector2;

use godot::classes::{Area2D, Control, IControl, PackedScene};

use godot::global::godot_print;
use godot::obj::{Gd, WithBaseField};

use godot::tools::try_load;

use godot::{
    obj::Base,
    prelude::{GodotClass, godot_api},
};

use crate::not_nhac::NotNhac;

use crate::quan_ly_sheet::QuanLySheet;
pub struct DuLieuNot {
    pub nhip_dich: f32,
    pub not_node: Gd<NotNhac>,
}

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct KhuongNhac {
    #[base]
    base: Base<Control>,
    nut_scene: Option<Gd<PackedScene>>,
    area_nhan_not: Option<Gd<Area2D>>,
    quan_ly_sheet: Option<Gd<QuanLySheet>>,
    //Vec
    cac_not_trong_area: Vec<Gd<NotNhac>>,
    cac_not_trong_khuong: Vec<DuLieuNot>,
}

#[godot_api]

impl KhuongNhac {
    // pub fn get_cac_not_trong_area(&self) -> HashSet<i32> {
    //     self.cac_not_trong_area
    //         .iter()
    //         .map(|not| not.bind().get_id_not())
    //         .collect()
    // }
    pub fn get_cac_not_trong_area(&mut self) -> Vec<Gd<NotNhac>> {
        self.cac_not_trong_area
            .retain(|not| not.is_instance_valid());

        self.cac_not_trong_area.clone()
    }
    pub fn tao_not(
        &mut self,
        id_not: i32,
        ten_not: &str,
        // mau_not: &str,
        mau_not: &str,
        vi_tri_so: i32,
        nhip_dich: f32,
        nhip_giu: f32,
        toc_do_chung: f32,
    ) {
        let Some(scene) = &self.nut_scene else {
            return;
        };

        let Some(mut not_nhac) = scene.try_instantiate_as::<NotNhac>() else {
            return;
        };

        let vt: Vector2 = match vi_tri_so {
            // Mỗi vị trí cách nhau chính xác 14 pixel
            0 => Vector2::new(2040.0, -10.0),
            1 => Vector2::new(2040.0, 1.0),    // +14
            2 => Vector2::new(2040.0, 14.0),    // +14
            3 => Vector2::new(2040.0, 27.0),   // +14
            4 => Vector2::new(2040.0, 40.0),   // +14
            5 => Vector2::new(2040.0, 53.0),   // +14
            6 => Vector2::new(2040.0, 65.0),   // +14 (Dòng kẻ trên cùng)
            7 => Vector2::new(2040.0, 79.0),   // +14
            8 => Vector2::new(2040.0, 90.0),   // +14
            9 => Vector2::new(2040.0, 105.0),   // +14
            10 => Vector2::new(2040.0, 117.0),  // +14
            11 => Vector2::new(2040.0, 131.0), // +14
            12 => Vector2::new(2040.0, 142.0), // +14 (Dòng kẻ thứ hai)
            13 => Vector2::new(2040.0, 157.0), // +14
            14 => Vector2::new(2040.0, 169.0),
            15 => Vector2::new(2040.0, 184.0),
            16 => Vector2::new(2040.0, 196.0), 
            _ => return,
        };

        {
            let mut not_bind = not_nhac.bind_mut();
            not_bind.set_id_not(id_not);
            not_bind.set_ten_not(ten_not.into());
            not_bind.set_mau_not(mau_not.into());
            not_bind.set_chieu_dai_duoi(nhip_giu * toc_do_chung);
        }
        if nhip_giu > 0.0 {
            if let Some(mut panel) = not_nhac.try_get_node_as::<godot::classes::Panel>("nen") {
                let mut size: Vector2 = panel.get_size();
                size.x += nhip_giu * toc_do_chung;
                panel.set_size(size);
            }
        }

        not_nhac.set_position(Vector2::new(2040.0, vt.y));
        self.base_mut().add_child(&not_nhac);
        self.cac_not_trong_khuong.push(DuLieuNot {
            nhip_dich: nhip_dich,
            not_node: not_nhac,
        });
    }
    /// id_not, ten_not, vi_tri_so, thoi_gian_dich, thoi_gian_giu
    pub fn tao_nhieu_not(
        &mut self,
        danh_sach_not: Vec<(i32, &str, &str, i32, f32, f32)>,
        khoang_cach_nhip: f32, // toc do
    ) {
        for (id_not, ten_not, mau_not, vi_tri_so, thoi_gian_dich, thoi_gian_giu) in danh_sach_not {
            self.tao_not(
                id_not,
                ten_not,
                mau_not,
                vi_tri_so,
                thoi_gian_dich,
                thoi_gian_giu,
                khoang_cach_nhip,
            );
        }
    }

    #[func]
    pub fn xu_ly_di_chuyen_not(&mut self, thoi_gian_hien_tai: f32, toc_do_chung: f32) {
        const TOA_DO_X_DICH: f32 = 115.0;
        self.cac_not_trong_khuong.retain_mut(|du_lieu| {
            let tg_dich = du_lieu.nhip_dich;
            let vi_tri_x: f32 = TOA_DO_X_DICH + ((tg_dich - thoi_gian_hien_tai) * toc_do_chung);
            let toa_do_y: f32 = du_lieu.not_node.get_position().y;

            du_lieu
                .not_node
                .set_position(Vector2::new(vi_tri_x, toa_do_y));

            // nếu không nốt Hold dài sẽ biến mất khi đang lướt qua vạch
            let chieu_dai = du_lieu.not_node.bind().get_chieu_dai_duoi();
            let toa_do_xoa = vi_tri_x + chieu_dai;

            if toa_do_xoa < -50.0 {
                du_lieu.not_node.queue_free();
                false
            } else {
                true
            }
        });
    }

    #[func]
    pub fn kiem_tra_not_vao(&mut self, area: Gd<Area2D>) {
        let Some(not_nhac) = area.try_get_node_as::<NotNhac>(".") else {
            godot_print!("khong tim thay area");
            return;
        };

        // not_nhac.bind_mut().bat_dau_giu();
        self.cac_not_trong_area.push(not_nhac);
    }

    #[func]
    pub fn kiem_tra_not_ra(&mut self, area: Gd<Area2D>) {
        let Some(not_nhac) = area.try_get_node_as::<NotNhac>(".") else {
            return;
        };
        // Kiểm tra Miss trước khi xóa khỏi mảng
        if !not_nhac.bind().get_da_duoc_danh() {
            godot_print!("Người chơi đã bỏ lỡ nốt.");
            // Gắn logic trừ máu hoặc reset combo ở đây
        }

        if let Some(index) = self
            .cac_not_trong_area
            .iter()
            .position(|node| node == &not_nhac)
        {
            self.cac_not_trong_area.swap_remove(index);
        }
    }
    //test
    
}

#[godot_api]
impl IControl for KhuongNhac {
    fn ready(&mut self) {
        self.nut_scene = try_load::<PackedScene>("res://scene/not.tscn").ok();

        self.quan_ly_sheet = self.base().try_get_node_as::<QuanLySheet>("..");
        self.area_nhan_not = self.base().try_get_node_as::<Area2D>("NhanDien");
    }
}
