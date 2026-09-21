
use serde::{Deserialize, Serialize};
use std::collections::HashMap; 

// Struct chứa thông tin riêng của mỗi bài hát
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThongTinBaiHat {
    pub diem_cao_nhat: i32,
    pub so_lan_choi: i32,
    pub da_hoan_thanh: bool, // Tùy chọn: Đánh dấu bài đã qua bàn
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuLieuNguoiDung {
    pub ten_nguoi_choi: String,
    pub do_tre_phim: f32,
    pub lich_su_bai_hat: HashMap<String, ThongTinBaiHat>, 
}

impl DuLieuNguoiDung {
    pub fn khoi_tao(ten_nguoi_choi: String) -> Self {
        Self {
            ten_nguoi_choi,
            do_tre_phim: 0.0,
            lich_su_bai_hat: HashMap::new(),
        }
    }
    pub fn set_bai_hat(&mut self, ten_bai_hat: String, diem: i32, da_hoan_thanh: bool) {
        let thong_tin: &mut ThongTinBaiHat = self.lich_su_bai_hat.entry(ten_bai_hat.clone()).or_insert(ThongTinBaiHat {
            diem_cao_nhat: diem,
            so_lan_choi: 0,
            da_hoan_thanh,
        });

        // Cập nhật điểm cao nhất nếu điểm mới cao hơn
        if diem > thong_tin.diem_cao_nhat {
            thong_tin.diem_cao_nhat = diem;
        }

        // Tăng số lần chơi
        thong_tin.so_lan_choi += 1;

        // Cập nhật trạng thái hoàn thành nếu cần
        if da_hoan_thanh {
            thong_tin.da_hoan_thanh = true;
        }
    }
    pub fn get_bai_hat_theo_ten(&self, ten_bai_hat: &str) -> Option<&ThongTinBaiHat> {
        self.lich_su_bai_hat.get(ten_bai_hat)
    }
    pub fn set_do_tre_phim(&mut self, do_tre: f32) {
        self.do_tre_phim = do_tre;
    }
    /// Lấy độ trễ của người chơi
    pub fn get_do_tre_phim(&self) -> f32 {
        self.do_tre_phim
    }
}