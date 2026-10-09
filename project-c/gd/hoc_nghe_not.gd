extends HocNgheNot

@onready var danh_sach: HFlowContainer = $DanhSach
var ten_not_scene: PackedScene = preload("res://scene/phanchoi/ten_not_doan.tscn")

func _ready() -> void:
	khoi_dong_bai_hoc()
	pass

# Đã sửa tham số nhận vào thành String
func them_not_doan(chuoi_id: String) -> void:
	var tn = ten_not_scene.instantiate()
	
	# Gán vào biến chuoi_id mới
	tn.chuoi_id = chuoi_id
	tn.ten_not = self.chuyen_doi_id_thanh_ten(chuoi_id)

	danh_sach.add_child(tn)

func xoa_cac_not_doan() -> void:
	for i in danh_sach.get_children():
		i.queue_free()

func hien_thi_not_doan_dung(vi_tri: int) -> void:
	if vi_tri < danh_sach.get_child_count():
		var tn = danh_sach.get_child(vi_tri)
		tn.hien_not_dung()

func hien_thi_not_doan_sai(vi_tri: int) -> void:
	if vi_tri < danh_sach.get_child_count():
		var tn = danh_sach.get_child(vi_tri)
		tn.hieu_ung_sai()

func _on_nghe_lai_button_down() -> void:
	self.phat_chuoi_am_thanh()
	pass
