extends Control

@onready var ban_phim_piano: BanPhimMidi = $BanPhimPiano
@onready var thanh_chon_nhac_cu: OptionButton = $OptionButton
@onready var pedal: Label = $pedal
@onready var midi: Label = $midi
@onready var button: Button = $Button
@onready var cac_phim: Label = $cac_phim
@onready var hop_am: Label = $hop_am
@onready var an_hien: Button = $an_hien
@onready var khung_nen_dan: Panel = $KhungNenDan
@onready var nut_cai_dat: Button = $nut_cai_dat
@onready var panel_cai_dat: Panel = $PanelCaiDatDan

# Các công tắc trong PanelCaiDatDan
@onready var check_hien_c: CheckButton = $PanelCaiDatDan/VBox/CheckHienNotC
@onready var check_hien_all: CheckButton = $PanelCaiDatDan/VBox/CheckHienTatCaNot
@onready var check_hop_am: CheckButton = $PanelCaiDatDan/VBox/CheckKiemTraHopAm
@onready var check_phim_bam: CheckButton = $PanelCaiDatDan/VBox/CheckHienPhimBam
@onready var check_nhac_cu: CheckButton = $PanelCaiDatDan/VBox/CheckChonNhacCu
@onready var dong_panel_btn: Button = $PanelCaiDatDan/VBox/HangTieuDe/DongPanel_Btn

@export var hien_thi_c: bool = true
@export var hien_thi_all: bool = false
@export var hien_thi_midi: bool = true
@export var chon_nhac_cu: bool = true
@export var hien_ten_midi: bool = true


func _ready() -> void:
	ban_phim_piano.hien_thi_ten_not_c = hien_thi_c
	ban_phim_piano.hien_thi_ten_not_all = hien_thi_all
	ban_phim_piano.hien_thi_ten_not()

	thanh_chon_nhac_cu.visible = chon_nhac_cu and hien_thi_midi
	midi.visible = hien_ten_midi and hien_thi_midi

	# Đồng bộ trạng thái lên các nút gạt trong PanelCaiDatDan
	if check_hien_c:
		check_hien_c.set_pressed_no_signal(ban_phim_piano.hien_thi_ten_not_c)
		check_hien_c.toggled.connect(_khi_bat_tat_not_c)
	if check_hien_all:
		check_hien_all.set_pressed_no_signal(ban_phim_piano.hien_thi_ten_not_all)
		check_hien_all.toggled.connect(_khi_bat_tat_tat_ca_not)
	if check_hop_am:
		check_hop_am.set_pressed_no_signal(ban_phim_piano.cho_phep_kiem_tra_hop_am)
		check_hop_am.toggled.connect(_khi_bat_tat_hop_am)
	if check_phim_bam:
		check_phim_bam.set_pressed_no_signal(ban_phim_piano.cho_phep_hien_thi_phim)
		check_phim_bam.toggled.connect(_khi_bat_tat_hien_phim_bam)
	if check_nhac_cu:
		check_nhac_cu.set_pressed_no_signal(chon_nhac_cu)
		check_nhac_cu.toggled.connect(_khi_bat_tat_chon_nhac_cu)

	if nut_cai_dat:
		nut_cai_dat.pressed.connect(_khi_bam_nut_cai_dat)
	if dong_panel_btn:
		dong_panel_btn.pressed.connect(func(): panel_cai_dat.visible = false)
	if button:
		button.pressed.connect(func(): ban_phim_piano.kiem_tra_dau_vao())

	# Áp dụng trạng thái ẩn/hiện ban đầu (ví dụ ở màn hình HocVideo khi hien_thi_midi = false)
	_cap_nhat_trang_thai_an_hien()


func _khi_bam_nut_cai_dat() -> void:
	if panel_cai_dat:
		panel_cai_dat.visible = not panel_cai_dat.visible


func _khi_bat_tat_not_c(bat: bool) -> void:
	hien_thi_c = bat
	ban_phim_piano.hien_thi_ten_not_c = bat
	ban_phim_piano.hien_thi_ten_not()


func _khi_bat_tat_tat_ca_not(bat: bool) -> void:
	hien_thi_all = bat
	ban_phim_piano.hien_thi_ten_not_all = bat
	ban_phim_piano.hien_thi_ten_not()


func _khi_bat_tat_hop_am(bat: bool) -> void:
	ban_phim_piano.cho_phep_kiem_tra_hop_am = bat
	hop_am.visible = bat and hien_thi_midi
	if not bat:
		hop_am.text = ""


func _khi_bat_tat_hien_phim_bam(bat: bool) -> void:
	ban_phim_piano.cho_phep_hien_thi_phim = bat
	cac_phim.visible = bat and hien_thi_midi
	if not bat:
		cac_phim.text = ""


func _khi_bat_tat_chon_nhac_cu(bat: bool) -> void:
	chon_nhac_cu = bat
	thanh_chon_nhac_cu.visible = bat and hien_thi_midi


func _cap_nhat_trang_thai_an_hien() -> void:
	an_hien.text = "▼" if hien_thi_midi else "▲"
	if khung_nen_dan:
		khung_nen_dan.visible = hien_thi_midi
	ban_phim_piano.visible = hien_thi_midi
	thanh_chon_nhac_cu.visible = chon_nhac_cu and hien_thi_midi
	pedal.visible = hien_thi_midi
	midi.visible = hien_ten_midi and hien_thi_midi
	button.visible = hien_thi_midi
	cac_phim.visible = ban_phim_piano.cho_phep_hien_thi_phim and hien_thi_midi
	hop_am.visible = ban_phim_piano.cho_phep_kiem_tra_hop_am and hien_thi_midi
	if nut_cai_dat:
		nut_cai_dat.visible = hien_thi_midi
	if not hien_thi_midi and panel_cai_dat:
		panel_cai_dat.visible = false


func _on_an_hien_button_down() -> void:
	hien_thi_midi = not hien_thi_midi
	_cap_nhat_trang_thai_an_hien()
