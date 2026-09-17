
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

pub fn co_thang_giang(id: i32) -> bool {
    // Nếu ID nằm ngoài khoảng mảng, bỏ qua
    if id < 0 || id > 83 {
        return false;
    }
    // Nốt C luôn có dư là 0. Các phím đen nằm ở dư: 1, 3, 6, 8, 10
    match id % 12 {
        1 | 3 | 6 | 8 | 10 => true, 
        _ => false,
    }
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