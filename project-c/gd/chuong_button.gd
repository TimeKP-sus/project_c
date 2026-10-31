class_name ChuongButton
extends Button

@onready var danh_sach_bai_hoc: DanhSachBaiHoc = $"../../../.."

var ten_chuong: String = ""
var ten_file: String = ""

func _ready() -> void:
	self.text = ten_chuong

func _on_button_down() -> void:
	if self.name != "all":
		danh_sach_bai_hoc.call("cat_nhat_danh_sach_bai_hoc", ten_file)
		danh_sach_bai_hoc.call("danh_dau_chuong_dang_chon", ten_file)
	else:
		danh_sach_bai_hoc.call("lay_tat_ca_bai_hoc")
		danh_sach_bai_hoc.call("danh_dau_chuong_dang_chon", "all")
