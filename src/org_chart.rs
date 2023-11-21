// use std
use std::cell::{RefCell};
use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};

//  use local types
use crate::line::LineNode;
use crate::mock_org_chart_data::MockChartData;
use crate::utils::traverse_tree_by_dfs;

pub enum CardNodeType {
    NORMAL,
    LITE,
    BATCH,
    EXTEND,
}

pub struct CardNode {
    pub id: i64,
    pub children: Vec<Rc<RefCell<CardNode>>>,
    pub parent: Weak<RefCell<CardNode>>,
    pub previous: Weak<RefCell<CardNode>>,
    pub level: i64,
    pub level_previous: Weak<RefCell<CardNode>>,
    pub level_first: Weak<RefCell<CardNode>>,
    pub width: f32,
    pub height: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub mode: CardNodeType,
}

impl CardNode {
    pub fn new(id: i64, w: f32, h: f32, mode: CardNodeType) -> CardNode {
        CardNode {
            id,
            children: Vec::new(),
            parent: Weak::new(),
            previous: Weak::new(),
            level: 0,
            level_previous: Weak::new(),
            level_first: Weak::new(),
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
    previous_card: Weak<RefCell<CardNode>>,
    card_map: HashMap<i64, Rc<RefCell<CardNode>>>,
    card_list: Vec<Rc<RefCell<CardNode>>>,
    line_list: Vec<LineNode>,
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

        OrgChart {
            root: None,
            previous_card: Weak::new(),
            card_map: HashMap::new(),
            card_list: Vec::new(),
            line_list: Vec::new(),
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
        }
    }

    pub fn initialization(&mut self, card_raw_list: Vec<MockChartData>) {
        // initial the root node
        let root_data = &card_raw_list[0];
        self.root = Some(Rc::new(RefCell::new(CardNode::new(root_data.id, 200.0, 100.0, CardNodeType::NORMAL))));
        self.initialize_fixed_width_height_of_a_node(&self.root.clone().unwrap());

        // initial the card map
        let root = self.root.clone().unwrap();
        self.card_map.insert(root.borrow().id, Rc::clone(&root));

        // generate card node from raw data
        self.initialize_tree_from_raw_data(&card_raw_list);

        // build the level previous relationship
        self.link_level_prev_card_and_build_card_list();

        // generate the horizon x position and lines
        self.generate_horizon_pos_and_lines();
    }

    fn initialize_fixed_width_height_of_a_node(&self, node: &Rc<RefCell<CardNode>>) {
        // process the fixed size type
        if self.fixed_size {
            let root = self.root.clone().unwrap();
            node.borrow_mut().width = self.fixed_width;
            node.borrow_mut().height = self.fixed_height;
        }
    }

    fn initialize_tree_from_raw_data(&mut self, card_raw_list: &Vec<MockChartData>) {
        let card_list_len = card_raw_list.len();

        // build card node map
        for card_raw in card_raw_list {
            let MockChartData { id, .. } = card_raw;
            let new_card = Rc::new(RefCell::new(CardNode::new(*id, 0.0, 0.0, CardNodeType::NORMAL)));

            // process the fixed size type
            self.initialize_fixed_width_height_of_a_node(&new_card);
            self.card_map.insert(*id, new_card);
        }

        // establish relationship between nodes
        for card_raw in card_raw_list {
            let MockChartData { id, children } = card_raw;
            let card = self.card_map.get(id).unwrap();
            let mut previous_card = Weak::new();

            for card_id in children {
                let child = self.card_map.get(card_id).unwrap();
                child.borrow_mut().parent = Rc::downgrade(card);
                child.borrow_mut().previous = Weak::clone(&previous_card);
                previous_card = Rc::downgrade(child);
                card.borrow_mut().children.push(Rc::clone(child));
            }
        }
    }

    fn link_level_prev_card_and_build_card_list(&mut self) {
        let mut queue = VecDeque::from([self.root.clone().unwrap()]);

        // the current level of card node
        let mut level = 0;

        while !queue.is_empty() {
            let len = queue.len();
            let mut pre_level_card: Weak<RefCell<CardNode>> = Weak::new();
            level += 1;

            let level_first = Rc::clone(queue.front().unwrap());

            for _ in 0..len {
                let card = queue.pop_front().unwrap();

                let card_parent_option = card.borrow().parent.upgrade();
                if card_parent_option.is_some() {
                    let card_parent = card_parent_option.unwrap();
                    card.borrow_mut().pos_y = card_parent.borrow().pos_y + card_parent.borrow().height + self.vertical_gap;
                } else {
                    card.borrow_mut().pos_y = 0.0;
                }

                // link the level previous card node to the current node
                card.borrow_mut().level_previous = pre_level_card;
                card.borrow_mut().level = level;
                card.borrow_mut().level_first = Rc::downgrade(&level_first);
                pre_level_card = Rc::downgrade(&card);

                // build card_list
                self.card_list.push(card);
            }
        }
    }

    fn generate_horizon_pos_and_lines(&mut self) {
        if self.root.is_none() {
            return;
        }

        // update the horizon space for each node
        self.update_node_horizon_space();

        // todo: update the vertical space for each node

        // calculate the line pos
    }

    fn update_node_horizon_space(&mut self) {
        self.previous_card = Weak::new();

        traverse_tree_by_dfs(self.root.clone(), |node| {
            // most left node of each subtree
            self.update_node_horizon_space_most_left_leaf(node);
        })
    }

    fn update_node_horizon_space_most_left_leaf(&mut self, root: Rc<RefCell<CardNode>>) {}
}
