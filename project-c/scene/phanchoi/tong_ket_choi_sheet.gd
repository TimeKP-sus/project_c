extends Panel

@onready var choi_sheet: ChoiSheet = $"../.."
@onready var quan_ly_sheet: QuanLySheet = $"../../QuanLySheet"

@onready var nen_tong_ket: Panel = $".."
@onready var ten: Label = $ten
@onready var tac_gia: Label = $tac_gia
@onready var nguoi_tao: Label = $nguoi_tao
@onready var diem: Label = $diem
@onready var diem_cu: Label = $diem_cu
@onready var tot: Label = $tot
@onready var kha: Label = $kha
@onready var trat: Label = $trat
@onready var danh_gia: Label = $danh_gia
@onready var do_chinh_xac: Label = $do_chinh_xac
@onready var ve_ds_btn: Button = $ve_ds

func _ready() -> void:
	nen_tong_ket.hide()

func _on_choi_lai_button_down() -> void:
	var cay_thu_muc = get_tree()
	var goc = cay_thu_muc.root
	
	if choi_sheet.che_do_choi_thu:
		var tao_tap_scene = cay_thu_muc.current_scene
		
		# Gỡ an toàn bằng biến đã lưu
		goc.remove_child(choi_sheet)
		choi_sheet.queue_free()
		
		if tao_tap_scene and tao_tap_scene.has_method("choi_thu_doan_luyen_tap"):
			tao_tap_scene.call_deferred("choi_thu_doan_luyen_tap")
		return

	# Chế độ chơi bình thường từ Danh sách bài học
	var duong_dan_da_luu = choi_sheet.duong_dan_bai_hoc
	var la_luyen_tap = choi_sheet.phan_luyen_tap

	var choi_sheet_scene = load("res://scene/phanchoi/ChoiSheet.tscn")
	var scene_moi = choi_sheet_scene.instantiate()

	scene_moi.duong_dan_bai_hoc = duong_dan_da_luu
	scene_moi.phan_luyen_tap = la_luyen_tap

	# Gỡ an toàn bằng biến đã lưu
	goc.remove_child(choi_sheet)
	choi_sheet.queue_free()

	# Thêm Scene mới
	goc.add_child(scene_moi)
	cay_thu_muc.current_scene = scene_moi

func _on_ve_ds_button_down() -> void:
	if choi_sheet.che_do_choi_thu:
		choi_sheet.thoat_choi_thu()
	else:
		get_tree().change_scene_to_file("res://scene/menu/DanhSachBaiHoc.tscn")

func _on_quan_ly_sheet_bai_hat_ket_thuc() -> void:
	nen_tong_ket.show()
	if choi_sheet.che_do_choi_thu and ve_ds_btn:
		ve_ds_btn.text = "⬅ Về tạo bài"

	ten.text = quan_ly_sheet.get_ten_bai_hat()
	tac_gia.text = "Tác giả: " + quan_ly_sheet.get_tac_gia()
	nguoi_tao.text = "Người tạo: " + quan_ly_sheet.get_nguoi_tao_sheet()

	var so_tot = choi_sheet.get_so_phim_tot()
	var so_kha = choi_sheet.get_so_phim_kha()
	var so_trat = choi_sheet.get_so_phim_truot()
	var diem_moi = choi_sheet.get_diem()
	var ky_luc_cu = choi_sheet.lay_diem_cao_nhat_cu()

	tot.text = "Tốt: " + str(so_tot)
	kha.text = "Khá: " + str(so_kha)
	trat.text = "Trượt: " + str(so_trat)

	diem_cu.text = "Điểm kỷ lục: " + str(ky_luc_cu)
	if diem_moi > ky_luc_cu:
		diem.text = "Tổng điểm: " + str(diem_moi) + " (Kỷ lục mới!)"
	else:
		diem.text = "Tổng điểm: " + str(diem_moi)

	var tong_not_da_danh = so_tot + so_kha + so_trat
	var chinh_xac: float = 0.0

	if tong_not_da_danh > 0:
		# Tốt = 100%, Khá = 50%, Trật = 0%
		chinh_xac = (so_tot * 1.0 + so_kha * 0.5) / float(tong_not_da_danh) * 100.0

	do_chinh_xac.text = "Mức độ hoàn thành: %.2f%%" % chinh_xac

	if chinh_xac >= 100.0:
		danh_gia.text = "S+"
		danh_gia.modulate = Color.GOLD
	elif chinh_xac >= 95.0:
		danh_gia.text = "S"
		danh_gia.modulate = Color.YELLOW
	elif chinh_xac >= 90.0:
		danh_gia.text = "A"
		danh_gia.modulate = Color.GREEN_YELLOW
	elif chinh_xac >= 70.0:
		danh_gia.text = "B"
		danh_gia.modulate = Color.DEEP_SKY_BLUE
	elif chinh_xac >= 50.0:
		danh_gia.text = "C"
		danh_gia.modulate = Color.ORANGE
	elif chinh_xac >= 30.0:
		danh_gia.text = "D"
		danh_gia.modulate = Color.ORANGE_RED
	else:
		danh_gia.text = "F"
		danh_gia.modulate = Color.GRAY
