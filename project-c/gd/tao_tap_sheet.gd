extends TaoTapLuyenSheet

@onready var ten_bai_input: LineEdit = $KhungChinh/VBox/HangTen/TenBai_Input
@onready var duong_dan_input: LineEdit = $KhungChinh/VBox/HangSheet/DuongDanSheet_Input
@onready var thong_tin_sheet_lbl: Label = $KhungChinh/VBox/ThongTinSheet_Label
@onready var khuong_opt: OptionButton = $KhungChinh/VBox/HangCauHinh/Khuong_Option
@onready var bpm_spin: SpinBox = $KhungChinh/VBox/HangCauHinh/Bpm_Spin
@onready var cho_phep_cho_chk: CheckButton = $KhungChinh/VBox/HangCauHinh/ChoPhepCho_Check
@onready var nhip_bd_spin: SpinBox = $KhungChinh/VBox/HangCauHinh/NhipBatDau_Spin
@onready var nhip_kt_spin: SpinBox = $KhungChinh/VBox/HangCauHinh/NhipKetThuc_Spin


func _on_duong_dan_submitted(new_text: String) -> void:
	set_duong_dan_sheet(new_text)
	cap_nhat_thong_tin_giao_dien()

func cap_nhat_thong_tin_giao_dien() -> void:
	if ten_bai_input:
		ten_bai_input.text = get_ten_bai_hoc()
		
	if duong_dan_input:
		duong_dan_input.text = get_duong_dan_sheet()

	if khuong_opt:
		match get_khuong():
			"1": khuong_opt.select(1)
			"2": khuong_opt.select(2)
			_: khuong_opt.select(0)

	if cho_phep_cho_chk:
		cho_phep_cho_chk.button_pressed = get_cho_phep_cho()

	if bpm_spin:
		bpm_spin.value = get_bpm_tuy_chinh()

	var phan_tg: PackedStringArray = get_phan_thoi_gian().split("-")
	if phan_tg.size() == 2:
		if nhip_bd_spin:
			nhip_bd_spin.value = float(phan_tg[0])
		if nhip_kt_spin:
			nhip_kt_spin.value = float(phan_tg[1])

	cap_nhat_ui_ghi_chu()
