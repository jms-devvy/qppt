use crate::render::ThdRenderVec;

pub mod point_math;
pub mod render;

fn main() {
    let mut test3d = render::ThdRenderVec::init(2, 2, 2);
    println!("{:?}", test3d);
    test3d.render_point(vec![0, 0, 1], 0xFFFFFF);
    println!("\n\n\n{:?}", test3d);
}
