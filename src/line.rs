pub enum LineType {
    LINE,
    Square,
}

pub struct LineNode {
    pos_x: f32,
    pos_y: f32,
    width: f32,
    height: f32,
    border_width: f32,
    mode: LineType,
}

impl LineNode {
    pub fn new(x: f32, y: f32, w: f32, h: f32, mode: LineType, border_width: f32) -> LineNode {
        LineNode {
            pos_x: x,
            pos_y: y,
            width: w,
            height: h,
            border_width,
            mode,
        }
    }
}
