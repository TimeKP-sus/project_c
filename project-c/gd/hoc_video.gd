extends HocVideo

@onready var video: VideoStreamPlayer = $video
@onready var panel: Control = $panel
@onready var thanh_tua: HSlider = $panel/thanh_tua
@onready var thoi_gian: Label = $panel/thanh_tua/thoi_gian
@onready var ten_bai_hoc: Label = $panel/ten_bai_hoc
@onready var nguoi_tao: Label = $panel/nguoi_tao

var cac_chu_thich:Array[Dictionary]

var dang_keo_thanh: bool = false
var video_dang_chay: bool = true
var dang_phat_truoc_khi_tua: bool = true

# Biến đếm thời gian chuột đứng yên
var thoi_gian_chuot_bat_dong: float = 0.0

var label_tween: Tween
var panel_tween: Tween

func _ready() -> void:
	var duong_dan = self.get_duong_dan_video()
	self.khoi_dong_bai_hoc()

	ten_bai_hoc.text = self.get_ten_bai_hoc()
	nguoi_tao.text = self.get_nguoi_tao()
	cac_chu_thich = self.get_cac_moc_thoi_gian()

	if duong_dan != null and duong_dan != "":
		self.duong_dan_video = duong_dan
		var luong_moi = NativeVideoStream.new() 
		luong_moi.file = self.duong_dan_video

		video.stream = luong_moi
		video.play()
		video.paused = false
		video_dang_chay = true
		
		thanh_tua.max_value = video.get_stream_length()
		
		hieu_ung_xuat_hien()
		hien_thi_panel()

func hieu_ung_xuat_hien():
	if label_tween: label_tween.kill()
	
	ten_bai_hoc.modulate.a = 0.0
	nguoi_tao.modulate.a = 0.0
	
	label_tween = create_tween()
	label_tween.tween_property(ten_bai_hoc, "modulate:a", 1.0, 0.5)
	label_tween.parallel().tween_property(nguoi_tao, "modulate:a", 1.0, 0.5)
	label_tween.tween_interval(4.0)
	label_tween.tween_property(ten_bai_hoc, "modulate:a", 0.0, 1.0)
	label_tween.parallel().tween_property(nguoi_tao, "modulate:a", 0.0, 1.0)

# ================= HỆ THỐNG PANEL THÔNG MINH =================

# BẮT SỰ KIỆN: Chuột có nhúc nhích là hiện Panel, reset đồng hồ về 0
func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion:
		thoi_gian_chuot_bat_dong = 0.0
		hien_thi_panel()

func hien_thi_panel():
	Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
	# Chỉ tạo hiệu ứng nếu panel đang bị mờ đi
	if panel.modulate.a < 1.0 and (panel_tween == null or not panel_tween.is_running()):
		if panel_tween: panel_tween.kill()
		panel_tween = create_tween()
		panel_tween.tween_property(panel, "modulate:a", 1.0, 0.2)

func an_panel():
	Input.set_mouse_mode(Input.MOUSE_MODE_HIDDEN)
	# Chỉ ẩn nếu nó đang hiện rõ
	if panel.modulate.a > 0.0 and (panel_tween == null or not panel_tween.is_running()):
		if panel_tween: panel_tween.kill()
		panel_tween = create_tween()
		panel_tween.tween_property(panel, "modulate:a", 0.0, 0.5)

# ================= XỬ LÝ VIDEO & THỜI GIAN =================

func _process(delta: float) -> void:
	var tong_thoi_gian: float = video.get_stream_length()

	if tong_thoi_gian > 0 and thanh_tua.max_value != tong_thoi_gian:
		thanh_tua.max_value = tong_thoi_gian
		
	var thoi_gian_hien_thi: float = 0.0
	
	if dang_keo_thanh:
		thoi_gian_hien_thi = thanh_tua.value
	else:
		if video.is_playing() and not video.paused:
			thanh_tua.value = video.stream_position
		thoi_gian_hien_thi = video.stream_position

	thoi_gian.text = dinh_dang_thoi_gian(thoi_gian_hien_thi) + " / " + dinh_dang_thoi_gian(tong_thoi_gian)

	if not dang_keo_thanh:
		thoi_gian_chuot_bat_dong += delta
		if thoi_gian_chuot_bat_dong > 1.5:
			an_panel()

# ================= KÉO THANH TUA =================

func _on_thanh_tua_drag_started() -> void:
	dang_keo_thanh = true
	# Lưu lại trạng thái trước khi tua và TẠM DỪNG video
	dang_phat_truoc_khi_tua = !video.paused
	video.paused = true
	video_dang_chay = false


func _on_thanh_tua_drag_ended(value_changed: bool) -> void:
	dang_keo_thanh = false
	
	if value_changed:
		video.stream_position = thanh_tua.value
		
	if dang_phat_truoc_khi_tua:
		video.paused = false
		video_dang_chay = true
	else:
		video_dang_chay = false
		if value_changed:
			video.paused = false
			await get_tree().process_frame
			await get_tree().process_frame
			video.paused = true  

# ================= NÚT TƯƠNG TÁC =================

func _on_video_gui_input(event: InputEvent) -> void:
	# Click trực tiếp lên màn hình video để Dừng/Phát (Chuẩn Youtube)
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		_on_dung_lai_button_down()

func _on_dung_lai_button_down() -> void:
	if video_dang_chay:
		video.paused = true
		video_dang_chay = false
	else:
		video.paused = false
		video_dang_chay = true

func dinh_dang_thoi_gian(giay: float) -> String:
	var phut:int = int(giay / 60.0)
	var giay_le:int = int(giay) % 60
	return "%02d:%02d" % [phut, giay_le]


func _on_menu_button_down() -> void:
	pass # Replace with function body.

@onready var hoan_thanh: Button = $hoan_thanh
func _on_video_finished() -> void:
	hien_thi_panel()
	hoan_thanh.visible = true
	pass # Replace with function body.
