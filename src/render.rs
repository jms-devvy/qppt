use minifb::{Key, Window, WindowOptions};

//DEFAULT COORDS GO IN ORDER Z,X,Y

const WIDTH: usize = 200;
const HEIGHT: usize = 300;

#[derive(Clone, Debug)]
pub struct Camera {
    x_rotation_point: f64,
    y_rotation_point: f64,
}
#[derive(Clone, Debug)]
pub struct GivenCord {
    own_point: Vec<usize>,
    colour: u32,
    //default black
}

#[derive(Clone, Debug)]
pub struct RObject {
    owned_points: Vec<Vec<usize>>,
    rotated_by: i32
}

#[derive(Clone, Debug)]
pub struct ThdRenderVec {
    pub mapped: Vec<Vec<Vec<GivenCord>>>,
    pub axispoint: Vec<usize>,
    /*
    Simplest way to explain imo is that each vector is a layer (z), each layer has vectors inside of them which correspond to y(rows), in those y rows is a vec of x coords
     */
}

impl ThdRenderVec {
    pub fn init(x: i64, y: i64, z: i64) -> Self {
        let mut creator_vec: Vec<Vec<Vec<GivenCord>>> = Vec::new();
        for zval in 0..z {
            let mut twod_vec: Vec<Vec<GivenCord>> = Vec::new();
            for yval in 0..y {
                let mut y_row: Vec<GivenCord> = Vec::new();
                for xval in 0..x {
                    y_row.push(GivenCord {
                        own_point: vec![zval as usize, xval as usize, yval as usize],
                        colour: 0x000000,
                    });
                }
                twod_vec.push(y_row);
            }

            creator_vec.push(twod_vec)
        }

        let mut found_axis: Vec<usize> = Vec::new();
        let layer_to_access = (creator_vec.len() as f64 / 2.0).round() as usize;
        let row_to_access = (creator_vec[layer_to_access].len() as f64 / 2.0).round() as usize;
        let x_to_access =
            (creator_vec[layer_to_access][row_to_access].len() as f64 / 2.0).round() as usize;
        found_axis.push(layer_to_access);
        found_axis.push(x_to_access);
        found_axis.push(row_to_access);

        return ThdRenderVec {
            mapped: creator_vec,
            axispoint: found_axis,
        };
    }

    pub fn return_size_of_given_point()

    pub fn render_point(&mut self, points: Vec<usize>, colour: u32) {
        let z = points[0];
        let x = points[1];
        let y = points[2];
        self.mapped[z][y][x].colour = colour;
    }
}
