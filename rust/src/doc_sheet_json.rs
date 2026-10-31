use std::collections::BTreeMap;

use godot::{builtin::GString, classes::file_access::ModeFlags};
use godot::classes::FileAccess;
use godot::global::godot_print;
use serde::{Deserialize, Serialize};
use crate::check_ten_not::{hang_phim_trang_sang_id, id_sang_hang_phim_trang};

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
    #[serde(default)]
    pub dau_hoa: String,
    #[serde(default)]
    pub ten_khac: String,
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
/// Khoảng cách nhịp tối thiểu giữa 2 lần chuyển miền tự động để tránh nhảy miền liên tục
const KHOANG_NHIP_TOI_THIEU_GIUA_2_LAN_CHUYEN: f32 = 4.0;
/// Chỉ số hàng phím trắng tối đa để hàng trên cùng (hang + 16) không vượt quá phím 83 (B7)
const HANG_MIEN_TOI_DA: i32 = 32; // Hàng 32 là E5 -> Hàng 48 (32 + 16) là G7

pub fn la_phim_den(id_not: i32) -> bool {
    matches!(id_not.rem_euclid(12), 1 | 3 | 6 | 8 | 10)
}

pub fn dua_ve_phim_trang(id_not: i32) -> i32 {
    let hang = id_sang_hang_phim_trang(id_not.clamp(0, 83), false).clamp(0, HANG_MIEN_TOI_DA);
    hang_phim_trang_sang_id(hang)
}

/// Kiểm tra khoảng [min_id, max_id] có nằm gọn trong 17 hàng phím trắng (chênh lệch <= 16 hàng) không
fn co_mien_trang_hop_le(min_id: i32, max_id: i32) -> bool {
    let hang_min = id_sang_hang_phim_trang(min_id, false);
    let hang_max = id_sang_hang_phim_trang(max_id, false);
    hang_max - hang_min <= 16
}

/// Chọn miền bắt đầu (LUÔN LÀ PHÍM TRẮNG) cho cụm [cluster_min, cluster_max] trên khuông 17 phím trắng:
/// 1. Ưu tiên nhảy chẵn theo cụm quãng 8 (1c = 7 phím trắng, 2c = 14 phím trắng...) từ `mien_hien_tai`
/// 2. Nếu không nhảy chẵn cụm được thì chọn phím trắng giúp cụm nốt nằm chính giữa 17 hàng phím trắng.
fn chon_mien_trang_on_dinh(
    cluster_min: i32,
    cluster_max: i32,
    mien_hien_tai: i32,
) -> i32 {
    let hang_min = id_sang_hang_phim_trang(cluster_min, false);
    let hang_max = id_sang_hang_phim_trang(cluster_max, false);
    let hang_hien_tai = id_sang_hang_phim_trang(mien_hien_tai, false);

    // Hàng bắt đầu phải thỏa mãn: hang_mien <= hang_min và hang_mien + 16 >= hang_max
    let low_hang = (hang_max - 16).max(0);
    let high_hang = hang_min.min(HANG_MIEN_TOI_DA).max(low_hang);

    // Tâm lý tưởng trên 17 hàng phím trắng
    let hang_ly_tuong = (((hang_min + hang_max) - 16) / 2).clamp(low_hang, high_hang);

    // 1. Thử các bước nhảy chẵn cụm quãng 8: 1c (±7 hàng), 2c (±14 hàng), 3c (±21 hàng), 4c (±28 hàng)
    let mut best_chan_cum: Option<i32> = None;
    let mut min_lech_cum = i32::MAX;
    for so_cum in 1..=4 {
        for huong in [-1, 1] {
            let hang_ung_vien = (hang_hien_tai + huong * so_cum * 7).clamp(0, HANG_MIEN_TOI_DA);
            if hang_ung_vien >= low_hang && hang_ung_vien <= high_hang {
                let lech = (hang_ung_vien - hang_ly_tuong).abs();
                if lech < min_lech_cum {
                    min_lech_cum = lech;
                    best_chan_cum = Some(hang_ung_vien);
                }
            }
        }
    }
    if let Some(hang_cum) = best_chan_cum {
        return hang_phim_trang_sang_id(hang_cum);
    }

    // 2. Nếu không nhảy chẵn cụm được, lấy trực tiếp hàng phím trắng gần tâm nhất
    hang_phim_trang_sang_id(hang_ly_tuong)
}

/// Tìm cửa sổ 17 phím trắng (độ rộng <= 16 hàng phím trắng) chứa được nhiều nốt nhất trong 1 nhịp
fn tim_khoang_16_tot_nhat(ids: &[i32], mien_goi_y: i32) -> (i32, i32) {
    if ids.is_empty() {
        return (mien_goi_y, mien_goi_y);
    }
    let min_all = *ids.iter().min().unwrap();
    let max_all = *ids.iter().max().unwrap();

    // Nếu toàn bộ nốt trong nhịp nằm gọn trong 17 hàng phím trắng -> Lấy trọn bộ
    if co_mien_trang_hop_le(min_all, max_all) {
        return (min_all, max_all);
    }

    let hang_goi_y = id_sang_hang_phim_trang(mien_goi_y, false);
    let mut best_min = min_all;
    let mut best_max = max_all;
    let mut max_so_luong = 0;
    let mut min_khoang_cach_mien = i32::MAX;

    for &start_id in ids {
        let start_hang = id_sang_hang_phim_trang(start_id, false);
        let end_hang = start_hang + 16;
        let mut count = 0;
        let mut c_min = i32::MAX;
        let mut c_max = i32::MIN;

        for &id in ids {
            let h = id_sang_hang_phim_trang(id, false);
            if h >= start_hang && h <= end_hang {
                count += 1;
                c_min = c_min.min(id);
                c_max = c_max.max(id);
            }
        }

        let kc_mien = (start_hang - hang_goi_y).abs();
        if count > max_so_luong
            || (count == max_so_luong && (c_max > best_max || kc_mien < min_khoang_cach_mien))
        {
            max_so_luong = count;
            best_min = c_min;
            best_max = c_max;
            min_khoang_cach_mien = kc_mien;
        }
    }

    (best_min, best_max)
}

/// Kiểm tra nhanh khoảng [min_id, max_id] có nằm gọn trong khuông hiện tại (từ mien_hien_tai đến +16 phím trắng) không
fn nam_trong_khuong_hien_tai(min_id: i32, max_id: i32, mien_hien_tai: i32) -> bool {
    let hang_mien = id_sang_hang_phim_trang(mien_hien_tai, false);
    let hang_min = id_sang_hang_phim_trang(min_id, false);
    let hang_max = id_sang_hang_phim_trang(max_id, false);
    hang_min >= hang_mien && hang_max <= hang_mien + 16
}
/// Xác định nốt tại vị trí `i` trong danh sách dùng dấu Giáng (true) hay Thăng (false).
/// - Nếu `dau_hoa` được khóa thủ công ("giang" / "thang") -> Ưu tiên dùng theo lựa chọn.
/// - Nếu `dau_hoa` rỗng ("") -> Tự động phân tích theo hợp âm và hướng giai điệu.
pub fn xac_dinh_giang_tai_vi_tri(ds: &[NotNhacJson], i: usize) -> bool {
    let not = &ds[i];
    if not.id_not == 100 {
        return false;
    }
    match not.dau_hoa.as_str() {
        "giang" => true,
        "thang" => false,
        _ => {
            let mut id_truoc: Option<i32> = None;
            for k in (0..i).rev() {
                if ds[k].id_not != 100 {
                    id_truoc = Some(ds[k].id_not);
                    break;
                }
            }

            let mut id_sau: Option<i32> = None;
            for k in (i + 1)..ds.len() {
                if ds[k].id_not != 100 {
                    id_sau = Some(ds[k].id_not);
                    break;
                }
            }

            let cac_id_cung_nhip: Vec<i32> = ds
                .iter()
                .filter(|n| (n.nhip_dich - not.nhip_dich).abs() <= 0.01 && n.id_not != 100)
                .map(|n| n.id_not)
                .collect();

            crate::check_ten_not::tu_dong_xac_dinh_giang(
                not.id_not,
                id_truoc,
                id_sau,
                &cac_id_cung_nhip,
            )
        }
    }
}
pub fn tien_xu_ly_khuong(
    danh_sach_goc: Vec<NotNhacJson>,
    mien_goc: i32,
    dang_luyen_tap: bool,
    nhip_bat_dau: f32,
    nhip_ket_thuc: f32,
) -> (i32, Vec<NotNhacJson>) {
    let mut mien_hien_tai = dua_ve_phim_trang(mien_goc);
    let mut ds_loc: Vec<NotNhacJson> = Vec::new();

    // 1. Lọc nốt theo khoảng thời gian luyện tập VÀ bỏ các vạch ID 100 tự động cũ
    for mut not in danh_sach_goc {
        if !dang_luyen_tap && not.id_not == 100 && not.mau_not != "thu_cong" {
            continue;
        }

        if dang_luyen_tap {
            if not.nhip_dich < nhip_bat_dau {
                if not.id_not == 100 {
                    mien_hien_tai = dua_ve_phim_trang(mien_hien_tai + not.chuyen_not);
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

    // 2. Sắp xếp các nốt theo nhịp tăng dần
    ds_loc.sort_by(|a, b| {
        a.nhip_dich
            .partial_cmp(&b.nhip_dich)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.id_not.cmp(&a.id_not))
    });

    let mut ket_qua: Vec<NotNhacJson> = Vec::new();
    let mut mien_khoi_dau_moi = mien_hien_tai;
    let mut da_co_not_thuc_te = false;
    let mut i = 0;
    let mut nhip_vua_chuyen_thu_cong: f32 = -9999.0;
    let mut nhip_vua_chuyen_tu_dong: f32 = -9999.0;

    // 3. Duyệt theo từng bước nhịp và gom cụm tối đa 17 phím trắng (tương đương ~28 bán âm)
    while i < ds_loc.len() {
        let not = &ds_loc[i];

        if not.id_not == 100 && not.mau_not == "thu_cong" {
            let mien_dich = dua_ve_phim_trang(mien_hien_tai + not.chuyen_not);
            let chuyen_thuc_te = mien_dich - mien_hien_tai;
            mien_hien_tai = mien_dich;
            nhip_vua_chuyen_thu_cong = not.nhip_dich;
            nhip_vua_chuyen_tu_dong = not.nhip_dich;

            let mut not_thu_cong = not.clone();
            not_thu_cong.chuyen_not = chuyen_thuc_te;
            ket_qua.push(not_thu_cong);
            i += 1;
            continue;
        }

        let nhip_ht = not.nhip_dich;

        let mut cuoi_nhip_idx = i;
        let mut ids_trong_nhip: Vec<i32> = Vec::new();
        while cuoi_nhip_idx < ds_loc.len()
            && (ds_loc[cuoi_nhip_idx].nhip_dich - nhip_ht).abs() <= 0.01
        {
            if ds_loc[cuoi_nhip_idx].id_not != 100 {
                ids_trong_nhip.push(ds_loc[cuoi_nhip_idx].id_not);
            }
            cuoi_nhip_idx += 1;
        }

        if (nhip_ht - nhip_vua_chuyen_thu_cong).abs() <= 0.01 || ids_trong_nhip.is_empty() {
            for k in i..cuoi_nhip_idx {
                if ds_loc[k].id_not != 100 {
                    ket_qua.push(ds_loc[k].clone());
                    da_co_not_thuc_te = true;
                }
            }
            i = cuoi_nhip_idx;
            continue;
        }

        let (beat_min, beat_max) = tim_khoang_16_tot_nhat(&ids_trong_nhip, mien_hien_tai);

        // Kiểm tra nếu bước nhịp hiện tại vượt ra ngoài 17 hàng phím trắng của mien_hien_tai
        if !nam_trong_khuong_hien_tai(beat_min, beat_max, mien_hien_tai) {
            let mut cluster_min = beat_min;
            let mut cluster_max = beat_max;
            let mut nhip_cuoi_cum = nhip_ht;

            // Gom Lookahead tất cả các nhịp tiếp theo miễn là vẫn nằm gọn trong 17 phím trắng (<= 16 hàng)
            let mut m = cuoi_nhip_idx;
            while m < ds_loc.len() {
                if ds_loc[m].id_not == 100 {
                    break;
                }
                let nhip_m = ds_loc[m].nhip_dich;
                let mut cuoi_m = m;
                let mut ids_m: Vec<i32> = Vec::new();
                while cuoi_m < ds_loc.len() && (ds_loc[cuoi_m].nhip_dich - nhip_m).abs() <= 0.01 {
                    if ds_loc[cuoi_m].id_not != 100 {
                        ids_m.push(ds_loc[cuoi_m].id_not);
                    }
                    cuoi_m += 1;
                }

                if !ids_m.is_empty() {
                    let (next_min, next_max) = tim_khoang_16_tot_nhat(&ids_m, cluster_min);
                    let tam_min = cluster_min.min(next_min);
                    let tam_max = cluster_max.max(next_max);

                    if co_mien_trang_hop_le(tam_min, tam_max) {
                        cluster_min = tam_min;
                        cluster_max = tam_max;
                        nhip_cuoi_cum = nhip_m;
                    } else {
                        break;
                    }
                }
                m = cuoi_m;
            }

            // KIỂM TRA CHỐNG NHẢY MIỀN LIÊN TỤC:
            let mut la_not_nhay_ngan_quay_ve = false;
            if m < ds_loc.len() && (nhip_cuoi_cum - nhip_ht) < KHOANG_NHIP_TOI_THIEU_GIUA_2_LAN_CHUYEN {
                let nhip_sau = ds_loc[m].nhip_dich;
                if (nhip_sau - nhip_ht) < KHOANG_NHIP_TOI_THIEU_GIUA_2_LAN_CHUYEN {
                    let mut ids_sau: Vec<i32> = Vec::new();
                    let mut k_sau = m;
                    while k_sau < ds_loc.len()
                        && (ds_loc[k_sau].nhip_dich - nhip_sau).abs() <= 0.01
                    {
                        if ds_loc[k_sau].id_not != 100 {
                            ids_sau.push(ds_loc[k_sau].id_not);
                        }
                        k_sau += 1;
                    }
                    let (sau_min, sau_max) = tim_khoang_16_tot_nhat(&ids_sau, mien_hien_tai);
                    if nam_trong_khuong_hien_tai(sau_min, sau_max, mien_hien_tai) {
                        la_not_nhay_ngan_quay_ve = true;
                    }
                }
            }

            if !la_not_nhay_ngan_quay_ve {
                let mien_moi = chon_mien_trang_on_dinh(cluster_min, cluster_max, mien_hien_tai);
                let chuyen = mien_moi - mien_hien_tai;

                if chuyen != 0 {
                    if !da_co_not_thuc_te {
                        mien_khoi_dau_moi = mien_moi;
                        mien_hien_tai = mien_moi;
                        nhip_vua_chuyen_tu_dong = nhip_ht;
                    } else if (nhip_ht - nhip_vua_chuyen_tu_dong)
                        < KHOANG_NHIP_TOI_THIEU_GIUA_2_LAN_CHUYEN
                    {
                        if let Some(vach_cu) = ket_qua
                            .iter_mut()
                            .rev()
                            .find(|n| n.id_not == 100 && n.mau_not == "auto")
                        {
                            vach_cu.chuyen_not += chuyen;
                            mien_hien_tai = mien_moi;
                            if vach_cu.chuyen_not == 0 {
                                ket_qua.retain(|n| {
                                    !(n.id_not == 100 && n.mau_not == "auto" && n.chuyen_not == 0)
                                });
                            }
                        }
                    } else {
                        ket_qua.push(NotNhacJson {
                            id_not: 100,
                            mau_not: String::from("auto"),
                            nhip_dich: nhip_ht,
                            nhip_giu: 0.0,
                            chuyen_not: chuyen,
                            dau_hoa: String::new(),
                            ten_khac: String::new(),
                        });
                        mien_hien_tai = mien_moi;
                        nhip_vua_chuyen_tu_dong = nhip_ht;
                    }
                }
            }
        }

        for k in i..cuoi_nhip_idx {
            if ds_loc[k].id_not != 100 {
                ket_qua.push(ds_loc[k].clone());
                da_co_not_thuc_te = true;
            }
        }
        i = cuoi_nhip_idx;
    }

    (mien_khoi_dau_moi, ket_qua)
}
/// Chỉ sắp xếp danh sách nốt theo nhịp mà KHÔNG tự động sinh lại các vạch ID 100 đã bị xóa/sửa thủ công
pub fn sap_xep_khuong_giu_nguyen(
    mut danh_sach: Vec<NotNhacJson>,
    mien_goc: i32,
) -> (i32, Vec<NotNhacJson>) {
    danh_sach.sort_by(|a, b| {
        a.nhip_dich
            .partial_cmp(&b.nhip_dich)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.id_not.cmp(&a.id_not))
    });
    (mien_goc, danh_sach)
}

// =========================================================================
// 1. CẤU TRÚC JSON CHO BÀI TẬP LUYỆN SHEET (luyen_tap_2.json)
// =========================================================================
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LuyenTapJson {
    pub ten_bai_hoc: String,
    pub duong_dan_sheet: String,
    pub khuong: String,
    pub cho_phep_cho: bool,
    pub phan_thoi_gian: String,
    #[serde(default)]
    pub bpm_tuy_chinh: Option<f32>,
    #[serde(default)]
    pub cac_ghi_chu: BTreeMap<String, String>,
}

pub fn doc_file_luyen_tap(duong_dan: &str) -> Option<LuyenTapJson> {
    let file_text: GString = FileAccess::get_file_as_string(duong_dan);
    if file_text.to_string().is_empty() {
        godot_print!("Không thể đọc hoặc file trống: {}", duong_dan);
        return None;
    }
    match serde_json::from_str::<LuyenTapJson>(&file_text.to_string()) {
        Ok(bai_hoc) => Some(bai_hoc),
        Err(loi) => {
            godot_print!("Lỗi cú pháp JSON luyện tập: {}", loi);
            None
        }
    }
}

pub fn luu_file_luyen_tap(duong_dan: &str, data: &LuyenTapJson) -> bool {
    match serde_json::to_string_pretty(data) {
        Ok(json_str) => {
            if let Some(mut file) = FileAccess::open(duong_dan, ModeFlags::WRITE) {
                file.store_string(&json_str);
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

// =========================================================================
// 2. CẤU TRÚC JSON CHO BÀI HỌC VIDEO (xem_video.json)
// =========================================================================
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MocThoiGianJson {
    pub thoi_gian: i32,
    pub tieu_de: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BaiHocVideoJson {
    pub ten_bai_hoc: String,
    pub nguoi_tao: String,
    pub duong_dan_video: String,
    #[serde(default)]
    pub cac_moc_thoi_gian: Vec<MocThoiGianJson>,
}

pub fn doc_file_video_json(duong_dan: &str) -> Option<BaiHocVideoJson> {
    let txt = FileAccess::get_file_as_string(duong_dan).to_string();
    if txt.is_empty() {
        return None;
    }
    serde_json::from_str::<BaiHocVideoJson>(&txt).ok()
}

pub fn luu_file_video_json(duong_dan: &str, data: &BaiHocVideoJson) -> bool {
    if let Ok(json_str) = serde_json::to_string_pretty(data) {
        if let Some(mut file) = FileAccess::open(duong_dan, ModeFlags::WRITE) {
            file.store_string(&json_str);
            return true;
        }
    }
    false
}

// =========================================================================
// 3. CẤU TRÚC JSON CHO NGHE ĐOÁN NỐT & TÌM NỐT (nghe_va_doan.json)
// =========================================================================
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NgheTimNotJson {
    pub ten_bai_hoc: String,
    pub nguoi_tao: String,
    #[serde(default)]
    pub danh_sach_bai: Vec<String>,
    #[serde(default)]
    pub cac_ghi_chu: Vec<String>,
}

pub fn doc_file_nghe_tim_json(duong_dan: &str) -> Option<NgheTimNotJson> {
    let txt = FileAccess::get_file_as_string(duong_dan).to_string();
    if txt.is_empty() {
        return None;
    }
    serde_json::from_str::<NgheTimNotJson>(&txt).ok()
}

pub fn luu_file_nghe_tim_json(duong_dan: &str, data: &NgheTimNotJson) -> bool {
    if let Ok(json_str) = serde_json::to_string_pretty(data) {
        if let Some(mut file) = FileAccess::open(duong_dan, ModeFlags::WRITE) {
            file.store_string(&json_str);
            return true;
        }
    }
    false
}

// =========================================================================
// 4. CẤU TRÚC JSON CHO DANH SÁCH CHƯƠNG (chuong_1.json)
// =========================================================================
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MucBaiHocChuongJson {
    pub ten_bai_hoc: String,
    pub loai_bai_hoc: String,
    pub duong_dan: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChuongJson {
    pub ten_chuong: String,
    #[serde(default)]
    pub bai_hoc: Vec<MucBaiHocChuongJson>,
}

pub fn doc_file_chuong_json(duong_dan: &str) -> Option<ChuongJson> {
    let txt = FileAccess::get_file_as_string(duong_dan).to_string();
    if txt.is_empty() {
        return None;
    }
    serde_json::from_str::<ChuongJson>(&txt).ok()
}

pub fn luu_file_chuong_json(duong_dan: &str, data: &ChuongJson) -> bool {
    if let Ok(json_str) = serde_json::to_string_pretty(data) {
        if let Some(mut file) = FileAccess::open(duong_dan, ModeFlags::WRITE) {
            file.store_string(&json_str);
            return true;
        }
    }
    false
}


