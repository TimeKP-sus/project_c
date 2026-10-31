extends Panel

@onready var hoc_tim_not: HocTimNot = $"../.."

@onready var ten: Label = $ten
@onready var nguoi_tao: Label = $nguoi_tao
@onready var diem: Label = $diem
@onready var diem_cu: Label = $diem_cu
@onready var so_lan_dung: Label = $so_lan_dung
@onready var so_lan_sai: Label = $so_lan_sai
@onready var danh_gia: Label = $danh_gia
@onready var so_lan_doan: Label = $so_lan_doan
@onready var do_chinh_xac: Label = $do_chinh_xac
@onready var nen_tong_ket: Panel = $".."

func _ready() -> void:
	# Ẩn panel tổng kết lúc mới vào game
	nen_tong_ket.hide()

func _on_ve_ds_button_down() -> void:
	get_tree().change_scene_to_file("res://scene/menu/DanhSachBaiHoc.tscn")

func _on_choi_lai_button_down() -> void:
	var scene_hien_tai = get_tree().current_scene
	var duong_dan_da_luu = scene_hien_tai.duong_dan_bai_hoc

	var hoc_tim_not_scene = load("res://scene/phanchoi/HocTimNot.tscn")
	var scene_moi = hoc_tim_not_scene.instantiate()

	scene_moi.duong_dan_bai_hoc = duong_dan_da_luu

	get_tree().root.add_child(scene_moi)
	get_tree().current_scene = scene_moi
	scene_hien_tai.queue_free()

func _on_hoc_tim_not_xong_het_bai() -> void:
	nen_tong_ket.show()

	# Lấy thông tin cơ bản
	ten.text = "Bài học: " + str(hoc_tim_not.get_ten_bai_hoc())
	nguoi_tao.text = "Người tạo: " + str(hoc_tim_not.get_nguoi_tao())

	# Lấy thống kê
	var dung = hoc_tim_not.get_so_lan_dung()
	var sai = hoc_tim_not.get_so_lan_sai()
	var doan = dung + sai # Tổng số lần đã đánh giá
	var diem_so = hoc_tim_not.get_diem()
	var ky_luc_cu = hoc_tim_not.get_diem_cu()

	so_lan_dung.text = "✔ Đúng: " + str(dung)
	so_lan_sai.text = "✖ Sai: " + str(sai)
	so_lan_doan.text = "🎯 Số lần đánh: " + str(doan)

	# Hiển thị điểm và so sánh kỷ lục
	diem_cu.text = "Điểm kỷ lục: " + str(ky_luc_cu)
	if diem_so > ky_luc_cu:
		diem.text = "Tổng điểm: " + str(diem_so) + " (Kỷ lục mới!)"
	else:
		diem.text = "Tổng điểm: " + str(diem_so)

	# Tính % chính xác
	var chinh_xac: float = 0.0
	if doan > 0:
		chinh_xac = (float(dung) / float(doan)) * 100.0
		
	do_chinh_xac.text = "Mức độ hoàn thành: %.2f%%" % chinh_xac

	# Hiển thị hạng (Grade) tương ứng với tỷ lệ chính xác
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
