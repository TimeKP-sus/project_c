extends RichTextLabel

@export var chuoi_id: String
@export var ten_not: String

func _ready() -> void:
	hien_thi_ban_dau()
	if "_" in chuoi_id:
		thiet_lap_vien_hop_am()
	self.scale = Vector2.ZERO
	var tween = create_tween()
	tween.tween_property(self, "scale", Vector2.ONE, 0.2).set_trans(Tween.TRANS_BACK).set_ease(Tween.EASE_OUT)

func hien_thi_ban_dau() -> void:
	# Khác biệt so với Nghe nốt: Ở bài Tìm nốt, tên nốt hiện sẵn để người dùng nhìn và tìm
	self.text = ten_not
	self.modulate = Color.WHITE
	self.add_theme_color_override("default_color", Color.WHITE)

func hieu_ung_sai() -> void:
	self.add_theme_color_override("default_color", Color.RED)
	var tween = create_tween()
	var x_goc = self.position.x
	tween.tween_property(self, "position:x", x_goc + 5, 0.05)
	tween.tween_property(self, "position:x", x_goc - 5, 0.05)
	tween.tween_property(self, "position:x", x_goc, 0.05)

func hien_not_dung() -> void:
	self.add_theme_color_override("default_color", Color.GREEN)
	self.scale = Vector2(1.5, 1.5)
	var tween = create_tween()
	tween.tween_property(self, "scale", Vector2.ONE, 0.2).set_trans(Tween.TRANS_BOUNCE)

func thiet_lap_vien_hop_am() -> void:
	self.add_theme_color_override("font_outline_color", Color.DARK_BLUE)
