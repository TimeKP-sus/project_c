extends Control

@onready var nhan_dien: Area2D = $nhan_dien

var cac_not_trong_area: Array[Sprite2D] = []
var cac_not_da_bam_trung: Array[Sprite2D] = []

func _ready() -> void:
	pass

func _input(event: InputEvent) -> void:
	# Khi người chơi bấm phím tương ứng (ví dụ: Space)
	if event.is_action_pressed("ui_accept"):
		if cac_not_trong_area.size() > 0:
			# 1. Lấy nốt đầu tiên trong danh sách chờ (nốt bay vào sớm nhất)
			var not_muc_tieu: Sprite2D = cac_not_trong_area[0]
			
			var khoang_cach = abs(nhan_dien.position.x - not_muc_tieu.position.x)
			print(khoang_cach)
			
			# 2. Xóa nốt khỏi danh sách đang chờ (để không bị bấm trùng)
			cac_not_trong_area.pop_front()
			
			# 3. Thêm vào danh sách đã bấm trúng (nếu bạn cần quản lý lịch sử bấm)
			cac_not_da_bam_trung.append(not_muc_tieu)
			
			# 4. Ẩn nốt hoặc xóa nốt khỏi bộ nhớ
			not_muc_tieu.queue_free()
			
			print("Đã bấm trúng! Tổng nốt trúng: ", cac_not_da_bam_trung.size())
		else:
			# Người chơi bấm nhưng không có nốt nào trong vùng
			print("Bấm hụt (Miss/Bad)!")

func _on_nhan_dien_area_entered(area: Area2D) -> void:
	var not_bay_vao = area.get_parent() as Sprite2D
	
	if not_bay_vao:
		cac_not_trong_area.append(not_bay_vao)
		print("Nốt vào vùng: ", cac_not_trong_area.size())

func _on_nhan_dien_area_exited(area: Area2D) -> void:
	var node_thoat_ra = area.get_parent() as Sprite2D

	# Kiểm tra xem nốt thoát ra có còn trong mảng không 
	# (vì nếu bấm trúng ở _input, nó đã bị xóa khỏi mảng rồi)
	var index = cac_not_trong_area.find(node_thoat_ra)
	
	if index != -1:
		cac_not_trong_area.remove_at(index)
		print("Nốt trượt qua vùng (Miss): ", cac_not_trong_area.size())
		
		# Nếu nốt chưa bị xóa (bấm hụt), giải phóng bộ nhớ
		if is_instance_valid(node_thoat_ra):
			node_thoat_ra.queue_free()
