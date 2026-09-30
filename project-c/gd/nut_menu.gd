extends Control

@onready var menu: Control = $Menu

var dang_mo: bool = false
var vt_ban_dau: Vector2 = Vector2(0.0, -80.0)
var vt_dang_mo: Vector2 = Vector2(0.0, -5.0)

func _ready() -> void:
	menu.position = vt_ban_dau

func _on_nut_menu_button_down() -> void:
	var tween = create_tween()
	if not dang_mo:
		tween.tween_property(menu, "position", vt_dang_mo, 0.5)\
			.set_trans(Tween.TRANS_ELASTIC)\
			.set_ease(Tween.EASE_OUT)
		dang_mo = true
	else:
		tween.tween_property(menu, "position", vt_ban_dau, 0.3)\
			.set_trans(Tween.TRANS_CUBIC)\
			.set_ease(Tween.EASE_IN)
		dang_mo = false
