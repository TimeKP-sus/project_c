
const TBL_ID_NOT_VOI_THANG: [&str; 84] = [
    // Quãng 1 (ID 0 -> 11) - C1 là 0
    "C1", "C#1", "D1", "D#1", "E1", "F1", "F#1", "G1", "G#1", "A1", "A#1", "B1",
    // Quãng 2 (ID 12 -> 23)
    "C2", "C#2", "D2", "D#2", "E2", "F2", "F#2", "G2", "G#2", "A2", "A#2", "B2",
    // Quãng 3 (ID 24 -> 35)
    "C3", "C#3", "D3", "D#3", "E3", "F3", "F#3", "G3", "G#3", "A3", "A#3", "B3",
    // Quãng 4 (ID 36 -> 47) 
    "C4", "C#4", "D4", "D#4", "E4", "F4", "F#4", "G4", "G#4", "A4", "A#4", "B4",
    // Quãng 5 (ID 48 -> 59)
    "C5", "C#5", "D5", "D#5", "E5", "F5", "F#5", "G5", "G#5", "A5", "A#5", "B5",
    // Quãng 6 (ID 60 -> 71)
    "C6", "C#6", "D6", "D#6", "E6", "F6", "F#6", "G6", "G#6", "A6", "A#6", "B6",
    // Quãng 7 (ID 72 -> 83)
    "C7", "C#7", "D7", "D#7", "E7", "F7", "F#7", "G7", "G#7", "A7", "A#7", "B7",
];

// Lưu sẵn bảng 84 nốt nhạc (Dùng Giáng - Flat)
const TBL_ID_NOT_VOI_GIANG: [&str; 84] = [
    // Quãng 1 (ID 0 -> 11)
    "C1", "D♭1", "D1", "E♭1", "E1", "F1", "G♭1", "G1", "A♭1", "A1", "B♭1", "B1",
    // Quãng 2 (ID 12 -> 23)
    "C2", "D♭2", "D2", "E♭2", "E2", "F2", "G♭2", "G2", "A♭2", "A2", "B♭2", "B2",
    // Quãng 3 (ID 24 -> 35)
    "C3", "D♭3", "D3", "E♭3", "E3", "F3", "G♭3", "G3", "A♭3", "A3", "B♭3", "B3",
    // Quãng 4 (ID 36 -> 47)
    "C4", "D♭4", "D4", "E♭4", "E4", "F4", "G♭4", "G4", "A♭4", "A4", "B♭4", "B4",
    // Quãng 5 (ID 48 -> 59)
    "C5", "D♭5", "D5", "E♭5", "E5", "F5", "G♭5", "G5", "A♭5", "A5", "B♭5", "B5",
    // Quãng 6 (ID 60 -> 71)
    "C6", "D♭6", "D6", "E♭6", "E6", "F6", "G♭6", "G6", "A♭6", "A6", "B♭6", "B6",
    // Quãng 7 (ID 72 -> 83)
    "C7", "D♭7", "D7", "E♭7", "E7", "F7", "G♭7", "G7", "A♭7", "A7", "B♭7", "B7",
];

// Giữ nguyên 2 mảng TBL_ID_NOT_VOI_THANG và TBL_ID_NOT_VOI_GIANG của bạn...

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoaiDauHoa {
    BinhThuong, // Phím trắng (C, D, E, F, G, A, B)
    Thang,      // Nốt thăng (#) -> Nhích cao lên một chút
    Giang,      // Nốt giáng (♭ hoặc b) -> Nhích thấp xuống một chút
}

pub fn co_thang_giang(id: i32) -> bool {
    if id < 0 || id > 83 {
        return false;
    }
    matches!(id % 12, 1 | 3 | 6 | 8 | 10)
}

/// Kiểm tra một nốt (theo id và chế độ dung_giang) là Bình thường, Thăng hay Giáng
pub fn kiem_tra_thang_hay_giang(id: i32, dung_giang: bool) -> LoaiDauHoa {
    if !co_thang_giang(id) {
        LoaiDauHoa::BinhThuong
    } else if dung_giang {
        LoaiDauHoa::Giang
    } else {
        LoaiDauHoa::Thang
    }
}

/// Kiểm tra trực tiếp từ chuỗi tên nốt (VD: "C#4" -> Thang, "D♭4" / "Db4" -> Giang, "C4" -> BinhThuong)
pub fn kiem_tra_ten_not_thang_giang(ten_not: &str) -> LoaiDauHoa {
    if ten_not.contains('#') || ten_not.contains('♯') {
        LoaiDauHoa::Thang
    } else if ten_not.contains('♭') || (ten_not.len() >= 3 && ten_not.chars().nth(1) == Some('b')) {
        LoaiDauHoa::Giang
    } else {
        LoaiDauHoa::BinhThuong
    }
}


/// Trả về true nếu nên dùng dấu Giáng (♭), trả về false nếu nên dùng dấu Thăng (#)
pub fn tu_dong_xac_dinh_giang(
    id_hien_tai: i32,
    id_truoc: Option<i32>,
    id_sau: Option<i32>,
    cac_id_cung_nhip: &[i32],
) -> bool {
    // Phím trắng không có dấu hóa
    if !co_thang_giang(id_hien_tai) {
        return false;
    }

    let bac = id_hien_tai.rem_euclid(12);

    // =========================================================================
    // QUY LUẬT 1: XÉT THEO CẤU TẠO HỢP ÂM CÙNG NHỊP
    // =========================================================================
    if cac_id_cung_nhip.len() > 1 {
        // Dư 10: Phím giữa A và B
        if bac == 10 {
            // Đi cùng F (5) hoặc D (2) -> Hợp âm F / Dm / Bb -> Dùng B♭
            if cac_id_cung_nhip.iter().any(|&n| matches!(n.rem_euclid(12), 5 | 2)) {
                return true;
            }
            // Đi cùng F# (6) -> Hợp âm F# -> Dùng A#
            if cac_id_cung_nhip.iter().any(|&n| n.rem_euclid(12) == 6) {
                return false;
            }
        }

        // Dư 3: Phím giữa D và E
        if bac == 3 {
            // Đi cùng C (0) hoặc G (7) -> Hợp âm Cm / Eb -> Dùng E♭
            if cac_id_cung_nhip.iter().any(|&n| matches!(n.rem_euclid(12), 0 | 7)) {
                return true;
            }
            // Đi cùng B (11) -> Hợp âm B -> Dùng D#
            if cac_id_cung_nhip.iter().any(|&n| n.rem_euclid(12) == 11) {
                return false;
            }
        }

        // Dư 1: Phím giữa C và D
        if bac == 1 {
            // Đi cùng A (9) -> Hợp âm A trưởng (A - C# - E) -> Dùng C#
            if cac_id_cung_nhip.iter().any(|&n| n.rem_euclid(12) == 9) {
                return false;
            }
            // Đi cùng F (5) và Ab (8) -> Hợp âm Bm/Db -> Dùng D♭
            if cac_id_cung_nhip.iter().any(|&n| n.rem_euclid(12) == 8) {
                return true;
            }
        }

        // Dư 6: Phím giữa F và G
        if bac == 6 {
            // Đi cùng D (2) -> Hợp âm D trưởng (D - F# - A) -> Dùng F#
            if cac_id_cung_nhip.iter().any(|&n| n.rem_euclid(12) == 2) {
                return false;
            }
            // Đi cùng Db (1) hoặc Bb (10) -> Hợp âm Gb / Ebm -> Dùng G♭
            if cac_id_cung_nhip.iter().any(|&n| matches!(n.rem_euclid(12), 1 | 10)) {
                return true;
            }
        }
    }

    // =========================================================================
    // QUY LUẬT 2: XÉT THEO HƯỚNG CHUYỂN ĐỘNG GIAI ĐIỆU (MELODIC DIRECTION)
    // =========================================================================
    // Ưu tiên nốt liền sau (nốt mục tiêu mà ngón tay đang tiến tới)
    if let Some(sau) = id_sau {
        if sau < id_hien_tai {
            return true;  // Đang đi xuống -> Dùng Giáng (♭)
        } else if sau > id_hien_tai {
            return false; // Đang đi lên -> Dùng Thăng (#)
        }
    }

    // Nếu không có nốt sau, xét nốt liền trước
    if let Some(truoc) = id_truoc {
        if truoc > id_hien_tai {
            return true;  // Đi từ trên xuống -> Dùng Giáng (♭)
        } else if truoc < id_hien_tai {
            return false; // Đi từ dưới lên -> Dùng Thăng (#)
        }
    }

    // =========================================================================
    // QUY LUẬT 3: XU HƯỚNG TỰ NHIÊN CỦA 5 PHÍM ĐEN (FALLBACK)
    // =========================================================================
    match bac {
        10 => true,  // B♭
        3  => true,  // E♭
        6  => false, // F#
        1  => false, // C#
        8  => false, // G#
        _  => false,
    }
}

/// Quy đổi id_not sang chỉ số hàng phím trắng (0..=48):
/// - Nếu dung_giang = false (dùng Thăng #): C# nằm cùng hàng với C
/// - Nếu dung_giang = true (dùng Giáng ♭): D♭ nằm cùng hàng với D
pub fn id_sang_hang_phim_trang(id_not: i32, dung_giang: bool) -> i32 {
    let quang_8 = id_not.div_euclid(12);
    let du = id_not.rem_euclid(12);
    let bac_trang = if !dung_giang {
        match du {
            0 | 1 => 0,  // C, C# -> hàng C
            2 | 3 => 1,  // D, D# -> hàng D
            4 => 2,      // E     -> hàng E
            5 | 6 => 3,  // F, F# -> hàng F
            7 | 8 => 4,  // G, G# -> hàng G
            9 | 10 => 5, // A, A# -> hàng A
            11 => 6,     // B     -> hàng B
            _ => 0,
        }
    } else {
        match du {
            0 => 0,      // C      -> hàng C
            1 | 2 => 1,  // D♭, D  -> hàng D
            3 | 4 => 2,  // E♭, E  -> hàng E
            5 => 3,      // F      -> hàng F
            6 | 7 => 4,  // G♭, G  -> hàng G
            8 | 9 => 5,  // A♭, A  -> hàng A
            10 | 11 => 6,// B♭, B  -> hàng B
            _ => 0,
        }
    };
    quang_8 * 7 + bac_trang
}



/// Quy đổi ngược từ chỉ số hàng phím trắng về id_not của phím trắng đó
pub fn hang_phim_trang_sang_id(hang_trang: i32) -> i32 {
    let quang_8 = hang_trang.div_euclid(7);
    let ban_am = match hang_trang.rem_euclid(7) {
        0 => 0,  // C
        1 => 2,  // D
        2 => 4,  // E
        3 => 5,  // F
        4 => 7,  // G
        5 => 9,  // A
        6 => 11, // B
        _ => 0,
    };
    (quang_8 * 12 + ban_am).clamp(0, 83)
}

/// Trích xuất tên nốt trực tiếp từ index
pub fn chuyen_id_thanh_ten(id: i32, dung_giang: bool) -> &'static str {
    // ID khớp chính xác với index (0 -> 83)
    if id >= 0 && id <= 83 {
        let index = id as usize;
        
        if dung_giang {
            TBL_ID_NOT_VOI_GIANG[index]
        } else {
            TBL_ID_NOT_VOI_THANG[index]
        }
    } else {
        ""
    }
}