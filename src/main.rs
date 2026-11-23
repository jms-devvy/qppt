use crate::render::ThdRenderVec;

pub mod point_math;
pub mod render;

fn main() {
    let mut test3d = render::ThdRenderVec::init(2, 2, 2);
    test3d.render_point(vec![0, 1, 0], 0xFFFFFF);
    test3d.render_point(vec![0, 0, 0], 0xFFFFFF);
    let mut single_point = render::RObject {
        in_render_map: test3d,
        owned_points: vec![vec![0, 1, 0], vec![0, 0, 0]],
        rotated_by: 0,
    };
    single_point.show_status();
    single_point.shift(0, 0, 1);
    single_point.show_status();
}
