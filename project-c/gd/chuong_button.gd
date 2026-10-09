class_name ChuongButton

extends Button
@onready var danh_sach_bai_hoc: DanhSachBaiHoc = $"../../../.."

var ten_chuong: String
var ten_file: String

func _ready() -> void:
	self.text = ten_chuong
	pass

func _on_button_down() -> void:
	print(ten_file)
	if self.name != "all":
		danh_sach_bai_hoc.call("cat_nhat_danh_sach_bai_hoc", ten_file)
	else:
		danh_sach_bai_hoc.call("lay_tat_ca_bai_hoc")
	pass # Replace with function body.
