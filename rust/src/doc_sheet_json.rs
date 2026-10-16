use godot::{builtin::GString, classes::file_access::ModeFlags};
use godot::classes::FileAccess;
use godot::global::godot_print;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaiHatJson {
    pub ten_bai_hat: String,
    pub tac_gia: String,
    pub nguoi_tao_sheet: String,
    pub loai_vach_nhip: String,
    pub bpm: f32,
    #[serde(default)]
    pub mien_not_bat_dau_k1: i32,
    #[serde(default)]
    pub mien_not_bat_dau_k2: i32,
    pub khuong_1: Vec<NotNhacJson>,
    pub khuong_2: Vec<NotNhacJson>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotNhacJson {
    pub id_not: i32,
    #[serde(default)]
    pub mau_not: String,
    pub nhip_dich: f32,
    #[serde(default)]
    pub nhip_giu: f32,
    #[serde(default)]
    pub chuyen_not: i32,
}

pub fn luu_file_json(duong_dan: &str, bai_hat: &BaiHatJson) -> bool {
    match serde_json::to_string_pretty(bai_hat) {
        Ok(noi_dung_json) => {
            if let Some(mut file) = FileAccess::open(duong_dan, ModeFlags::WRITE) {
                file.store_string(&noi_dung_json);
                godot_print!("Đã lưu sheet thành công tại: {}", duong_dan);
                true
            } else {
                godot_print!("Lỗi: Không thể mở file để ghi tại {}", duong_dan);
                false
            }
        }
        Err(loi) => {
            godot_print!("Lỗi đóng gói JSON: {}", loi);
            false
        }
    }
}

pub fn doc_file_json(duong_dan: &str) -> Option<BaiHatJson> {
    let file_text = FileAccess::get_file_as_string(duong_dan);
    if file_text.to_string().is_empty() { return None; }
    match serde_json::from_str::<BaiHatJson>(&file_text.to_string()) {
        Ok(bai_hat) => Some(bai_hat),
        Err(loi) => { godot_print!("Lỗi JSON: {}", loi); None }
    }
}
pub fn tien_xu_ly_khuong(
    danh_sach_goc: Vec<NotNhacJson>,
    mien_goc: i32,
    dang_luyen_tap: bool,
    nhip_bat_dau: f32,
    nhip_ket_thuc: f32,
) -> (i32, Vec<NotNhacJson>) {
    let mut mien_hien_tai = mien_goc;
    let mut ds_loc: Vec<NotNhacJson> = Vec::new();

    // 1. Lọc nốt theo khoảng thời gian luyện tập (nếu có)
    for mut not in danh_sach_goc {
        if dang_luyen_tap {
            if not.nhip_dich < nhip_bat_dau {
                if not.id_not == 100 {
                    mien_hien_tai += not.chuyen_not;
                }
                continue;
            }
            if not.nhip_dich > nhip_ket_thuc {
                continue;
            }
            not.nhip_dich -= nhip_bat_dau;
        }
        ds_loc.push(not);
    }

    // 2. Sắp xếp các nốt theo nhịp tăng dần (ưu tiên ID 100 đứng trước nếu cùng nhịp)
    ds_loc.sort_by(|a, b| {
        a.nhip_dich
            .partial_cmp(&b.nhip_dich)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.id_not.cmp(&a.id_not))
    });

    let mut ket_qua: Vec<NotNhacJson> = Vec::new();
    let mien_khoi_dau_moi = mien_hien_tai;

    // 3. Duyệt qua danh sách nốt và TỰ ĐỘNG THÊM ID 100 khi nốt vượt quá miền [mien_hien_tai, mien_hien_tai + 16]
    for i in 0..ds_loc.len() {
        let not = &ds_loc[i];

        // Nếu đã có sẵn nốt chuyển thủ công (ID 100)
        if not.id_not == 100 {
            mien_hien_tai += not.chuyen_not;
            ket_qua.push(not.clone());
            continue;
        }

        // Kiểm tra tại nhịp này có nốt nào cao hơn (mien_hien_tai + 16) hoặc thấp hơn mien_hien_tai không
        let nhip_ht = not.nhip_dich;
        let mut can_dieu_chinh = false;
        for k in i..ds_loc.len() {
            if (ds_loc[k].nhip_dich - nhip_ht).abs() > 0.001 {
                break;
            }
            if ds_loc[k].id_not != 100 {
                let id = ds_loc[k].id_not;
                if id < mien_hien_tai || id > mien_hien_tai + 16 {
                    can_dieu_chinh = true;
                    break;
                }
            }
        }

        if can_dieu_chinh {
            // Nhìn trước (Lookahead) để gom cụm nốt trong phạm vi 16 bậc
            let mut min_id = not.id_not;
            let mut max_id = not.id_not;

            for j in (i + 1)..ds_loc.len() {
                if ds_loc[j].id_not == 100 {
                    break;
                }
                let next_id = ds_loc[j].id_not;
                let tam_min = min_id.min(next_id);
                let tam_max = max_id.max(next_id);

                if tam_max - tam_min <= 16 {
                    min_id = tam_min;
                    max_id = tam_max;
                } else {
                    break;
                }
            }

            let low_bound = max_id - 16;
            let high_bound = min_id;
            let mien_moi = mien_hien_tai.clamp(low_bound, high_bound).clamp(0, 67);
            let chuyen = mien_moi - mien_hien_tai;

            if chuyen != 0 {
                // Luôn tự động chèn ký tự chuyển miền (ID 100) trước nốt vượt miền
                ket_qua.push(NotNhacJson {
                    id_not: 100,
                    mau_not: String::new(),
                    nhip_dich: not.nhip_dich,
                    nhip_giu: 0.0,
                    chuyen_not: chuyen,
                });
                mien_hien_tai = mien_moi;
            }
        }

        ket_qua.push(not.clone());
    }

    (mien_khoi_dau_moi, ket_qua)
}


#[derive(Deserialize, Debug)]
pub struct LuyenTapJson {
    pub ten_bai_hoc: String,
    pub duong_dan_sheet: String,
    pub khuong: String,
    pub cho_phep_cho: bool,
    pub phan_thoi_gian: String,
    // Note: cac_ghi_chu can be implemented later with a HashMap if needed
}

pub fn doc_file_luyen_tap(duong_dan: &str) -> Option<LuyenTapJson> {
    let file_text: GString = FileAccess::get_file_as_string(duong_dan);

    if file_text.to_string().is_empty() {
        godot_print!("Không thể đọc hoặc file trống {}", duong_dan);
        return None;
    }

    match serde_json::from_str::<LuyenTapJson>(&file_text.to_string()) {
        Ok(bai_hoc) => Some(bai_hoc),
        Err(loi) => {
            godot_print!("Lỗi cú pháp file JSON luyện tập: {}", loi);
            None
        }
    }
}