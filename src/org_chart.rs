use std::cell::{Ref, RefCell};
use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};
use crate::line::LineNode;

pub enum CardNodeType {
    NORMAL,
    LITE,
    BATCH,
    EXTEND,
}

pub struct CardNode {
    id: i64,
    children: RefCell<Vec<Rc<CardNode>>>,
    parent: RefCell<Weak<CardNode>>,
    previous: RefCell<Weak<CardNode>>,
    level: i64,
    level_previous: RefCell<Weak<CardNode>>,
    level_first: RefCell<Weak<CardNode>>,
    width: f32,
    height: f32,
    pos_x: f32,
    pos_y: f32,
    mode: CardNodeType,
}

impl CardNode {
    pub fn new(id: i64, w: f32, h: f32, mode: CardNodeType) -> CardNode {
        CardNode {
            id,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
            previous: RefCell::new(Weak::new()),
            level: 0,
            level_previous: RefCell::new(Weak::new()),
            level_first: RefCell::new(Weak::new()),
            width: w,
            height: h,
            pos_x: f32::MIN,
            pos_y: 0.0,
            mode,
        }
    }
}

pub struct OrgChart {
    root: Option<Rc<CardNode>>,
    previous_card: RefCell<Weak<CardNode>>,
    card_map: HashMap<i64, Rc<CardNode>>,
    card_list: RefCell<Vec<Rc<CardNode>>>,
    card_linked_list: RefCell<VecDeque<Vec<Rc<CardNode>>>>,
    line_list: RefCell<Vec<LineNode>>,
    line_width: f32,
    line_color: String,
    line_radius: f32,
    fixed_size: bool,
    fixed_width: f32,
    fixed_height: f32,
    lite_width: f32,
    lite_height: f32,
    fixed_overall_width: f32,
    fixed_overall_height: f32,
    horizon_gap: f32,
    vertical_gap: f32,
    batch_column_capacity: i64,
}

impl OrgChart {
    pub fn new() {}
}
