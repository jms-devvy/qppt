use minifb::{Key, Window, WindowOptions};

//DEFAULT COORDS GO IN ORDER Z,X,Y

const WIDTH: usize = 200;
const HEIGHT: usize = 300;
const DEFAULT_BLANK: u32 = 0x000000;

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

impl GivenCord {
    pub fn clear(&mut self) {
        self.colour = DEFAULT_BLANK;
    }
}

//Vec<usize> used for references to points on a map, while the actual structure of a map is is Vec<Vec<Vec<GivenCord>>>

#[derive(Debug)]
pub struct RObject {
    pub in_render_map: ThdRenderVec,
    pub owned_points: Vec<Vec<usize>>,
    pub rotated_by: i32, //Rotations
}

impl RObject {
    pub fn rotate() {}

    pub fn shift(&mut self, z_plus: usize, y_plus: usize, x_plus: usize) {
        for i in &mut self.owned_points {
            let oldz = i[0];
            let oldy = i[1];
            let oldx = i[2];
            let newz = i[0] + z_plus;
            let newy = i[1] + y_plus;
            let newx = i[2] + x_plus;
            //for every owned point -> shift its colour in the grid -> get rid of its original colour -> change the location of the owned point
            //changes the colours in the grid

            //NEW POINT.colour = OLD POINT.colour
            // not using vars to make it readable because borrow checker is a nob
            self.in_render_map.mapped[newz][newy][newx].colour =
                self.in_render_map.mapped[oldz][oldy][oldx].colour;
            //clears old points colour
            self.in_render_map.mapped[oldz][oldy][oldx].clear();
            //changes the location of each owned point
            i[0] = newz;
            i[1] = newy;
            i[2] = newx;
        }
    }

    pub fn show_status(&self) {
        println!("{:?}", self.in_render_map.mapped);
    }
}

#[derive(Clone, Debug)]
pub struct ThdRenderVec {
    pub mapped: Vec<Vec<Vec<GivenCord>>>,
    pub axispoint: Vec<usize>,
    /*
    Simplest way to explain imo is that each vector is a layer (z), each layer has vectors inside of them, which correspond to y(rows), in those y rows is a vec of x coords
    [z][y][x]
    */
}

impl ThdRenderVec {
    pub fn init(z: i64, y: i64, x: i64) -> Self {
        let mut creator_vec: Vec<Vec<Vec<GivenCord>>> = Vec::new();
        for zval in 0..z {
            let mut twod_vec: Vec<Vec<GivenCord>> = Vec::new();
            for yval in 0..y {
                let mut y_row: Vec<GivenCord> = Vec::new();
                for xval in 0..x {
                    y_row.push(GivenCord {
                        own_point: vec![zval as usize, yval as usize, xval as usize],
                        colour: DEFAULT_BLANK,
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
    pub fn return_size_of_given_point(&self) {}

    pub fn render_point(&mut self, points: Vec<usize>, colour: u32) {
        let z = points[0];
        let y = points[1];
        let x = points[2];
        self.mapped[z][y][x].colour = colour;
    }
}
