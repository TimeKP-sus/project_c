extends Control

@onready var menu: Control = $Menu

var dang_mo: bool = false
var vt_ban_dau: Vector2 = Vector2(0.0, -80.0)
var vt_dang_mo: Vector2 = Vector2(0.0, -5.0)

func _ready() -> void:
	menu.position = vt_ban_dau
	var nut_bat_dau = get_node_or_null("Menu/bat_dau")
	if nut_bat_dau and nut_bat_dau.get("path_scene_den") == null:
		nut_bat_dau.set("path_scene_den", load("res://scene/menu/MenuChinh.tscn"))
		
	var nut_quay_lai = get_node_or_null("Menu/quay_lai")
	if nut_quay_lai and nut_quay_lai.get("path_scene_den") == null:
		nut_quay_lai.set("path_scene_den", load("res://scene/menu/DanhSachBaiHoc.tscn"))
		nut_quay_lai.set("loai_nut", "chuyen scene")
	if self.get_parent().name == "ChoiSheet":
		if self.get_parent().phan_luyen_tap == false:
			get_node_or_null("Menu/bat_tat_nhip").visible = false
			get_node_or_null("Menu/che_do_cho").visible = false

func _on_nut_menu_button_down() -> void:
	var tween = create_tween()
	if not dang_mo:
		if get_node_or_null("../QuanLySheet") != null:
			get_node("../QuanLySheet").da_dung = true
		tween.tween_property(menu, "position", vt_dang_mo, 0.5)\
			.set_trans(Tween.TRANS_ELASTIC)\
			.set_ease(Tween.EASE_OUT)
		dang_mo = true
	else:
		if get_node_or_null("../QuanLySheet") != null:
			get_node("../QuanLySheet").da_dung = false
		tween.tween_property(menu, "position", vt_ban_dau, 0.3)\
			.set_trans(Tween.TRANS_CUBIC)\
			.set_ease(Tween.EASE_IN)
		dang_mo = false
