use std::collections::HashSet;

// Khai báo pub struct để các file khác có thể truy cập được
pub struct HopAm;

impl HopAm {
    pub fn kiem_tra(danh_sach_phim: &HashSet<i32>) -> String {
        // Đổi điều kiện thành < 2 để bắt đầu hỗ trợ hợp âm Power Chord (chỉ cần 2 nốt)
        if danh_sach_phim.len() < 2 {
            return String::from("");
        }

        let ten_not: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        
        // 1. TÌM NỐT BASS (Nốt trầm nhất đang bấm trên bàn phím)
        let phim_bass = *danh_sach_phim.iter().min().unwrap();
        let not_bass = phim_bass % 12;

        // Đưa về nốt cơ bản (0-11) và loại bỏ trùng lặp
        let mut cac_not_co_ban: Vec<i32> = danh_sach_phim.iter().map(|&not| not % 12).collect();
        cac_not_co_ban.sort_unstable();
        cac_not_co_ban.dedup();

        // 2. KHAI BÁO THƯ VIỆN CÔNG THỨC BẰNG BITMASK (MẶT NẠ BIT)
        // Ví dụ: Major (0, 4, 7) = (1 dịch 0 bit) + (1 dịch 4 bit) + (1 dịch 7 bit)
        const MAJOR: u16 = (1 << 0) | (1 << 4) | (1 << 7);
        const MINOR: u16 = (1 << 0) | (1 << 3) | (1 << 7);
        const AUG: u16   = (1 << 0) | (1 << 4) | (1 << 8);
        const DIM: u16   = (1 << 0) | (1 << 3) | (1 << 6);
        const SUS4: u16  = (1 << 0) | (1 << 5) | (1 << 7);
        const SUS2: u16  = (1 << 0) | (1 << 2) | (1 << 7);
        
        // Hợp âm 4 nốt (Mở rộng từ hợp âm 3 nốt bằng phép OR)
        const DOM7: u16  = MAJOR | (1 << 10);
        const MAJ7: u16  = MAJOR | (1 << 11);
        const MIN7: u16  = MINOR | (1 << 10);
        const M7B5: u16  = DIM   | (1 << 10);
        
        // Hợp âm đặc biệt / Khuyết nốt
        const POWER5: u16   = (1 << 0) | (1 << 7);
        const DOM7_NO5: u16 = (1 << 0) | (1 << 4) | (1 << 10); // 7th bỏ nốt bậc 5

        // 3. THỬ TỪNG NỐT LÀM GỐC
        for &not_goc in &cac_not_co_ban {
            let mut bitmask: u16 = 0;
            
            for &not in &cac_not_co_ban {
                let khoang_cach: i32 = (not - not_goc + 12) % 12;
                // Bật bit thứ 'khoang_cach' lên 1
                bitmask |= 1 << khoang_cach;
            }

            // So khớp bitmask
            let loai_hop_am: &str = match bitmask {
                MAJOR => "",
                MINOR => "m",
                AUG => "aug",
                DIM => "dim",
                SUS4 => "sus4",
                SUS2 => "sus2",
                DOM7 => "7",
                MAJ7 => "maj7",
                MIN7 => "m7",
                M7B5 => "m7b5",
                POWER5 => "5",
                DOM7_NO5 => "7(no5)",
                _ => continue,
            };

            let ten_goc = ten_not[not_goc as usize];
            
            // 4. KIỂM TRA HỢP ÂM ĐẢO (SLASH CHORD)
            // Nếu nốt trầm nhất (Bass) không trùng với nốt gốc, đây là hợp âm đảo!
            if not_goc != not_bass {
                let ten_bass = ten_not[not_bass as usize];
                return format!("{}{}/{}", ten_goc, loai_hop_am, ten_bass); // Ví dụ: C/E
            } else {
                return format!("{}{}", ten_goc, loai_hop_am); // Ví dụ: C
            }
        }

        String::from("")
    }
}