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

func _ready() -> void:
	# Ẩn panel tổng kết lúc mới vào game
	nen_tong_ket.hide()

func _on_choi_lai_button_down() -> void:
	var scene_hien_tai = get_tree().current_scene
	var duong_dan_da_luu = scene_hien_tai.duong_dan_bai_hoc

	var choi_sheet_scene = load("res://scene/phanchoi/ChoiSheet.tscn")
	var scene_moi = choi_sheet_scene.instantiate()

	scene_moi.duong_dan_bai_hoc = duong_dan_da_luu

	get_tree().root.add_child(scene_moi)
	get_tree().current_scene = scene_moi
	scene_hien_tai.queue_free()

func _on_ve_ds_button_down() -> void:
	get_tree().change_scene_to_file("res://scene/menu/DanhSachBaiHoc.tscn")

func _on_quan_ly_sheet_bai_hat_ket_thuc() -> void:
	nen_tong_ket.show()

	ten.text = quan_ly_sheet.get_tac_gia()
	tac_gia.text = "Tác giả: " + quan_ly_sheet.get_tac_gia()
	nguoi_tao.text = "Người tạo: " + quan_ly_sheet.get_nguoi_tao_sheet()
	

	var so_tot = choi_sheet.get_so_phim_tot()
	var so_kha = choi_sheet.get_so_phim_kha()
	var so_trat = choi_sheet.get_so_phim_truot()
	
	tot.text =  "Tốt: " + str(so_tot)
	kha.text = "Khá: " + str(so_kha)
	trat.text = "Trật: " + str(so_trat)
	diem.text = str(choi_sheet.get_diem())
	
	var tong_not_da_danh = so_tot + so_kha + so_trat
	var chinh_xac: float = 0.0
	
	if tong_not_da_danh > 0:
		# Tốt = 100%, Khá = 50%, Trật = 0%
		chinh_xac = (so_tot * 1.0 + so_kha * 0.5) / float(tong_not_da_danh) * 100.0
		
	do_chinh_xac.text = "%.2f%%" % chinh_xac
	
	if chinh_xac >= 100.0:
		danh_gia.text = "SSS"
		danh_gia.modulate = Color.GOLD
	if chinh_xac >= 95.0:
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
