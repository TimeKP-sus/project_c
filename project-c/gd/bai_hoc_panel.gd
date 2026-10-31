class_name BaiHocPanel
extends Panel

var so: String = "1"
var ten: String = ""
var loai_bai_hoc: String = ""
var nguoi_tao: String = ""
var bai_hoc_path: String = ""

var diem_cao_nhat: int = 0
var ti_le_hoan_thanh: float = 0.0
var so_lan_choi: int = 0
var hang_tot_nhat: String = "-"

@onready var ten_label: Label = $ten
@onready var so_label: Label = $so
@onready var nguoi_tao_label: Label = $nguoi_tao
@onready var ket_qua_label: Label = get_node_or_null("ket_qua")
@onready var loai_bai_label: Label = $loai_bai
@onready var danh_sach_bai_hoc: DanhSachBaiHoc = $"../../../.."
@onready var thong_tin_label: RichTextLabel = $"../../../../menu_phai/thong_tin"

const PATH_CHOI_SHEET = "res://scene/phanchoi/ChoiSheet.tscn"
const PATH_NGHE_NOT = "res://scene/phanchoi/HocNgheNot.tscn"
const PATH_VIDEO = "res://scene/phanchoi/HocVideo.tscn"
const PATH_TIM_NOT = "res://scene/phanchoi/HocTimNot.tscn"


func lay_ten_che_do_hien_thi() -> String:
	match loai_bai_hoc:
		"choi_sheet":
			return "🎼 Đọc Bản Nhạc"
		"tap_luyen_sheet", "luyen_tap_sheet":
			return "🎯 Luyện Tập Đoạn"
		"nghe_va_doan_not":
			return "👂 Nghe & Đoán Nốt"
		"tim_not":
			return "🎹 Tìm & Đánh Nốt"
		"xem_video":
			return "🎬 Học Qua Video"
		_:
			return "📌 Khác"


func _ready() -> void:
	so_label.text = so
	ten_label.text = ten
	nguoi_tao_label.text = "👤 Tác giả: " + (nguoi_tao if not nguoi_tao.is_empty() else "TimeKP")
	loai_bai_label.text = lay_ten_che_do_hien_thi()

	cap_nhat_du_lieu_lich_su()

	if not mouse_exited.is_connected(_on_mouse_exited):
		mouse_exited.connect(_on_mouse_exited)


func cap_nhat_du_lieu_lich_su() -> void:
	if not bai_hoc_path.is_empty():
		var ql_lich_su = QuanLyLichSu.new()
		
		var kq: Dictionary = ql_lich_su.lay_ket_qua_bai_hoc(loai_bai_hoc, bai_hoc_path)
		
		diem_cao_nhat = int(kq.get("diem_cao_nhat", 0))
		ti_le_hoan_thanh = float(kq.get("ti_le_hoan_thanh", 0.0))
		so_lan_choi = int(kq.get("so_lan_choi", 0))
		hang_tot_nhat = str(kq.get("hang_tot_nhat", "-"))

	if ket_qua_label:
		if so_lan_choi > 0:
			if loai_bai_hoc == "xem_video":
				ket_qua_label.text = "✅ Đã xem\n🎯 %.0f%%" % ti_le_hoan_thanh
			else:
				ket_qua_label.text = "🏆 %d đ\n🎯 %.0f%% (%s)" % [diem_cao_nhat, ti_le_hoan_thanh, hang_tot_nhat]
			ket_qua_label.modulate = Color(1.0, 1.0, 1.0, 1.0)
		else:
			ket_qua_label.text = "🏆 Chưa học\n🎯 0%"
			ket_qua_label.modulate = Color(0.75, 0.85, 0.95, 0.85)


func _on_hoc_button_down() -> void:
	print("Đang mở bài: ", ten, " - Loại: ", loai_bai_hoc)
	print("Đường dẫn file data: ", bai_hoc_path)

	match loai_bai_hoc:
		"choi_sheet":
			chuyen_scene_bai_hoc(load(PATH_CHOI_SHEET), bai_hoc_path)
		"nghe_va_doan_not":
			chuyen_scene_bai_hoc(load(PATH_NGHE_NOT), bai_hoc_path)
		"tim_not":
			chuyen_scene_bai_hoc(load(PATH_TIM_NOT), bai_hoc_path)
		"xem_video":
			chuyen_scene_bai_hoc(load(PATH_VIDEO), bai_hoc_path)
		"tap_luyen_sheet", "luyen_tap_sheet":
			var scene_mau: PackedScene = load(PATH_CHOI_SHEET)
			var scene_moi: ChoiSheet = scene_mau.instantiate()
			scene_moi.set("duong_dan_bai_hoc", bai_hoc_path)
			scene_moi.phan_luyen_tap = true
			var root = get_tree().root
			var scene_cu = get_tree().current_scene
			root.add_child(scene_moi)
			get_tree().current_scene = scene_moi
			scene_cu.queue_free()
		_:
			print("Không nhận diện được loại bài học: ", loai_bai_hoc)


func chuyen_scene_bai_hoc(scene_mau: PackedScene, duong_dan: String) -> void:
	var scene_moi = scene_mau.instantiate()
	scene_moi.set("duong_dan_bai_hoc", duong_dan)
	var root = get_tree().root
	var scene_cu = get_tree().current_scene
	root.add_child(scene_moi)
	get_tree().current_scene = scene_moi
	scene_cu.queue_free()


func _on_mouse_entered() -> void:
	self.modulate = Color(1.12, 1.12, 1.15, 1.0)

	var ten_loai_bai = lay_ten_che_do_hien_thi()
	var tac_gia_hien_thi = nguoi_tao if not nguoi_tao.is_empty() else "TimeKP"
	var trang_thai_str = (
		"[color=#51cf66]Đã chơi (%d lần)[/color]" % so_lan_choi
		if so_lan_choi > 0
		else "[color=#adb5bd]Chưa từng chơi[/color]"
	)

	var bbcode_text = ""
	bbcode_text += "[center][font_size=26][b][color=#ffe066]%s[/color][/b][/font_size][/center]\n\n" % ten
	bbcode_text += "[b]• Bài số:[/b] [color=#ffffff]%s[/color]\n\n" % so
	bbcode_text += "[b]• Chế độ học:[/b]\n  [color=#66d9ff]%s[/color]\n\n" % ten_loai_bai
	bbcode_text += "[b]• Người biên soạn:[/b]\n  [color=#8ce99a]%s[/color]\n\n" % tac_gia_hien_thi
	bbcode_text += "[b]• Thành tích cá nhân:[/b]\n"
	bbcode_text += "  - Trạng thái: %s\n" % trang_thai_str
	bbcode_text += "  - Điểm cao nhất: [color=#ffd43b][b]%d điểm[/b][/color]\n" % diem_cao_nhat
	bbcode_text += "  - Hoàn thành: [color=#63e6be][b]%.1f%%[/b][/color] (Hạng: [b]%s[/b])\n\n" % [ti_le_hoan_thanh, hang_tot_nhat]
	bbcode_text += "[color=#a5d8ff][font_size=15][i]📁 Tệp dữ liệu:\n%s[/i][/font_size][/color]" % bai_hoc_path

	if thong_tin_label:
		thong_tin_label.text = bbcode_text


func _on_mouse_exited() -> void:
	self.modulate = Color(1, 1, 1, 1)
