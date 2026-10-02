extends Control

@onready var ban_phim_piano: BanPhimMidi = $BanPhimPiano

@export var hien_thi_c: bool
@export var hien_thi_all: bool

func _ready() -> void:
	ban_phim_piano.hien_thi_ten_not_c = hien_thi_c
	ban_phim_piano.hien_thi_ten_not_all = hien_thi_all
	ban_phim_piano.hien_thi_ten_not()
	pass # Replace with function body.
