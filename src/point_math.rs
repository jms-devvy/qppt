pub struct Point {
    pub x_pos: i64,
    pub y_pos: i64,
    pub prob: f32,
}

impl Point {
    //translates a prob float to a colour between 0 and 65,535 before before adding 0xFF0000 to it to get a valid colour
    pub fn getcol(&self) -> i32 {
        ((65535.0 / 100.0) * self.prob).round() as i32 + 0xFF0000
    }
}
