extends DanhSachBaiHoc

@onready var ds_bai_hoc: VBoxContainer = $menu_giua/ScrollContainer/DanhSachBaiHoc
@onready var danh_sach_chuong: VBoxContainer = $menu_trai/ScrollContainer/DanhSachChuong

@onready var thong_tin: RichTextLabel = $menu_phai/thong_tin
@onready var tim_kiem: TextEdit = $menu_tren/tim_kiem


var chuong_button: PackedScene = preload("res://scene/menu/chuong_button.tscn")
var bai_hoc_panel: PackedScene = preload("res://scene/menu/bai_hoc_panel.tscn")


func _ready() -> void:
	cat_nhat_danh_sach_chuong()
	pass

func cat_nhat_danh_sach_chuong():
	var ds = lay_danh_sach_chuong()
	var chuong_all:ChuongButton = chuong_button.instantiate()
	chuong_all.name = "all"
	chuong_all.ten_chuong = "Tất cả"
	danh_sach_chuong.add_child(chuong_all)
	
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

func lay_tat_ca_bai_hoc():
	xoa_danh_sach_bai_hoc()
	var danh_sach_chuong_json = lay_danh_sach_chuong()
	
	var so = 1 

	for chuong in danh_sach_chuong_json:
		var ds_bai_hoc_trong_chuong = lay_json_bai_hoc(chuong.ten_file)
		for bai in ds_bai_hoc_trong_chuong:
			var bh: BaiHocPanel = bai_hoc_panel.instantiate()
			bh.so = str(so)
			bh.ten = bai.ten_bai_hoc
			bh.loai_bai_hoc = bai.loai_bai_hoc
			bh.bai_hoc_path = bai.duong_dan
			
			ds_bai_hoc.add_child(bh)
			so += 1

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


func _on_tim_kiem_text_changed() -> void:
	# Lấy từ khóa, xóa khoảng trắng thừa ở hai đầu và chuyển thành chữ thường
	var tu_khoa = tim_kiem.text.strip_edges().to_lower()
	
	# Duyệt qua tất cả các bài học (BaiHocPanel) đang có trong danh sách
	for bh in ds_bai_hoc.get_children():
		# Nếu ô tìm kiếm trống, hoặc tên bài học có chứa từ khóa
		if tu_khoa == "" or tu_khoa in bh.ten.to_lower():
			bh.visible = true  # Hiện bài học
		else:
			bh.visible = false # Ẩn bài học
