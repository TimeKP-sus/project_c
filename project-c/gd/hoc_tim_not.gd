extends HocTimNot

@onready var danh_sach: HFlowContainer = $DanhSach
var tim_not_scene: PackedScene = preload("res://scene/phanchoi/ten_not_tim.tscn")

func _ready() -> void:
	khoi_dong_bai_hoc()

func them_not_tim(chuoi_id: String) -> void:
	var tn = tim_not_scene.instantiate()
	tn.chuoi_id = chuoi_id
	tn.ten_not = self.chuyen_doi_id_thanh_ten(chuoi_id)
	danh_sach.add_child(tn)

func xoa_cac_not_tim() -> void:
	for i in danh_sach.get_children():
		i.queue_free()

func hien_thi_not_tim_dung(vi_tri: int) -> void:
	if vi_tri < danh_sach.get_child_count():
		var tn = danh_sach.get_child(vi_tri)
		tn.hien_not_dung()

func hien_thi_not_tim_sai(vi_tri: int) -> void:
	if vi_tri < danh_sach.get_child_count():
		var tn = danh_sach.get_child(vi_tri)
		tn.hieu_ung_sai()
