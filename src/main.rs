pub mod point_math;
pub mod render;

fn main() {
    let mut test3d = render::ThdRenderVec::init(2, 2, 2, "ESC TO EXIT".to_string());
    test3d.render_point(vec![0, 1, 0], 0xFFFFFF);
    test3d.render_point(vec![0, 0, 0], 0xFFFFFF);
    let mut single_point = render::RObject {
        owned_points: vec![vec![0, 1, 0], vec![0, 0, 0]],
        rotated_byx: 0,
        rotated_byy: 0,
    };
    test3d.show_current_state();
    test3d.shift_obj(&mut single_point, 0, 0, 1);
    test3d.show_current_state();
    render::renderr();
}
