use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;

/*
Todo:
-Handle objects shifting into conjoined areas and coming out with their own colours instead of another object's;
will probably handle this by giving each object an internal mini 3d grid with a hashmap to each ThdRenderVec cord so internal cords can
retain colour if another object overlaps them
-do math to get the size of a specific point using a basic sort of layer by layer point size decrease
-rotation logic

 */

//DEFAULT COORDS GO IN ORDER Z,X,Y

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
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

#[derive(Debug, Clone)]
pub struct RObject {
    //pub in_render_map: ThdRenderVec,
    pub owned_points: Vec<Vec<usize>>,
    pub rotated_byx: i32, //Rotations
    pub rotated_byy: i32,
}

impl RObject {
    pub fn rotate() {}
}

#[derive(Clone, Debug)]
pub struct ThdRenderVec {
    pub mapped: Vec<Vec<Vec<GivenCord>>>,
    pub axispoint: Vec<usize>,
    pub owned_objs: Vec<Option<RObject>>,
    pub title: String,
    pub two_d_list_of_pixel_locations_indexes: Vec<i64>,
    /*
    Simplest way to explain imo is that each vector is a layer (z), each layer has vectors inside of them, which correspond to y(rows), in those y rows is a vec of x coords
    [z][y][x]
    */
}

impl ThdRenderVec {
    fn get_px_indexes(&self, u32vec: &Vec<u32>) -> HashMap<usize, &GivenCord> {
        let len = u32vec.len();
        let own_layer_len = self.mapped[0].len() * self.mapped[0][0].len();
        //amount of y rows * x in those rows
        let multiplier = len / own_layer_len;
        //assumes own layers len is less than pixel buffer len
    }

    pub fn init(z: i64, y: i64, x: i64, desired_title: String) -> Self {
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
            owned_objs: vec![],
            title: desired_title,
            two_d_list_of_pixel_locations_indexes: Vec::new(), //placeholder
        };
    }
    pub fn return_size_of_given_point(&self) {}

    pub fn render_point(&mut self, points: Vec<usize>, colour: u32) {
        let z = points[0];
        let y = points[1];
        let x = points[2];
        self.mapped[z][y][x].colour = colour;
    }

    pub fn shift_obj(&mut self, obj: &mut RObject, z_plus: usize, y_plus: usize, x_plus: usize) {
        for i in &mut obj.owned_points {
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
            self.mapped[newz][newy][newx].colour = self.mapped[oldz][oldy][oldx].colour;
            //clears old points colour
            self.mapped[oldz][oldy][oldx].clear();
            //changes the location of each owned point
            i[0] = newz;
            i[1] = newy;
            i[2] = newx;
        }
    }
    //returns a vector of y rows
    fn create_2d(&self) -> Vec<Vec<GivenCord>> {
        let mut two_d_constructor = Vec::new();
        for i in self.mapped[0].clone() {
            two_d_constructor.push(i);
        }
        return two_d_constructor;
    }
    pub fn render_to_screen(&self) {
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let mut window = Window::new(&self.title, WIDTH, HEIGHT, WindowOptions::default())
            .expect("Unable to open window");
        window.set_target_fps(60);
        let mut frame: u32 = 0;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            // Write pixel data into the buffer

            window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .expect("Failed to update buffer");

            frame = frame.wrapping_add(1);
        }
    }

    pub fn show_current_state(&self) {
        println!("{:?}", self.mapped);
    }
}
