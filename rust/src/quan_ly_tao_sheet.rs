use godot::builtin::{Color, Rect2, Transform2D, Vector2};
use godot::classes::{
    Control, INode2D, InputEvent, InputEventMouseButton, InputEventMouseMotion, Node2D,
    PhysicsPointQueryParameters2D, PhysicsShapeQueryParameters2D, RectangleShape2D, Window,
};
use godot::global::MouseButton;
use godot::obj::{Base, Gd, NewGd, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

use crate::not_nhac::NotNhac;

#[derive(Clone, Debug)]
pub struct ThongTinNotChon {
    pub la_khuong_1: bool,
    pub id_not: i32,
    pub nhip_dich: f32,
    pub nhip_giu: f32,
    pub mau_not: String,
    pub chuyen_not: i32,
    pub ten_khac: String,
}

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct QuanLyTaoSheet {
    #[base]
    base: Base<Node2D>,

    mang_da_chon: Vec<Gd<NotNhac>>,
    dang_keo_chon: bool,
    diem_bat_dau_keo: Vector2,
    diem_hien_tai_keo: Vector2,
}

#[godot_api]
impl QuanLyTaoSheet {
    #[signal]
    fn thay_doi_chon_not();

    /// Tạo hình chữ nhật chuẩn (kể cả khi kéo ngược từ phải sang trái hoặc dưới lên trên)
    fn tao_rect_tu_2_diem(p1: Vector2, p2: Vector2) -> Rect2 {
        let min_x: f32 = p1.x.min(p2.x);
        let min_y: f32 = p1.y.min(p2.y);
        let max_x: f32 = p1.x.max(p2.x);
        let max_y: f32 = p1.y.max(p2.y);
        Rect2::new(
            Vector2::new(min_x, min_y),
            Vector2::new(max_x - min_x, max_y - min_y),
        )
    }

    /// Kiểm tra xem chuột có đang bấm vào các bảng UI (Thanh công cụ, PanelSuaNot, FileDialog) không
    /// Kiểm tra chính xác xem chuột có đang bấm vào các bảng UI hay không
    fn chuot_nam_trong_ui(&self, vi_tri_chuot: Vector2) -> bool {
        for path in ["../ThanhThongTin", "../ThanhCongCuSoan", "../PanelSuaNot", "../MIDI"] {
            if let Some(panel) = self.base().try_get_node_as::<Control>(path) {
                if panel.is_visible_in_tree() && panel.get_global_rect().has_point(vi_tri_chuot) {
                    return true;
                }
            }
        }

        if vi_tri_chuot.y < 96.0 || vi_tri_chuot.y > 870.0 {
            return true;
        }

        // Kiểm tra các hộp thoại FileDialog
        for dialog_path in ["../HopThoaiLuuFile", "../HopThoaiMoFile", "../HopThoaiNhapMid"] {
            if let Some(dialog) = self.base().try_get_node_as::<Window>(dialog_path) {
                if dialog.is_visible() {
                    return true;
                }
            }
        }

        false
    }

    /// Chọn hoặc hủy chọn 1 nốt khi click chuột đơn
    pub fn chon_hoac_huy_not(&mut self, mut not_nhac: Gd<NotNhac>) {
        self.mang_da_chon.retain(|n| n.is_instance_valid());

        let id_moi = not_nhac.bind().get_id_not();
        let la_vach_100 = id_moi == 100;

        // Nếu đang chọn nốt thường mà click sang vạch chuyển miền (hoặc ngược lại) -> Xóa nhóm cũ trước
        if let Some(not_dau) = self.mang_da_chon.first() {
            let dang_chon_vach_100 = not_dau.bind().get_id_not() == 100;
            if dang_chon_vach_100 != la_vach_100 {
                self.lam_trong_mang_da_chon_noi_bo();
            }
        }

        if let Some(vi_tri) = self.mang_da_chon.iter().position(|n| n == &not_nhac) {
            self.mang_da_chon.swap_remove(vi_tri);
            not_nhac.bind_mut().set_duoc_chon(false);
        } else {
            not_nhac.bind_mut().set_duoc_chon(true);
            self.mang_da_chon.push(not_nhac);
        }

        self.base_mut().emit_signal("thay_doi_chon_not", &[]);
    }

    /// Xóa trắng mảng đã chọn (không phát signal để tránh lỗi borrow khi TaoSheet gọi trực tiếp)
    pub fn lam_trong_mang_da_chon_noi_bo(&mut self) {
        for mut not in self.mang_da_chon.drain(..) {
            if not.is_instance_valid() {
                not.bind_mut().set_duoc_chon(false);
            }
        }
    }

    #[func]
    pub fn lam_trong_mang_da_chon(&mut self) {
        self.lam_trong_mang_da_chon_noi_bo();
        self.base_mut().emit_signal("thay_doi_chon_not", &[]);
    }

    /// Lấy danh sách thông tin chi tiết của tất cả các nốt/vạch đang được chọn
    pub fn lay_thong_tin_da_chon(&mut self) -> Vec<ThongTinNotChon> {
        self.mang_da_chon.retain(|n| n.is_instance_valid());
        let mut ds = Vec::new();

        for not_node in &self.mang_da_chon {
            let bind = not_node.bind();
            let la_khuong_1 = not_node
                .get_parent()
                .map(|p| p.get_name().to_string() == "K1")
                .unwrap_or(true);

            ds.push(ThongTinNotChon {
                la_khuong_1,
                id_not: bind.get_id_not(),
                nhip_dich: bind.get_nhip_dich(),
                nhip_giu: bind.get_nhip_giu_goc(),
                mau_not: bind.get_mau_not(),
                chuyen_not: bind.get_chuyen_not(),
                ten_khac: bind.get_ten_khac().to_string(),
            });
        }
        ds
    }

    /// Trả về danh sách các nốt cần xóa: Vec<(la_khuong_1, id_not, nhip_dich)>
    pub fn lay_va_xoa_not_da_chon(&mut self) -> Vec<(bool, i32, f32)> {
        let mut ds_can_xoa = Vec::new();

        for mut not_node in self.mang_da_chon.drain(..) {
            if not_node.is_instance_valid() {
                let id_not = not_node.bind().get_id_not();
                let nhip_dich = not_node.bind().get_nhip_dich();

                let la_khuong_1 = not_node
                    .get_parent()
                    .map(|p| p.get_name().to_string() == "K1")
                    .unwrap_or(true);

                ds_can_xoa.push((la_khuong_1, id_not, nhip_dich));
                not_node.queue_free();
            }
        }

        ds_can_xoa
    }

    /// Xử lý click đơn tại 1 điểm
    fn xu_ly_click_diem(&mut self, vi_tri_chuot: Vector2) {
        let Some(mut world_2d) = self.base().get_world_2d() else {
            return;
        };
        let Some(mut space_state) = world_2d.get_direct_space_state() else {
            return;
        };

        let mut query = PhysicsPointQueryParameters2D::new_gd();
        query.set_position(vi_tri_chuot);
        query.set_collide_with_areas(true);
        query.set_collide_with_bodies(false);

        let ket_qua = space_state.intersect_point(&query);

        let mut not_thuong_tim_thay: Option<Gd<NotNhac>> = None;
        let mut vach_100_tim_thay: Option<Gd<NotNhac>> = None;

        for dict in ket_qua.iter_shared() {
            if let Some(collider_var) = dict.get("collider") {
                if let Ok(not_nhac) = collider_var.try_to::<Gd<NotNhac>>() {
                    if not_nhac.bind().get_id_not() == 100 {
                        if vach_100_tim_thay.is_none() {
                            vach_100_tim_thay = Some(not_nhac);
                        }
                    } else {
                        not_thuong_tim_thay = Some(not_nhac);
                        break; // Ưu tiên nốt thường nếu click trúng cả hai
                    }
                }
            }
        }

        if let Some(not) = not_thuong_tim_thay.or(vach_100_tim_thay) {
            self.chon_hoac_huy_not(not);
        } else {
            // Click vào khoảng trống trên khuông -> Bỏ chọn tất cả
            if !self.mang_da_chon.is_empty() {
                self.lam_trong_mang_da_chon();
            }
        }
    }

    /// Xử lý khoanh vùng chọn nhiều nốt bằng hình chữ nhật
    fn xu_ly_khoanh_vung_chon(&mut self, rect: Rect2) {
        let Some(mut world_2d) = self.base().get_world_2d() else {
            return;
        };
        let Some(mut space_state) = world_2d.get_direct_space_state() else {
            return;
        };

        let mut shape = RectangleShape2D::new_gd();
        shape.set_size(Vector2::new(
            rect.size.x.max(2.0),
            rect.size.y.max(2.0),
        ));

        let tam: Vector2 = rect.position + rect.size * 0.5;
        let mut query = PhysicsShapeQueryParameters2D::new_gd();
        query.set_shape(&shape);
        query.set_transform(Transform2D::from_angle_origin(0.0, tam));
        query.set_collide_with_areas(true);
        query.set_collide_with_bodies(false);

        let ket_qua = space_state.intersect_shape_ex(&query).max_results(256).done();

        let mut ds_not_thuong: Vec<Gd<NotNhac>> = Vec::new();
        let mut ds_vach_100: Vec<Gd<NotNhac>> = Vec::new();

        for dict in ket_qua.iter_shared() {
            if let Some(collider_var) = dict.get("collider") {
                if let Ok(not_nhac) = collider_var.try_to::<Gd<NotNhac>>() {
                    if not_nhac.bind().get_id_not() == 100 {
                        if !ds_vach_100.contains(&not_nhac) {
                            ds_vach_100.push(not_nhac);
                        }
                    } else if !ds_not_thuong.contains(&not_nhac) {
                        ds_not_thuong.push(not_nhac);
                    }
                }
            }
        }

        self.lam_trong_mang_da_chon_noi_bo();

        // Nếu trong vùng khoanh có nốt thường thì chọn các nốt thường, nếu chỉ có vạch chuyển miền thì chọn vạch
        let danh_sach_chon = if !ds_not_thuong.is_empty() {
            ds_not_thuong
        } else {
            ds_vach_100
        };

        for mut not in danh_sach_chon {
            not.bind_mut().set_duoc_chon(true);
            self.mang_da_chon.push(not);
        }

        self.base_mut().emit_signal("thay_doi_chon_not", &[]);
    }
}

#[godot_api]
impl INode2D for QuanLyTaoSheet {
    fn ready(&mut self) {
        self.base_mut().set_process_input(true);
        self.base_mut().set_z_index(20);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        // 1. Khi nhấn hoặc nhả chuột trái
        if let Ok(mouse_btn) = event.clone().try_cast::<InputEventMouseButton>() {
            if mouse_btn.get_button_index() == MouseButton::LEFT {
                let vi_tri_chuot: Vector2 = self.base().get_global_mouse_position();

                if mouse_btn.is_pressed() {
                    // Chỉ bắt đầu chọn nếu click bên trong vùng khuông nhạc và không đè lên Panel UI
                    if !self.chuot_nam_trong_ui(vi_tri_chuot) {
                        self.dang_keo_chon = true;
                        self.diem_bat_dau_keo = vi_tri_chuot;
                        self.diem_hien_tai_keo = vi_tri_chuot;
                    }
                } else if self.dang_keo_chon {
                    self.dang_keo_chon = false;
                    self.diem_hien_tai_keo = vi_tri_chuot;
                    self.base_mut().queue_redraw();

                    let khoang_cach = self.diem_bat_dau_keo.distance_to(self.diem_hien_tai_keo);
                    if khoang_cach < 8.0 {
                        self.xu_ly_click_diem(vi_tri_chuot);
                    } else {
                        let rect =
                            Self::tao_rect_tu_2_diem(self.diem_bat_dau_keo, self.diem_hien_tai_keo);
                        self.xu_ly_khoanh_vung_chon(rect);
                    }
                }
            }
            return;
        }

        // 2. Khi di chuyển chuột lúc đang kéo khoanh vùng
        if self.dang_keo_chon {
            if event.try_cast::<InputEventMouseMotion>().is_ok() {
                self.diem_hien_tai_keo = self.base().get_global_mouse_position();
                self.base_mut().queue_redraw();
            }
        }
    }

    fn draw(&mut self) {
        if self.dang_keo_chon
            && self.diem_bat_dau_keo.distance_to(self.diem_hien_tai_keo) >= 8.0
        {
            let rect = Self::tao_rect_tu_2_diem(self.diem_bat_dau_keo, self.diem_hien_tai_keo);
            // Vẽ nền xanh trong suốt và viền khung chọn
            self.base_mut()
                .draw_rect(rect, Color::from_rgba(0.2, 0.65, 1.0, 0.22));
            self.base_mut()
                .draw_rect_ex(rect, Color::from_rgba(0.3, 0.8, 1.0, 0.95))
                .filled(false)
                .width(2.0)
                .done();
        }
    }
}