class_name BaiHocPanel
extends Panel

var so:String
var ten: String
var loai_bai_hoc: String
var nguoi_tao: String
var bai_hoc_path: String

@onready var ten_label: Label = $ten
@onready var so_label: Label = $so
@onready var nguoi_tao_label: Label = $nguoi_tao
@onready var loai_bai_label: Label = $loai_bai
@onready var danh_sach_bai_hoc:DanhSachBaiHoc = $"../../../.."
@onready var thong_tin_label:RichTextLabel = $"../../../../menu_phai/thong_tin"


var choi_sheet_scene: PackedScene = preload("res://scene/phanchoi/ChoiSheet.tscn")


func _ready() -> void:
	so_label.text = so
	ten_label.text = ten
	nguoi_tao_label.text = nguoi_tao
	loai_bai_label.text = loai_bai_hoc
	

func _on_hoc_button_down() -> void:
	print("Đang mở bài: ", ten, " - Loại: ", loai_bai_hoc)
	print("Đường dẫn file data: ", bai_hoc_path)
	if loai_bai_hoc == "choi_sheet":
		var choi_sheet: ChoiSheet = choi_sheet_scene.instantiate()
		choi_sheet.duong_dan_bai_hoc = bai_hoc_path
		var root = get_tree().root
		var scene_cu = get_tree().current_scene
		root.add_child(choi_sheet)
		get_tree().current_scene = choi_sheet
		scene_cu.queue_free()
	elif loai_bai_hoc == "nghe_va_doan_not":
		pass
	elif loai_bai_hoc == "tim_not":
		pass
	pass

func _on_mouse_entered() -> void:
	var ten_loai_bai = ""
	match loai_bai_hoc:
		"choi_sheet":
			ten_loai_bai = "Đọc Bản Nhạc (Sheet)"
		"nghe_va_doan_not":
			ten_loai_bai = "Nghe & Đoán Nốt"
		"tim_not":
			ten_loai_bai = "Tìm và đánh nốt"
		_:
			ten_loai_bai = "Khác"

	var bbcode_text = ""
	
	# Tiêu đề: Căn giữa, chữ to, in đậm, màu vàng
	bbcode_text += "[center][font_size=24][b][color=gold]%s[/color][/b][/font_size][/center]\n\n" % ten
	
	# Thông tin chi tiết: Tiêu đề in đậm, dữ liệu có màu nhấn
	bbcode_text += "[b]Bài số:[/b] %s\n" % so
	bbcode_text += "[b]Chế độ học:[/b] [color=cyan]%s[/color]\n" % ten_loai_bai
	bbcode_text += "[b]Người tạo:[/b] [color=lightgreen]%s[/color]\n\n" % nguoi_tao
	
	# Thông tin hệ thống (Đường dẫn): Chữ nhỏ, in nghiêng, màu xám cho đỡ rối mắt
	bbcode_text += "[color=gray][font_size=14][i]Tệp dữ liệu: %s[/i][/font_size][/color]" % bai_hoc_path
	
	# Gắn chuỗi đã định dạng vào RichTextLabel
	thong_tin_label.text = bbcode_text
