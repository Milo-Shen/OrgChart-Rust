use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};

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
            width: 0.0,
            height: 0.0,
            pos_x: 0.0,
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
    card_linked_list: VecDeque<Vec<Rc<CardNode<T>>>>,
}
