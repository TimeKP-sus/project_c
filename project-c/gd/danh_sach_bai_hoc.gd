extends DanhSachBaiHoc

@onready var ds_bai_hoc: VBoxContainer = $menu_giua/ScrollContainer/DanhSachBaiHoc
@onready var danh_sach_chuong: VBoxContainer = $menu_trai/ScrollContainer/DanhSachChuong
@onready var thong_tin: RichTextLabel = $menu_phai/thong_tin

var chuong_button: PackedScene = preload("res://scene/menu/chuong_button.tscn")
var bai_hoc_panel: PackedScene = preload("res://scene/menu/bai_hoc_panel.tscn")

func _ready() -> void:
	cat_nhat_danh_sach_chuong()
	pass

func cat_nhat_danh_sach_chuong():
	var ds = lay_danh_sach_chuong()
	for i in ds:
		var chuong:ChuongButton = chuong_button.instantiate()
		chuong.ten_chuong = i.ten_chuong
		chuong.ten_file = i.ten_file
		danh_sach_chuong.add_child(chuong)
	pass

func xoa_danh_sach_bai_hoc():
	for i in ds_bai_hoc.get_children():
		i.queue_free()
	pass

func cat_nhat_danh_sach_bai_hoc(ten_file: String):
	xoa_danh_sach_bai_hoc()
	var ds = lay_json_bai_hoc(ten_file)
	var so = 1
	for i in ds:
		var bh:BaiHocPanel = bai_hoc_panel.instantiate()
		bh.so = str(so)
		so+=1
		bh.ten = i.ten_bai_hoc
		bh.loai_bai_hoc = i.loai_bai_hoc
		bh.bai_hoc_path = i.duong_dan
		ds_bai_hoc.add_child(bh)
	pass
