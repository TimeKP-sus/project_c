use godot::classes::{Control, IControl, Label};
use godot::obj::{Gd, WithBaseField};
use godot::{
    obj::Base,
    prelude::{GodotClass, godot_api},
};

use crate::check_ten_not::chuyen_id_thanh_ten;
use crate::khuong_nhac::KhuongNhac;
use crate::not_nhac::NotNhac;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct QuanLySheet {
    #[base]
    base: Base<Control>,
    thoi_gian: f32,
    #[export]
    da_dung: bool,
    #[export]
    toc_do: f32,
    #[export]
    bpm: f32,

    #[export]
    thoi_gian_label: Option<Gd<Label>>,

    khuong_1: Option<Gd<KhuongNhac>>,
    khuong_2: Option<Gd<KhuongNhac>>,
    thoi_gian_cho: f32,
    // sheet_nhac: Json
}

#[godot_api]
impl QuanLySheet {

    // #[func]
    // pub fn nhan_tin_hieu_bam_phim(&mut self, id_not: i32) {
    //     if let Some(khuong) = self.khuong_1.as_mut() {
    //         khuong.bind_mut().xu_ly_bam_phim(id_not);
    //     }
    //     if let Some(khuong) = self.khuong_2.as_mut() {
    //         khuong.bind_mut().xu_ly_bam_phim(id_not);
    //     }
    // }

    // #[func]
    // pub fn nhan_tin_hieu_nha_phim(&mut self, id_not: i32) {
    //     // Truyền lệnh nhả phím xuống cho cả 2 khuông nhạc
    //     if let Some(khuong) = self.khuong_1.as_mut() {
    //         khuong.bind_mut().xu_ly_nha_phim(id_not);
    //     }
    //     if let Some(khuong) = self.khuong_2.as_mut() {
    //         khuong.bind_mut().xu_ly_nha_phim(id_not);
    //     }
    // }


    #[func]
    pub fn get_thoi_gian(&self) -> f32 {
        self.thoi_gian
    }
    #[func]
    pub fn get_danh_sach_not_trong_area(&mut self) -> Vec<Gd<NotNhac>> {
        let mut danh_sach_phim_trong_area: Vec<Gd<NotNhac>> = Vec::new();
        if let Some(khuong) = self.khuong_1.as_mut() {
            let ds_phim_khuong_1 = khuong.bind_mut().get_cac_not_trong_area();
            danh_sach_phim_trong_area.extend(ds_phim_khuong_1);
        }
        if let Some(khuong) = self.khuong_2.as_mut() {
            let ds_phim_khuong_2 = khuong.bind_mut().get_cac_not_trong_area();
            danh_sach_phim_trong_area.extend(ds_phim_khuong_2);
        }
        // godot::global::godot_print!(
        //     "Danh sách nốt trong vùng: {:?}",
        //     danh_sach_phim_trong_area
        //         .iter()
        //         .map(|not| not.bind().get_id_not())
        //         .collect::<Vec<i32>>()
        // );
        danh_sach_phim_trong_area
    }
}

#[godot_api]
impl IControl for QuanLySheet {
    fn ready(&mut self) {
        self.khuong_1 = self.base().try_get_node_as::<KhuongNhac>("K1");
        self.khuong_2 = self.base().try_get_node_as::<KhuongNhac>("K2");
        // cai dat
        self.thoi_gian_cho = 10.0;
        self.thoi_gian = 0.0 - self.thoi_gian_cho;

        let khoang_cach_nhip: f32 = self.toc_do; // Lấy biến toc_do đã export ở QuanLySheet
        // ♭
        // Tay phải (Khuông 1): Giai điệu chính
        self.khuong_1.as_mut().map(|khuong| {
            khuong.bind_mut().tao_nhieu_not(
                vec![
                    // Câu 1: "Twinkle twinkle little star"
                    // (id, tên nốt, màu, vị trí Y, nhịp đích, nhịp giữ)
                    (36, chuyen_id_thanh_ten(36, false), "1", 15, 0.0, 1.0), // C4
                    (36, chuyen_id_thanh_ten(36, false), "1", 15, 1.0, 1.0), // C4
                    (43, chuyen_id_thanh_ten(43, false), "3", 11, 2.0, 1.0), // G4
                    (43, chuyen_id_thanh_ten(43, false), "3", 11, 3.0, 1.0), // G4
                    (45, chuyen_id_thanh_ten(45, false), "5", 10, 4.0, 1.0), // A4
                    (45, chuyen_id_thanh_ten(45, false), "5", 10, 5.0, 1.0), // A4
                    (43, chuyen_id_thanh_ten(43, false), "4", 11, 6.0, 2.0), // G4 (Ngân dài 2 phách)
                    // Câu 2: "How I wonder what you are"
                    (41, chuyen_id_thanh_ten(41, false), "2", 12, 8.0, 1.0), // F4
                    (41, chuyen_id_thanh_ten(41, false), "2", 12, 9.0, 1.0), // F4
                    (40, chuyen_id_thanh_ten(40, false), "4", 13, 10.0, 1.0), // E4
                    (40, chuyen_id_thanh_ten(40, false), "4", 13, 11.0, 1.0), // E4
                    (38, chuyen_id_thanh_ten(38, false), "1", 14, 12.0, 1.0), // D4
                    (38, chuyen_id_thanh_ten(38, false), "1", 14, 13.0, 1.0), // D4
                    (36, chuyen_id_thanh_ten(36, false), "3", 15, 14.0, 2.0), // C4 (Ngân dài 2 phách)
                ],
                khoang_cach_nhip,
            )
        });

        // Tay trái (Khuông 2): Đệm Bass đơn giản
        self.khuong_2.as_mut().map(|khuong| {
            khuong.bind_mut().tao_nhieu_not(
                vec![
                    // Tay trái sẽ đánh các nốt trầm, mỗi nốt ngân dài hẳn 4 phách (1 ô nhịp)
                    (24, chuyen_id_thanh_ten(24, false), "3", 8, 0.0, 4.0), // C3
                    (29, chuyen_id_thanh_ten(29, false), "2", 5, 4.0, 4.0), // F3
                    (24, chuyen_id_thanh_ten(24, false), "3", 8, 8.0, 4.0), // C3
                    (19, chuyen_id_thanh_ten(19, false), "1", 12, 12.0, 4.0), // G2
                ],
                khoang_cach_nhip,
            )
        });
    }
    fn process(&mut self, delta: f64) {
        if self.da_dung {
            return;
        }
        self.thoi_gian += delta as f32;
        // if let Some(label) = self.thoi_gian_label.as_mut() {
        //     label.set_text(&format!("{:.2}", self.thoi_gian).into());
        // }
        self.thoi_gian_label.as_mut().map(|label| {
            label.set_text(&format!("{:.2}", self.thoi_gian + self.thoi_gian_cho));
        });
        let khoang_cach_nhip: f32 = self.toc_do;
        let nhip_hien_tai: f32 = (self.thoi_gian * self.bpm) / 60.0;
        if let Some(khuong) = self.khuong_1.as_mut() {
            khuong
                .bind_mut()
                .xu_ly_di_chuyen_not(nhip_hien_tai, khoang_cach_nhip);
        }

        if let Some(khuong) = self.khuong_2.as_mut() {
            khuong
                .bind_mut()
                .xu_ly_di_chuyen_not(nhip_hien_tai, khoang_cach_nhip);
        }
    }
}
