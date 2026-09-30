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


func _ready() -> void:
	so_label.text = so
	ten_label.text = ten
	nguoi_tao_label.text = nguoi_tao
	loai_bai_label.text = loai_bai_hoc
	

func _on_hoc_button_down() -> void:
	print("Đang mở bài: ", ten, " - Loại: ", loai_bai_hoc)
	print("Đường dẫn file data: ", bai_hoc_path)
	if loai_bai_hoc == "choi_sheet":
		pass
	elif loai_bai_hoc == "nghe_va_doan_not":
		pass
	elif loai_bai_hoc == "tim_not":
		pass
	pass

func _on_mouse_entered() -> void:
	print(name)
	pass 
