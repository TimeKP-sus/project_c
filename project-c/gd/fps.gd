extends Label

@export var hien_fps:bool


func _process(_delta: float) -> void:
	if hien_fps == true:
		self.text = "FPS: " + str(Engine.get_frames_per_second())
	pass
