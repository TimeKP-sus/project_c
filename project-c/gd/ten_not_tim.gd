extends RichTextLabel

@export var chuoi_id: String
@export var ten_not: String

func _ready() -> void:
	hien_thi_ban_dau()
	if "_" in chuoi_id:
		thiet_lap_vien_hop_am()
		
	self.scale = Vector2.ZERO
	# Tăng thời gian lên 0.35s để nốt nhạc nở ra mềm mại hơn
	var tween = create_tween()
	tween.tween_property(self, "scale", Vector2.ONE, 0.35).set_trans(Tween.TRANS_BACK).set_ease(Tween.EASE_OUT)

func hien_thi_ban_dau() -> void:
	self.text = ten_not
	self.modulate = Color.WHITE
	self.add_theme_color_override("default_color", Color.WHITE)

func hieu_ung_sai() -> void:
	self.add_theme_color_override("default_color", Color.RED)
	var tween = create_tween()
	var x_goc = self.position.x
	# Rung lắc mượt dần đều (biên độ giảm dần 5 -> -3 -> 0)
	tween.tween_property(self, "position:x", x_goc + 5, 0.05).set_trans(Tween.TRANS_SINE)
	tween.tween_property(self, "position:x", x_goc - 3, 0.05).set_trans(Tween.TRANS_SINE)
	tween.tween_property(self, "position:x", x_goc, 0.05).set_trans(Tween.TRANS_SINE)

func hien_not_dung() -> void:
	self.add_theme_color_override("default_color", Color.GREEN)
	
	var tween = create_tween()
	tween.tween_property(self, "scale", Vector2(1.25, 1.25), 0.1).set_trans(Tween.TRANS_SINE).set_ease(Tween.EASE_OUT)
	tween.tween_property(self, "scale", Vector2.ONE, 0.25).set_trans(Tween.TRANS_BACK).set_ease(Tween.EASE_OUT)

func thiet_lap_vien_hop_am() -> void:
	self.add_theme_color_override("font_outline_color", Color.DEEP_SKY_BLUE)
