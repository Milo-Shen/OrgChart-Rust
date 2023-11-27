// use std
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};

//  use local types
use crate::line::{LineNode, LineType};
use crate::mock_org_chart_data::MockChartData;
use crate::utils::{is_even, is_leaf, traverse_tree_by_dfs};

#[derive(Debug)]
pub enum CardNodeType {
    NORMAL,
    LITE,
    BATCH,
    EXTEND,
}

#[derive(Debug)]
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
    pub root: Option<Rc<RefCell<CardNode>>>,
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
    horizon_gap: f32,
    vertical_gap: f32,
    batch_column_capacity: i64,
}

impl OrgChart {
    pub fn new(fixed_size: bool, fixed_width: f32, fixed_height: f32, lite_width: f32, lite_height: f32, horizon_gap: f32, vertical_gap: f32, line_width: f32, batch_column_capacity: i64) -> OrgChart {
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
            horizon_gap,
            vertical_gap,
            batch_column_capacity,
        }
    }

    pub fn initialization(&mut self, card_raw_list: Vec<MockChartData>) {
        let card_capacity = card_raw_list.len();
        self.card_list = Vec::with_capacity(card_capacity);
        self.line_list = Vec::with_capacity(card_capacity);

        // initial the root node
        let root_data = &card_raw_list[0];
        self.root = Some(Rc::new(RefCell::new(CardNode::new(root_data.id, 0.0, 0.0, CardNodeType::NORMAL))));
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
        if !self.fixed_size {
            return;
        }

        let root = self.root.clone().unwrap();
        node.borrow_mut().width = self.fixed_width;
        node.borrow_mut().height = self.fixed_height;
    }

    fn initialize_tree_from_raw_data(&mut self, card_raw_list: &Vec<MockChartData>) {
        // build card node map
        let card_raw_list_len = card_raw_list.len();

        for i in 1..card_raw_list_len {
            let card_raw = &card_raw_list[i];
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
                self.card_list.push(Rc::clone(&card));

                // loop the next level of nodes
                for child in &card.borrow().children {
                    queue.push_back(Rc::clone(child));
                }
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
        self.calculate_line_pos();
    }

    fn update_node_horizon_space(&mut self) {
        self.previous_card = Weak::new();

        traverse_tree_by_dfs(self.root.clone(), |node| {
            // most left node of each subtree
            self.update_node_horizon_space_most_left_leaf(Rc::clone(&node));

            // sibling node
            self.update_node_horizon_space_sibling_nodes(Rc::clone(&node));

            // go to the parent node
            self.update_node_horizon_space_parent_node(Rc::clone(&node));
        })
    }

    fn update_node_horizon_space_most_left_leaf(&mut self, node: Rc<RefCell<CardNode>>) {
        // most left node of each subtree
        if !is_leaf(&node) || node.borrow().previous.upgrade().is_some() {
            return;
        }

        let level_previous_option = node.borrow().level_previous.upgrade();
        if level_previous_option.is_some() {
            let level_previous = level_previous_option.unwrap();
            node.borrow_mut().pos_x = level_previous.borrow().pos_x + level_previous.borrow().width + self.horizon_gap;
        } else {
            node.borrow_mut().pos_x = 0.0;
        }

        self.readjust_horizon_pos_of_subtree(Rc::clone(&node));
        self.previous_card = Rc::downgrade(&node);
    }

    fn readjust_horizon_pos_of_subtree(&self, root: Rc<RefCell<CardNode>>) {
        let level_previous_option = root.borrow().level_previous.upgrade();
        if level_previous_option.is_none() {
            return;
        }

        let level_previous = level_previous_option.unwrap();
        let min_pos = level_previous.borrow().pos_x + level_previous.borrow().width + self.horizon_gap;
        if min_pos < root.borrow().pos_x {
            return;
        }

        let diff = min_pos - root.borrow().pos_x;
        let mut queue = VecDeque::from([Rc::clone(&root)]);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let new_pos_x = node.borrow().pos_x + diff;
            node.borrow_mut().pos_x = new_pos_x;

            for child in &node.borrow().children {
                queue.push_back(Rc::clone(child));
            }
        }
    }

    fn update_node_horizon_space_sibling_nodes(&mut self, node: Rc<RefCell<CardNode>>) {
        // sibling node
        let previous_opt = node.borrow().previous.upgrade();
        let previous_card_opt = self.previous_card.upgrade();
        if previous_opt.is_none() || previous_card_opt.is_none() {
            return;
        }

        let previous = previous_opt.unwrap();
        let previous_card = previous_card_opt.unwrap();

        if previous.borrow().id != previous_card.borrow().id {
            return;
        }

        let new_pos_x = previous.borrow().pos_x + previous.borrow().width + self.horizon_gap;
        node.borrow_mut().pos_x = new_pos_x;

        self.previous_card = Rc::downgrade(&node);
    }

    fn update_node_horizon_space_parent_node(&mut self, node: Rc<RefCell<CardNode>>) {
        if self.previous_card.upgrade().is_none() {
            return;
        }

        let previous_card = self.previous_card.upgrade().unwrap();

        let previous_card_parent_opt = previous_card.borrow().parent.upgrade();
        if previous_card_parent_opt.is_none() {
            return;
        }

        if previous_card_parent_opt.unwrap().borrow().id != node.borrow().id {
            return;
        }

        let node_children_len = node.borrow().children.len();
        if node_children_len == 1 {
            // todo: performance optimization -> readjust_horizon_pos_of_subtree ?
            // if the parent only has one child, the pos_x of the parent node will as same as the child
            node.borrow_mut().pos_x = previous_card.borrow().pos_x;
            // odd number case
        } else if !is_even(node_children_len) {
            let mid_pos = node_children_len / 2;
            let pos_x = node.borrow().children[mid_pos].borrow().pos_x;
            node.borrow_mut().pos_x = pos_x;
        } else {
            let start = node.borrow().children[0].borrow().pos_x;
            let end = node.borrow().children[node_children_len - 1].borrow().pos_x;
            node.borrow_mut().pos_x = (start + end) / 2.0;
        }

        self.readjust_horizon_pos_of_subtree(Rc::clone(&node));
        self.previous_card = Rc::downgrade(&node);
    }

    fn calculate_line_pos(&mut self) {
        traverse_tree_by_dfs(self.root.clone(), |node| {
            if is_leaf(&node) {
                return;
            }

            // create line node
            let children_len = node.borrow().children.len();

            // case one: one parent has one child
            if children_len == 1 {
                let x = node.borrow().pos_x + (node.borrow().width - self.line_width) / 2.0;
                let y = node.borrow().pos_y + node.borrow().height;
                let w = self.line_width;
                let h = self.vertical_gap;
                let line_node = LineNode::new(x, y, w, h, LineType::LINE, self.line_width);
                self.line_list.push(line_node);
            } else {
                // case two: one parent has multi children
                let first = Rc::clone(node.borrow().children.first().unwrap());
                let last = Rc::clone(node.borrow().children.last().unwrap());

                // get the mid pos of a card
                let start = first.borrow().pos_x + (first.borrow().width - self.line_width) / 2.0;
                let end = last.borrow().pos_x + (last.borrow().width - self.line_width) / 2.0;

                // update line info
                let x = start;
                let h = (self.vertical_gap + self.line_width) / 2.0;
                let y = first.borrow().pos_y - h;
                let w = end - start;
                let square_node = LineNode::new(x, y, w, h, LineType::Square, self.line_width);
                self.line_list.push(square_node);

                // case three: parent to category line
                let x = node.borrow().pos_x + (node.borrow().width - self.line_width) / 2.0;
                let y = node.borrow().pos_y + node.borrow().height;
                let w = self.line_width;
                let h = (self.vertical_gap - self.line_width) / 2.0;
                let p_to_c_line = LineNode::new(x, y, w, h, LineType::LINE, self.line_width);
                self.line_list.push(p_to_c_line);

                // case four: parent to node line
                for child in &node.borrow().children {
                    let x = child.borrow().pos_x + (child.borrow().width - self.line_width) / 2.0;
                    let y = child.borrow().pos_y - (self.vertical_gap - self.line_width) / 2.0;
                    let w = self.line_width;
                    let h = (self.vertical_gap + self.line_width) / 2.0;
                    let p_to_n_line = LineNode::new(x, y, w, h, LineType::LINE, self.line_width);
                    self.line_list.push(p_to_n_line);
                }
            }
        })
    }

    pub fn get_render_data(&self) -> (&Vec<Rc<RefCell<CardNode>>>, &Vec<LineNode>) {
        (&self.card_list, &self.line_list)
    }
}
