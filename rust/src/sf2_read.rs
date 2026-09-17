use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex, OnceLock},
    thread,
    time::Duration,
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use godot::global::godot_print;
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};

// Thời gian trễ (milli-giây) trước khi thực sự tắt một nốt nhạc
const DO_TRE_NHA_PHIM_MS: u64 = 180;

// Cấu trúc chứa bộ tổng hợp âm thanh (Synthesizer) và luồng âm thanh đang chạy
struct BoDocSf2 {
    bo_tong_hop: Arc<Mutex<Synthesizer>>,
    _luong_am_thanh: cpal::Stream, // Giữ biến này lại để stream âm thanh không bị hủy (drop)
}

impl BoDocSf2 {
    // Hàm khởi tạo bộ xử lý từ một đường dẫn tệp SF2
    fn moi(duong_dan_sf2: &str) -> Result<Self, String> {
        // Mở tệp SoundFont
        let tep_tin = File::open(duong_dan_sf2)
            .map_err(|e| format!("Khong mo duoc SF2 tai '{}': {}", duong_dan_sf2, e))?;
        let mut bo_doc = BufReader::new(tep_tin);
        
        // Nạp dữ liệu SoundFont từ bộ đọc
        let phong_am_thanh = Arc::new(
            SoundFont::new(&mut bo_doc)
                .map_err(|e| format!("Doc SF2 that bai '{}': {:?}", duong_dan_sf2, e))?,
        );

        // Cài đặt bộ tổng hợp với tần số lấy mẫu là 44100Hz
        let cai_dat = SynthesizerSettings::new(44_100);
        let bo_tong_hop = Synthesizer::new(&phong_am_thanh, &cai_dat)
            .map_err(|e| format!("Khoi tao synthesizer that bai: {:?}", e))?;

        // Đưa bộ tổng hợp vào Arc và Mutex để có thể chia sẻ an toàn giữa các luồng (thread)
        let bo_tong_hop_an_toan = Arc::new(Mutex::new(bo_tong_hop));
        
        // Tạo và khởi động luồng phát âm thanh liên tục
        let luong_am_thanh = tao_luong_dau_ra(Arc::clone(&bo_tong_hop_an_toan))?;
        luong_am_thanh
            .play()
            .map_err(|e| format!("Khong the chay audio stream: {}", e))?;

        Ok(Self {
            bo_tong_hop: bo_tong_hop_an_toan,
            _luong_am_thanh: luong_am_thanh,
        })
    }
}

// Các biến tĩnh toàn cục (Singleton) lưu trữ trạng thái của engine và các nốt nhạc
static BODOC: OnceLock<Mutex<Option<BoDocSf2>>> = OnceLock::new();
static THE_HE_NOT_NHAC: OnceLock<Mutex<HashMap<i32, u64>>> = OnceLock::new();

// Lấy tham chiếu đến biến toàn cục của engine
fn khe_cam_bo_doc() -> &'static Mutex<Option<BoDocSf2>> {
    BODOC.get_or_init(|| Mutex::new(None))
}

// Lấy tham chiếu đến bộ đếm thế hệ của các nốt nhạc (dùng để chặn việc tắt nhầm nốt khi nốt đó vừa được bấm lại nhanh)
fn danh_sach_the_he_not() -> &'static Mutex<HashMap<i32, u64>> {
    THE_HE_NOT_NHAC.get_or_init(|| Mutex::new(HashMap::new()))
}

// Tăng giá trị thế hệ của một nốt nhạc cụ thể mỗi khi nó được tác động
fn tang_the_he_not(not_nhac: i32) -> u64 {
    let khe_chua = danh_sach_the_he_not();
    let mut chot_khoa = match khe_chua.lock() {
        Ok(khoa) => khoa,
        Err(_) => return 0,
    };

    // Tăng giá trị lên 1, an toàn chống tràn số (saturating_add)
    let muc_du_lieu: &mut u64 = chot_khoa.entry(not_nhac).or_insert(0);
    *muc_du_lieu = muc_du_lieu.saturating_add(1);
    *muc_du_lieu
}

// Lấy thế hệ hiện tại của một nốt nhạc
fn lay_the_he_not_hien_tai(not_nhac: i32) -> Option<u64> {
    let khe_chua = danh_sach_the_he_not();
    let chot_khoa = khe_chua.lock().ok()?;
    chot_khoa.get(&not_nhac).copied()
}

// Chạy một luồng nền để chờ một khoảng thời gian ngắn (Delay) rồi mới thực sự tắt nốt
fn hen_gio_tat_not(not_nhac: i32, the_he_ky_vong: u64) {
    thread::spawn(move || {
        // Đợi theo hằng số đã định sẵn
        thread::sleep(Duration::from_millis(DO_TRE_NHA_PHIM_MS));

        // Nếu thế hệ nốt nhạc đã thay đổi (nốt này vừa được bấm lại), thì không tắt nữa
        if lay_the_he_not_hien_tai(not_nhac) != Some(the_he_ky_vong) {
            return;
        }

        // Tắt nốt nhạc trên bộ tổng hợp
        if let Some(bo_tong_hop) = lay_bo_tong_hop() {
            if let Ok(mut bth) = bo_tong_hop.lock() {
                let phim_dan = not_nhac.clamp(0, 127);
                bth.note_off(0, phim_dan);
            }
        }
    });
}

// API CHÍNH: Khởi tạo hệ thống âm thanh (cần gọi một lần duy nhất)
pub fn khoi_tao_bo_doc(duong_dan_tuyet_doi_sf2: &str) -> Result<(), String> {
    let khe_chua = khe_cam_bo_doc();
    let mut chot_khoa: std::sync::MutexGuard<'_, Option<BoDocSf2>> = khe_chua
        .lock()
        .map_err(|_| String::from("Khong the khoa bo nho engine"))?;

    // Nếu engine đã được khởi tạo trước đó thì bỏ qua
    if chot_khoa.is_some() {
        return Ok(());
    }

    let bo_doc = BoDocSf2::moi(duong_dan_tuyet_doi_sf2)?;
    *chot_khoa = Some(bo_doc);
    godot_print!("SF2 da nap xong: {}", duong_dan_tuyet_doi_sf2);
    Ok(())
}

// API: Thay đổi file SF2 đang sử dụng tại runtime
pub fn thay_doi_sf2(duong_dan_moi: &str) -> Result<(), String> {
    // Tạo bộ đọc mới trước để đảm bảo không mất âm thanh nếu nạp thất bại
    let bo_doc_moi = BoDocSf2::moi(duong_dan_moi)?;

    let khe_chua = khe_cam_bo_doc();
    let mut chot_khoa = khe_chua
        .lock()
        .map_err(|_| String::from("Khong the khoa bo nho engine"))?;

    // Thay thế engine cũ (nếu có) bằng engine mới; engine cũ sẽ bị drop và stream dừng
    *chot_khoa = Some(bo_doc_moi);
    godot_print!("SF2 da duoc doi sang: {}", duong_dan_moi);
    Ok(())
}

// API CHÍNH: Phát âm thanh một nốt nhạc
pub fn bat_not_nhac(not_nhac: i32, luc_nhan: i32) {
    tang_the_he_not(not_nhac);

    if let Some(bo_tong_hop) = lay_bo_tong_hop() {
        if let Ok(mut bth) = bo_tong_hop.lock() {
            // Giới hạn giá trị chuẩn của chuẩn MIDI (0 - 127)
            let phim_dan = not_nhac.clamp(0, 127);
            let luc = luc_nhan.clamp(0, 127);
            bth.note_on(0, phim_dan, luc);
        }
    }
}

// API CHÍNH: Dừng âm thanh của một nốt nhạc
pub fn tat_not_nhac(not_nhac: i32) {
    let the_he = tang_the_he_not(not_nhac);
    hen_gio_tat_not(not_nhac, the_he);
}

// API CHÍNH: Xử lý thay đổi control (ví dụ: đạp pedal ngân âm, thay đổi âm lượng)
pub fn thay_doi_dieu_khien(bo_dieu_khien: i32, gia_tri: i32) {
    if let Some(bo_tong_hop) = lay_bo_tong_hop() {
        if let Ok(mut bth) = bo_tong_hop.lock() {
            let cc = bo_dieu_khien.clamp(0, 127);
            let gt = gia_tri.clamp(0, 127);
            // 0xB0 là mã Control Change trên kênh 0 theo chuẩn MIDI
            bth.process_midi_message(0, 0xB0, cc, gt);
        }
    }
}

// Hàm hỗ trợ nội bộ để lấy ra tham chiếu của bộ tổng hợp đang chạy
fn lay_bo_tong_hop() -> Option<Arc<Mutex<Synthesizer>>> {
    let khe_chua = khe_cam_bo_doc();
    let chot_khoa = khe_chua.lock().ok()?;
    chot_khoa.as_ref().map(|bo_doc| Arc::clone(&bo_doc.bo_tong_hop))
}

// Hàm khởi tạo và kết nối bộ tổng hợp với thiết bị phát âm thanh của hệ điều hành
fn tao_luong_dau_ra(bo_tong_hop: Arc<Mutex<Synthesizer>>) -> Result<cpal::Stream, String> {
    let may_chu_audio = cpal::default_host();
    let thiet_bi = may_chu_audio
        .default_output_device()
        .ok_or_else(|| String::from("Khong tim thay thiet bi audio output mac dinh"))?;

    let cau_hinh = thiet_bi
        .default_output_config()
        .map_err(|e| format!("Khong lay duoc cau hinh audio output: {}", e))?;

    let cau_hinh_luong = cau_hinh.config();
    let so_kenh = cau_hinh_luong.channels as usize;

    let ham_bao_loi = |loi| {
        godot_print!("Loi stream audio: {}", loi);
    };

    // Dựa vào định dạng mẫu (Sample Format) của thiết bị để tạo luồng xử lý phù hợp
    match cau_hinh.sample_format() {
        cpal::SampleFormat::F32 => {
            let bth_f32 = Arc::clone(&bo_tong_hop);
            thiet_bi
            .build_output_stream(
                cau_hinh_luong.clone(),
                move |du_lieu: &mut [f32], _| ghi_du_lieu_f32(du_lieu, so_kenh, &bth_f32),
                ham_bao_loi,
                None,
            )
            .map_err(|e| format!("Khong tao duoc stream f32: {}", e))
        }
        cpal::SampleFormat::I16 => {
            let bth_i16 = Arc::clone(&bo_tong_hop);
            thiet_bi
            .build_output_stream(
                cau_hinh_luong.clone(),
                move |du_lieu: &mut [i16], _| ghi_du_lieu_i16(du_lieu, so_kenh, &bth_i16),
                ham_bao_loi,
                None,
            )
            .map_err(|e| format!("Khong tao duoc stream i16: {}", e))
        }
        cpal::SampleFormat::U16 => {
            let bth_u16 = Arc::clone(&bo_tong_hop);
            thiet_bi
            .build_output_stream(
                cau_hinh_luong,
                move |du_lieu: &mut [u16], _| ghi_du_lieu_u16(du_lieu, so_kenh, &bth_u16),
                ham_bao_loi,
                None,
            )
            .map_err(|e| format!("Khong tao duoc stream u16: {}", e))
        }
        _loi_dinh_dang => Err(format!("Dinh dang sample khong ho tro: {:?}", _loi_dinh_dang)),
    }
}

// Hàm render: Xử lý và trích xuất một khối bộ đệm âm thanh kênh trái và kênh phải
fn xuat_khoi_am_thanh(bo_tong_hop: &Arc<Mutex<Synthesizer>>, so_mau: usize) -> (Vec<f32>, Vec<f32>) {
    let mut kenh_trai = vec![0.0_f32; so_mau];
    let mut kenh_phai = vec![0.0_f32; so_mau];

    if let Ok(mut bth) = bo_tong_hop.lock() {
        bth.render(&mut kenh_trai, &mut kenh_phai);
    }

    (kenh_trai, kenh_phai)
}

// Hàm chuyển đổi và ghi dữ liệu âm thanh dưới dạng Float 32-bit cho CPAL Stream
fn ghi_du_lieu_f32(dau_ra: &mut [f32], so_kenh: usize, bo_tong_hop: &Arc<Mutex<Synthesizer>>) {
    let so_mau = dau_ra.len() / so_kenh;
    let (kenh_trai, kenh_phai) = xuat_khoi_am_thanh(bo_tong_hop, so_mau);

    for mau in 0..so_mau {
        let trai: f32 = kenh_trai[mau].clamp(-1.0, 1.0);
        let phai: f32 = kenh_phai[mau].clamp(-1.0, 1.0);

        for kenh in 0..so_kenh {
            dau_ra[mau * so_kenh + kenh] = if kenh % 2 == 0 { trai } else { phai };
        }
    }
}

// Hàm chuyển đổi và ghi dữ liệu âm thanh dưới dạng Signed Integer 16-bit
fn ghi_du_lieu_i16(dau_ra: &mut [i16], so_kenh: usize, bo_tong_hop: &Arc<Mutex<Synthesizer>>) {
    let so_mau = dau_ra.len() / so_kenh;
    let (kenh_trai, kenh_phai) = xuat_khoi_am_thanh(bo_tong_hop, so_mau);

    for mau in 0..so_mau {
        // Ánh xạ dải tín hiệu -1.0 đến 1.0 sang giá trị số nguyên i16
        let trai: i16 = (kenh_trai[mau].clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        let phai: i16 = (kenh_phai[mau].clamp(-1.0, 1.0) * i16::MAX as f32) as i16;

        for kenh in 0..so_kenh {
            dau_ra[mau * so_kenh + kenh] = if kenh % 2 == 0 { trai } else { phai };
        }
    }
}

// Hàm chuyển đổi và ghi dữ liệu âm thanh dưới dạng Unsigned Integer 16-bit
fn ghi_du_lieu_u16(dau_ra: &mut [u16], so_kenh: usize, bo_tong_hop: &Arc<Mutex<Synthesizer>>) {
    let so_mau = dau_ra.len() / so_kenh;
    let (kenh_trai, kenh_phai) = xuat_khoi_am_thanh(bo_tong_hop, so_mau);

    for mau in 0..so_mau {
        // Tịnh tiến dải tín hiệu từ [-1.0, 1.0] thành [0.0, 1.0] rồi nhân với u16::MAX
        let trai = ((kenh_trai[mau].clamp(-1.0, 1.0) * 0.5 + 0.5) * u16::MAX as f32) as u16;
        let phai = ((kenh_phai[mau].clamp(-1.0, 1.0) * 0.5 + 0.5) * u16::MAX as f32) as u16;

        for kenh in 0..so_kenh {
            dau_ra[mau * so_kenh + kenh] = if kenh % 2 == 0 { trai } else { phai };
        }
    }
}