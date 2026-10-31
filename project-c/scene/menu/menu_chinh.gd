extends Control

@onready var panel_chon_loai_bai_hoc: Control = $PanelChonLoaiBaiHoc

var tao_sheet_scene: PackedScene = load("res://scene/phantao/TaoSheet.tscn")
var tao_tap_luyen_scene: PackedScene = load("res://scene/phantao/TaoTapSheet.tscn")
var tao_nghe_not_scene: PackedScene = load("res://scene/phantao/TaoNgheNot.tscn")
var tao_tim_not_scene: PackedScene = load("res://scene/phantao/TaoTimNot.tscn")
var tao_video_scene: PackedScene = load("res://scene/phantao/TaoVideo.tscn")
var tao_danh_sach_scene: PackedScene = load("res://scene/phantao/TaoDanhSach.tscn")

func chuyen_scene(scene: PackedScene):
	get_tree().change_scene_to_packed(scene)

func _on_btn_tao_sheet_goc_button_down() -> void:
	chuyen_scene(tao_sheet_scene)
	pass # Replace with function body.


func _on_btn_tao_tap_luyen_button_down() -> void:
	chuyen_scene(tao_tap_luyen_scene)
	pass # Replace with function body.


func _on_btn_tao_nghe_not_button_down() -> void:
	chuyen_scene(tao_nghe_not_scene)
	pass # Replace with function body.


func _on_btn_tao_tim_not_button_down() -> void:
	chuyen_scene(tao_tim_not_scene)
	pass # Replace with function body.


func _on_btn_tao_video_button_down() -> void:
	chuyen_scene(tao_video_scene)
	pass # Replace with function body.


func _on_btn_tao_danh_sach_button_down() -> void:
	chuyen_scene(tao_danh_sach_scene)
	pass # Replace with function body.


func _on_btn_dong_panel_button_down() -> void:	
	panel_chon_loai_bai_hoc.visible = false
	pass # Replace with function body.


func _on_tao_bai_hoc_button_down() -> void:
	panel_chon_loai_bai_hoc.visible = true
	pass # Replace with function body.
