extends DanhSachBaiHoc

@onready var ds_bai_hoc: VBoxContainer = $menu_giua/ScrollContainer/DanhSachBaiHoc
@onready var danh_sach_chuong: VBoxContainer = $menu_trai/ScrollContainer/DanhSachChuong

@onready var thong_tin: RichTextLabel = $menu_phai/thong_tin
@onready var tim_kiem: TextEdit = $menu_tren/tim_kiem
@onready var quay_lai_btn: Button = $quay_lai

# Các Node bộ lọc mới trên menu_tren
@onready var loc_loai_opt: OptionButton = $menu_tren/BoLoc/LocLoaiBai_Option
@onready var sap_xep_opt: OptionButton = $menu_tren/BoLoc/SapXep_Option
@onready var dat_lai_btn: Button = $menu_tren/BoLoc/DatLaiLoc_Btn

var chuong_button: PackedScene = preload("res://scene/menu/chuong_button.tscn")
var bai_hoc_panel: PackedScene = preload("res://scene/menu/bai_hoc_panel.tscn")

func _ready() -> void:
	# Bổ sung mục "Học qua Video" vào OptionButton nếu chưa có
	if loc_loai_opt and loc_loai_opt.item_count == 5:
		loc_loai_opt.add_item("🎬 Học qua Video", 5)

	# Kết nối sự kiện cho các bộ lọc và nút quay lại
	if loc_loai_opt:
		loc_loai_opt.item_selected.connect(_khi_thay_doi_bo_loc)
	if sap_xep_opt:
		sap_xep_opt.item_selected.connect(_khi_thay_doi_bo_loc)
	if dat_lai_btn:
		dat_lai_btn.pressed.connect(dat_lai_bo_loc)
	if quay_lai_btn and not quay_lai_btn.pressed.is_connected(_on_quay_lai_pressed):
		quay_lai_btn.pressed.connect(_on_quay_lai_pressed)

	cat_nhat_danh_sach_chuong()
	# Tự động hiển thị toàn bộ bài học ngay khi vừa mở màn hình
	lay_tat_ca_bai_hoc()
	danh_dau_chuong_dang_chon("all")


func cat_nhat_danh_sach_chuong() -> void:
	for child in danh_sach_chuong.get_children():
		child.queue_free()

	var ds = lay_danh_sach_chuong()
	var chuong_all: ChuongButton = chuong_button.instantiate()
	chuong_all.name = "all"
	chuong_all.ten_chuong = "📚 Tất cả bài học"
	danh_sach_chuong.add_child(chuong_all)

	for i in ds:
		var chuong: ChuongButton = chuong_button.instantiate()
		chuong.ten_chuong = i.ten_chuong
		chuong.ten_file = i.ten_file
		danh_sach_chuong.add_child(chuong)


func danh_dau_chuong_dang_chon(ten_file_chon: String) -> void:
	for nut in danh_sach_chuong.get_children():
		if nut is ChuongButton:
			var dang_chon = (ten_file_chon == "all" and nut.name == "all") or (nut.ten_file == ten_file_chon)
			nut.modulate = Color(1.15, 1.15, 0.85, 1.0) if dang_chon else Color(1, 1, 1, 1)


func xoa_danh_sach_bai_hoc() -> void:
	for i in ds_bai_hoc.get_children():
		ds_bai_hoc.remove_child(i)
		i.queue_free()


func tao_the_bai_hoc(so_thu_tu: int, du_lieu: Dictionary) -> void:
	var bh: BaiHocPanel = bai_hoc_panel.instantiate()
	bh.so = str(so_thu_tu)
	bh.ten = du_lieu.get("ten_bai_hoc", "Bài học chưa đặt tên")
	bh.loai_bai_hoc = du_lieu.get("loai_bai_hoc", "")
	bh.bai_hoc_path = du_lieu.get("duong_dan", "")
	bh.nguoi_tao = du_lieu.get("nguoi_tao", "TimeKP")
	ds_bai_hoc.add_child(bh)


func lay_tat_ca_bai_hoc() -> void:
	xoa_danh_sach_bai_hoc()
	var danh_sach_chuong_json = lay_danh_sach_chuong()
	var so = 1

	for chuong in danh_sach_chuong_json:
		var ds_bai_hoc_trong_chuong = lay_json_bai_hoc(chuong.ten_file)
		for bai in ds_bai_hoc_trong_chuong:
			tao_the_bai_hoc(so, bai)
			so += 1

	ap_dung_bo_loc()


func cat_nhat_danh_sach_bai_hoc(ten_file: String) -> void:
	xoa_danh_sach_bai_hoc()
	var ds = lay_json_bai_hoc(ten_file)
	var so = 1
	for bai in ds:
		tao_the_bai_hoc(so, bai)
		so += 1

	ap_dung_bo_loc()


func _on_tim_kiem_text_changed() -> void:
	ap_dung_bo_loc()


func _khi_thay_doi_bo_loc(_index: int) -> void:
	ap_dung_bo_loc()


func ap_dung_bo_loc() -> void:
	var tu_khoa: String = tim_kiem.text.strip_edges().to_lower() if tim_kiem else ""
	var che_do_chon: int = loc_loai_opt.selected if loc_loai_opt else 0
	var kieu_sap_xep: int = sap_xep_opt.selected if sap_xep_opt else 0

	var cac_panel: Array = []
	for child in ds_bai_hoc.get_children():
		if child is BaiHocPanel:
			cac_panel.append(child)

	# 1. Lọc theo từ khóa và loại bài học
	for bh: BaiHocPanel in cac_panel:
		var khop_tu_khoa: bool = tu_khoa.is_empty() or (tu_khoa in bh.ten.to_lower()) or (tu_khoa in bh.nguoi_tao.to_lower())
		var khop_loai: bool = true

		match che_do_chon:
			1: khop_loai = (bh.loai_bai_hoc == "choi_sheet")
			2: khop_loai = (bh.loai_bai_hoc == "tap_luyen_sheet" or bh.loai_bai_hoc == "luyen_tap_sheet")
			3: khop_loai = (bh.loai_bai_hoc == "nghe_va_doan_not")
			4: khop_loai = (bh.loai_bai_hoc == "tim_not")
			5: khop_loai = (bh.loai_bai_hoc == "xem_video")
			_: khop_loai = true

		bh.visible = khop_tu_khoa and khop_loai

	# 2. Sắp xếp thứ tự các thẻ bài học
	match kieu_sap_xep:
		0: # Mặc định theo số thứ tự ban đầu
			cac_panel.sort_custom(func(a: BaiHocPanel, b: BaiHocPanel): return a.so.to_int() < b.so.to_int())
		1: # Tên A -> Z
			cac_panel.sort_custom(func(a: BaiHocPanel, b: BaiHocPanel): return a.ten.naturalnocasecmp_to(b.ten) < 0)
		2: # Tên Z -> A
			cac_panel.sort_custom(func(a: BaiHocPanel, b: BaiHocPanel): return a.ten.naturalnocasecmp_to(b.ten) > 0)

	for idx in range(cac_panel.size()):
		ds_bai_hoc.move_child(cac_panel[idx], idx)


func dat_lai_bo_loc() -> void:
	if tim_kiem:
		tim_kiem.text = ""
	if loc_loai_opt:
		loc_loai_opt.select(0)
	if sap_xep_opt:
		sap_xep_opt.select(0)
	ap_dung_bo_loc()


func _on_quay_lai_pressed() -> void:
	get_tree().change_scene_to_file("res://scene/menu/MenuChinh.tscn")
