extends PhimTuDo

@onready var vung_not_bay: Control = $VungNotBay
@onready var midi_piano = $MIDI/BanPhimPiano

@onready var btn_ghi_am = $ThanhThongTin/HBox/GhiAm_Btn
@onready var btn_dung = $ThanhThongTin/HBox/Dung_Btn
@onready var btn_phat = $ThanhThongTin/HBox/Phat_Btn

@onready var hop_thoai_mo = $HopThoaiMoFile
@onready var hop_thoai_luu = $HopThoaiLuuFile

# --- THÊM NODE HIỂN THỊ TÊN FILE ---
@onready var lbl_ten_file = $TenFilePhatLai_Label

var cac_not_dang_giu = {}
var cac_not_dang_bay = []
var toc_do_bay = 240.0 

func _ready():
	if midi_piano:
		midi_piano.connect("phim_vua_duoc_bam", Callable(self, "_on_nguoi_dung_bam"))
		midi_piano.connect("phim_vua_duoc_nha", Callable(self, "_on_nguoi_dung_nha"))
		
	connect("hieu_lenh_phat_phim", Callable(self, "_tao_hinh_not_bam"))
	connect("hieu_lenh_nha_phim", Callable(self, "_ket_thuc_hinh_not"))
	connect("ket_thuc_phat_lai", Callable(self, "_on_ket_thuc_phat"))
	
	cap_nhat_giao_dien_nut()

func _on_nguoi_dung_bam(so_phim: int):
	ghi_nhan_phim_bam(so_phim) 
	_tao_hinh_not_bam(so_phim)

func _on_nguoi_dung_nha(so_phim: int):
	ghi_nhan_phim_nha(so_phim) 
	_ket_thuc_hinh_not(so_phim)

func _tao_hinh_not_bam(so_phim: int):
	if so_phim < 0 or so_phim > 83: return
	
	# Fix lỗi dội phím: Bắt buộc đóng nốt cũ trước khi mở nốt mới
	if cac_not_dang_giu.has(so_phim):
		_ket_thuc_hinh_not(so_phim)
	
	if lay_trang_thai() == 2 and midi_piano and midi_piano.has_method("phat_am_thanh_not"):
		midi_piano.phat_am_thanh_not(so_phim, 100, 300) 
	
	var thong_tin = get_thong_tin_phim(so_phim)
	var panel = Panel.new()
	var style = StyleBoxFlat.new()
	style.bg_color = thong_tin.mau_sac
	style.corner_radius_top_left = 6
	style.corner_radius_top_right = 6
	style.corner_radius_bottom_left = 6
	style.corner_radius_bottom_right = 6
	panel.add_theme_stylebox_override("panel", style)
	
	var label = Label.new()
	label.text = thong_tin.ten_not
	label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	label.vertical_alignment = VERTICAL_ALIGNMENT_TOP
	label.set_anchors_preset(PRESET_TOP_WIDE)
	label.add_theme_font_size_override("font_size", 18)
	label.add_theme_color_override("font_color", Color.WHITE)
	label.add_theme_color_override("font_outline_color", Color.BLACK)
	label.add_theme_constant_override("outline_size", 6)
	label.offset_top = 4 
	panel.add_child(label)
	
	panel.size = Vector2(thong_tin.chieu_rong, 0)
	panel.position = Vector2(thong_tin.toa_do_x, vung_not_bay.size.y)
	
	# Lớp bảo vệ chắc chắn 100% không văng lỗi "already has a parent"
	if panel.get_parent() == null:
		vung_not_bay.add_child(panel)
		cac_not_dang_giu[so_phim] = panel

func _ket_thuc_hinh_not(so_phim: int):
	if cac_not_dang_giu.has(so_phim):
		var panel = cac_not_dang_giu[so_phim]
		cac_not_dang_giu.erase(so_phim)
		cac_not_dang_bay.append(panel)

func don_dep_not():
	for so_phim in cac_not_dang_giu:
		if is_instance_valid(cac_not_dang_giu[so_phim]):
			cac_not_dang_giu[so_phim].queue_free()
	cac_not_dang_giu.clear()
	for panel in cac_not_dang_bay:
		if is_instance_valid(panel):
			panel.queue_free()
	cac_not_dang_bay.clear()

func _process(delta):
	xu_ly_thoi_gian(delta)
	
	var do_tang = toc_do_bay * delta
	
	for so_phim in cac_not_dang_giu:
		var panel = cac_not_dang_giu[so_phim]
		panel.size.y += do_tang
		panel.position.y -= do_tang

	for i in range(cac_not_dang_bay.size() - 1, -1, -1):
		var panel = cac_not_dang_bay[i]
		panel.position.y -= do_tang
		if panel.position.y + panel.size.y < 0:
			panel.queue_free()
			cac_not_dang_bay.remove_at(i)

# --- XỬ LÝ NÚT BẤM & FILE ---
func _on_ghi_am_btn_pressed():
	don_dep_not()
	bat_dau_ghi()
	cap_nhat_giao_dien_nut()

func _ngat_toan_bo_not_dang_giu():
	var cac_phim_dang_ket = cac_not_dang_giu.keys()
	for so_phim in cac_phim_dang_ket:
		_ket_thuc_hinh_not(so_phim)

func _on_dung_btn_pressed():
	dung_hoat_dong()
	_ngat_toan_bo_not_dang_giu()
	cap_nhat_giao_dien_nut()

func _on_phat_btn_pressed():
	don_dep_not()
	phat_lai()
	cap_nhat_giao_dien_nut()
	
func _on_ket_thuc_phat():
	_ngat_toan_bo_not_dang_giu()
	cap_nhat_giao_dien_nut()

func _on_mo_file_btn_pressed():
	hop_thoai_mo.popup_centered()

func _on_luu_file_btn_pressed():
	hop_thoai_luu.popup_centered()

func _on_hop_thoai_mo_file_file_selected(path: String):
	don_dep_not()
	if mo_ban_ghi(path):
		# Lấy tên file từ đường dẫn (ví dụ: "C:/abc/xyz.json" -> "xyz.json")
		if lbl_ten_file:
			lbl_ten_file.text = "🎵 Đang phát: " + path.get_file()
	cap_nhat_giao_dien_nut()

func _on_hop_thoai_luu_file_file_selected(path: String):
	luu_ban_ghi(path)

func cap_nhat_giao_dien_nut():
	var tt = lay_trang_thai() 
	btn_ghi_am.disabled = (tt != 0)
	btn_phat.disabled = (tt != 0)
	btn_dung.disabled = (tt == 0)
	
	btn_ghi_am.text = "🔴 Đang ghi..." if tt == 1 else "⏺ Ghi âm"
	btn_phat.text = "▶ Đang phát..." if tt == 2 else "▶ Phát lại"
	
	# --- CẬP NHẬT HIỂN THỊ TÊN FILE ---
	# Chỉ hiện tên file (Label) khi trạng thái là 2 (Đang Phát)
	if lbl_ten_file:
		lbl_ten_file.visible = (tt == 2)

func _on_ve_menu_btn_pressed():
	get_tree().change_scene_to_file("res://scene/menu/MenuChinh.tscn")
