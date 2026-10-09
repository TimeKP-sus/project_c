extends HocVideo

@onready var video: VideoStreamPlayer = $video
@onready var thanh_tua: HSlider = $panel/thanh_tua
@onready var thoi_gian: Label = $panel/thanh_tua/thoi_gian

var dang_keo_thanh: bool = false
# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	if self.get_duong_dan_video() != null:
		self.duong_dan_video = "res://bai_hoc/video/v1.mp4"
		video.stream.file = self.get_duong_dan_video()
		print("duong_dan: " + self.get_duong_dan_video())
		video.play()
	thanh_tua.max_value = video.get_stream_length()
	print(video.get_stream_length())
	pass # Replace with function body.

func _process(_delta: float) -> void:
	if video.is_playing() and not dang_keo_thanh:
		thanh_tua.value = video.stream_position
		
func _on_thanh_tua_drag_started() -> void:
	dang_keo_thanh = true
	pass # Replace with function body.


func _on_thanh_tua_drag_ended(value_changed: bool) -> void:
	dang_keo_thanh = false
	if value_changed:
		video.stream_position = thanh_tua.value
	pass # Replace with function body.
