extends Button

@export var path_scene_den: PackedScene
@export_enum("chuyen scene", "them moi", "them moi xoa cu") var loai_nut: String

func _on_button_down() -> void:
	if path_scene_den == null:
		print("khong co gi")
		return
	if loai_nut == "chuyen scene":
		get_tree().change_scene_to_packed(path_scene_den)
	elif loai_nut == "them moi":
		var node_moi = path_scene_den.instantiate()
		get_tree().current_scene.add_child(node_moi)
	elif loai_nut == "them moi xoa cu":
		var node_moi = path_scene_den.instantiate()
		get_tree().current_scene.add_child(node_moi)
		if owner != null:
			owner.queue_free()
		else:
			get_parent().queue_free()
