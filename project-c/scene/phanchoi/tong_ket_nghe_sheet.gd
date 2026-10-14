extends Panel

@onready var hoc_nghe_not:HocNgheNot = $"../.."
@onready var nen_tong_ket: Panel = $".."

@onready var ten: Label = $ten
@onready var nguoi_tao: Label = $nguoi_tao
@onready var diem: Label = $diem
@onready var diem_cu: Label = $diem_cu
@onready var so_lan_dung: Label = $so_lan_dung
@onready var so_lan_sai: Label = $so_lan_sai
@onready var so_lan_nghe_lai: Label = $so_lan_nghe_lai
@onready var so_lan_doan: Label = $so_lan_doan
@onready var danh_gia: Label = $danh_gia
@onready var do_chinh_xac: Label = $do_chinh_xac

func _ready() -> void:
	# Ẩn panel tổng kết lúc mới vào game
	nen_tong_ket.hide()

func _on_ve_ds_button_down() -> void:
	get_tree().change_scene_to_file("res://scene/menu/DanhSachBaiHoc.tscn")

func _on_choi_lai_button_down() -> void:
	var scene_hien_tai = get_tree().current_scene
	var duong_dan_da_luu = scene_hien_tai.duong_dan_bai_hoc

	# Đảm bảo đường dẫn này trỏ đúng tới Scene luyện nghe của bạn
	var hoc_nghe_not_scene = load("res://scene/phanchoi/HocNgheNot.tscn")
	var scene_moi = hoc_nghe_not_scene.instantiate()

	scene_moi.duong_dan_bai_hoc = duong_dan_da_luu

	get_tree().root.add_child(scene_moi)
	get_tree().current_scene = scene_moi
	scene_hien_tai.queue_free()

func _on_hoc_nghe_not_xong_het_bai() -> void:
	nen_tong_ket.show()

	ten.text = "Bài học: " + str(hoc_nghe_not.get_ten_bai_hoc())
	nguoi_tao.text = "Người tạo: " + str(hoc_nghe_not.get_nguoi_tao())

	# Lấy dữ liệu thống kê từ Node Rust
	var dung = hoc_nghe_not.get_so_lan_dung()
	var sai = hoc_nghe_not.get_so_lan_sai()
	var doan = hoc_nghe_not.get_so_lan_doan()
	var nghe_lai = hoc_nghe_not.get_so_lan_nghe()
	var diem_so = hoc_nghe_not.get_diem()


	so_lan_dung.text = "Đúng: " + str(dung)
	so_lan_sai.text = "Sai: " + str(sai)
	so_lan_doan.text = "Tổng lần đoán: " + str(doan)
	so_lan_nghe_lai.text = "Nghe lại: " + str(nghe_lai)
	diem.text = str(diem_so)

	var chinh_xac: float = 0.0
	if doan > 0:
		var trong_so_phat = 0.3
		chinh_xac = (float(dung) / float(doan + (nghe_lai * trong_so_phat))) * 100.0
		
	do_chinh_xac.text = "%.2f%%" % chinh_xac
	
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
