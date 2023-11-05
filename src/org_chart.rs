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

pub struct CardNode<T> {
    id: i64,
    content: T,
    children: RefCell<Vec<Rc<CardNode<T>>>>,
    parent: RefCell<Weak<CardNode<T>>>,
    previous: RefCell<Weak<CardNode<T>>>,
    level: i64,
    level_previous: RefCell<Weak<CardNode<T>>>,
    level_first: RefCell<Weak<CardNode<T>>>,
    width: f32,
    height: f32,
    pos_x: f32,
    pos_y: f32,
    mode: CardNodeType,
}

impl<T> CardNode<T> {
    pub fn new(id: i64, content: T, w: f32, h: f32, mode: CardNodeType) -> CardNode<T> {
        CardNode {
            id,
            content,
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

pub struct OrgChart<T> {
    root: Option<Rc<CardNode<T>>>,
    previous_card: RefCell<Weak<CardNode<T>>>,
    card_map: HashMap<i64, Rc<CardNode<T>>>,
    card_list: RefCell<Vec<Rc<CardNode<T>>>>,
    card_linked_list: RefCell<VecDeque<Vec<Rc<CardNode<T>>>>>,
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
