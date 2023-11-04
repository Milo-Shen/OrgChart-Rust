use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub enum CardNodeType {
    NORMAL = 0,
    LITE = 1 << 0,
    BATCH = 1 << 1,
    EXTEND = 1 << 2,
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
    pos_x: f64,
    pos_y: f64,
    mode: CardNodeType,
}