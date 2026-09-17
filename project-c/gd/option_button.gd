extends OptionButton

@onready var ban_phim_piano: BanPhimMidi = $"../BanPhimPiano"

var dang_di_chuyen:bool = false
var dang_giu_chuot:bool = false
var drag_offset: Vector2i

var file_sf2:Array[String] = []
# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	
	var dir = DirAccess.open("res://sf2")
	if dir:
		dir.list_dir_begin()
		var file_name = dir.get_next()
		while file_name != "":
			if not dir.current_is_dir():
				file_sf2.append(file_name)
			file_name = dir.get_next()
	pass # Replace with function body.

func _on_button_down() -> void:
	self.clear()
	file_sf2.clear()
	
	var dir = DirAccess.open("res://sf2")
	if dir:
		dir.list_dir_begin()
		var file_name = dir.get_next()
		while file_name != "":
			if not dir.current_is_dir():
				file_sf2.append(file_name)
			file_name = dir.get_next()
			
	for i in file_sf2:
		self.add_item(i)
	print(file_sf2)


func _on_item_selected(index: int) -> void:
	var file:String = "res://sf2/" + get_item_text(index)
	ban_phim_piano.thay_doi_sf2(file)
	pass # Replace with function body.


func _on_button_button_down() -> void:
	ban_phim_piano.kiem_tra_dau_vao()
	pass # Replace with function body.
