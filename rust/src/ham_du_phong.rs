 pub fn kiem_tra_phim(&mut self) {
        let danh_sach_phim_bam: HashSet<i32> = self
            .ban_phim_midi_node
            .as_ref()
            .unwrap()
            .bind()
            .get_danh_sach_phim_bam();
        let cac_not_trong_vung: Vec<Gd<crate::not_nhac::NotNhac>> = self
            .quan_ly_sheet_node
            .as_ref()
            .unwrap()
            .bind()
            .get_danh_sach_not_trong_area();
        for mut not_node in cac_not_trong_vung {
            let id_cua_not: i32 = not_node.bind().get_id_not();
            if danh_sach_phim_bam.contains(&id_cua_not) {
                not_node.bind_mut().da_duoc_danh();
                self.diem += 100;
                break;
            }
        }
    }
fn test_nha_phim(){
    let vi_tri_x = not_node.get_position().x; 
    let chieu_dai = not_node.bind().get_chieu_dai_duoi();
    let toa_do_ket_thuc = vi_tri_x + chieu_dai; 

    if toa_do_ket_thuc <= 150.0 { 
        self.diem += 200;
        godot_print!("Perfect Hold! Điểm: {}", self.diem);
    } else {
        self.diem += 50; 
        godot_print!("Nhả tay quá sớm!");
}
}
