extends Control

@onready var ban_phim_piano: BanPhimMidi = $BanPhimPiano
@onready var thanh_chon_nhac_cu: OptionButton = $OptionButton
@onready var pedal: Label = $pedal
@onready var midi: Label = $midi
@onready var button: Button = $Button
@onready var cac_phim: Label = $cac_phim
@onready var hop_am: Label = $hop_am


@export var hien_thi_c: bool
@export var hien_thi_all: bool = false

@export var hien_thi_midi:bool = true
@export var chon_nhac_cu: bool
@export var hien_ten_midi: bool


func _ready() -> void:
	ban_phim_piano.hien_thi_ten_not_c = hien_thi_c
	ban_phim_piano.hien_thi_ten_not_all = hien_thi_all
	ban_phim_piano.hien_thi_ten_not()
	thanh_chon_nhac_cu.visible = chon_nhac_cu
	midi.visible = hien_ten_midi
	pass # Replace with function body.


func _on_an_hien_button_down() -> void:
	if hien_thi_midi == true:
		ban_phim_piano.visible = false
		thanh_chon_nhac_cu.visible = false
		pedal.visible = false
		midi.visible = false
		button.visible = false
		cac_phim.visible = false
		hop_am.visible = false
		hien_thi_midi = false
	else:
		ban_phim_piano.visible = true
		thanh_chon_nhac_cu.visible = true
		pedal.visible = true
		midi.visible = true
		button.visible = true
		cac_phim.visible = true
		hop_am.visible = true
		hien_thi_midi = true
	pass
