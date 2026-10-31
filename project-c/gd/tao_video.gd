extends TaoVideo

@onready var video: VideoStreamPlayer = $KhungVideo/VideoStreamPlayer
@onready var thanh_tua: HSlider = $KhungVideo/ThanhTua
@onready var thoi_gian_lbl: Label = $KhungVideo/ThoiGianLbl

var video_dang_chay: bool = false
var dang_keo_thanh: bool = false
var dang_phat_truoc_khi_tua: bool = false



# ----------------- HÀM ĐƯỢC RUST GỌI KHI LOAD VIDEO -----------------
func load_va_phat_video(duong_dan: String) -> void:
	if duong_dan == "" or duong_dan == null:
		return
		
	# QUAN TRỌNG 2: Dừng hẳn luồng video hiện tại trước khi nạp luồng mới
	video.stop()
	
	# QUAN TRỌNG 3: Đổi đường dẫn res:// thành đường dẫn thực tế (Absolute Path) để Add-on ngoài đọc được
	var duong_dan_tuyet_doi = ProjectSettings.globalize_path(duong_dan)
	
	var luong_moi = NativeVideoStream.new() 
	luong_moi.file = duong_dan_tuyet_doi
	
	video.stream = luong_moi
	video.play()
	video.paused = false
	video_dang_chay = true
	
	print("Bắt đầu xem trước video tại: ", duong_dan_tuyet_doi)

# ----------------- CẬP NHẬT GIAO DIỆN MỖI FRAME -----------------
func _process(_delta: float) -> void:
	if video.stream == null:
		return
		
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

	thoi_gian_lbl.text = dinh_dang_thoi_gian(thoi_gian_hien_thi) + " / " + dinh_dang_thoi_gian(tong_thoi_gian)

# ----------------- XỬ LÝ CLICK ĐỂ PLAY/PAUSE -----------------
func _on_video_stream_player_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		if video.stream != null:
			if video_dang_chay:
				video.paused = true
				video_dang_chay = false
			else:
				video.paused = false
				video_dang_chay = true

# ================= KÉO THANH TUA =================
func _on_thanh_tua_drag_started() -> void:
	dang_keo_thanh = true
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

# ----------------- HÀM ĐỊNH DẠNG THỜI GIAN -----------------
func dinh_dang_thoi_gian(giay: float) -> String:
	var phut: int = int(giay / 60.0)
	var giay_le: int = int(giay) % 60
	return "%02d:%02d" % [phut, giay_le]
