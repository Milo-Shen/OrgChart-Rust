// use std
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};

//  use local types
use crate::line::LineNode;
use crate::mock_org_chart_data::MockChartData;

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
    root: Option<Rc<RefCell<CardNode>>>,
    previous_card: RefCell<Weak<CardNode>>,
    card_map: HashMap<i64, Rc<RefCell<CardNode>>>,
    card_list: RefCell<Vec<Rc<CardNode>>>,
    card_linked_list: RefCell<VecDeque<Rc<CardNode>>>,
    line_list: RefCell<Vec<LineNode>>,
    line_width: f32,
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
    pub fn new(
        card_raw_list: Vec<Rc<RefCell<MockChartData>>>,
        fixed_size: bool,
        fixed_width: f32,
        fixed_height: f32,
        lite_width: f32,
        lite_height: f32,
        horizon_gap: f32,
        vertical_gap: f32,
        line_width: f32,
        batch_column_capacity: i64,
    ) -> OrgChart {
        // process the fixed size type
        let mut fixed_overall_width = 0.0;
        let mut fixed_overall_height = 0.0;
        if fixed_size {
            fixed_overall_width = fixed_width + horizon_gap;
            fixed_overall_height = fixed_height + vertical_gap;
        }

        // create the root node
        let root_data = &card_raw_list[0];
        let root = Rc::new(RefCell::new(CardNode::new(root_data.borrow().id, 200.0, 100.0, CardNodeType::NORMAL)));
        root.borrow_mut().pos_y = 0.0;

        let mut org_chart = OrgChart {
            root: Some(Rc::clone(&root)),
            previous_card: RefCell::new(Weak::new()),
            card_map: HashMap::new(),
            card_list: RefCell::new(vec![]),
            card_linked_list: RefCell::new(VecDeque::new()),
            line_list: RefCell::new(vec![]),
            line_width,
            fixed_size,
            fixed_width,
            fixed_height,
            lite_width,
            lite_height,
            fixed_overall_width,
            fixed_overall_height,
            horizon_gap,
            vertical_gap,
            batch_column_capacity,
        };

        org_chart.initialize_fixed_width_height_of_a_node(Rc::clone(&root));

        // initial the card map
        org_chart.card_map.insert(root.borrow().id, Rc::clone(&root));

        return org_chart;
    }

    pub fn initialize_fixed_width_height_of_a_node(&self, card_node: Rc<RefCell<CardNode>>) {
        // process the fixed size type
        if self.fixed_size {
            card_node.borrow_mut().width = self.fixed_width;
            card_node.borrow_mut().height = self.fixed_height;
        }
    }
}
