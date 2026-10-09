extends RichTextLabel

# Đã sửa int thành String để nhận chuỗi chứa nhiều nốt (VD: "36_40_43")
@export var chuoi_id: String
@export var ten_not: String

func _ready() -> void:
	an_not()
	if "_" in chuoi_id:
		thiet_lap_vien_hop_am()
	self.scale = Vector2.ZERO
	var tween = create_tween()
	tween.tween_property(self, "scale", Vector2.ONE, 0.2).set_trans(Tween.TRANS_BACK).set_ease(Tween.EASE_OUT)

func an_not() -> void:
	self.text = "[center]?[/center]"
	self.modulate = Color.WHITE # Đảm bảo Node không bị ám màu
	self.add_theme_color_override("default_color", Color.WHITE) # Chữ màu trắng

func hieu_ung_sai() -> void:
	# Chỉ đổi màu chữ thành Đỏ, không ảnh hưởng đến viền
	self.add_theme_color_override("default_color", Color.RED)
	
	# Hiệu ứng rung lắc trái phải (Giữ nguyên)
	var tween = create_tween()
	var x_goc = self.position.x
	tween.tween_property(self, "position:x", x_goc + 5, 0.05)
	tween.tween_property(self, "position:x", x_goc - 5, 0.05)
	tween.tween_property(self, "position:x", x_goc, 0.05)

func hien_not_dung() -> void:
	# Chỉ đổi màu chữ thành Xanh lá, viền Vàng Đồng của hợp âm vẫn sẽ rực rỡ
	self.add_theme_color_override("default_color", Color.GREEN)
	self.text = ten_not

	self.scale = Vector2(1.5, 1.5)
	var tween = create_tween()
	tween.tween_property(self, "scale", Vector2.ONE, 0.2).set_trans(Tween.TRANS_BOUNCE)

func thiet_lap_vien_hop_am() -> void:
	# Đổi màu viền chữ thành Vàng Đồng (hoặc màu bất kỳ bạn thích)
	self.add_theme_color_override("font_outline_color", Color.DARK_BLUE)
	#self.add_theme_color_override("default_color", Color.BLUE)
