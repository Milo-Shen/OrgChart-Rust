use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use rand::Rng;

pub struct GenerateID {
    id: i64,
}

impl GenerateID {
    pub fn new() -> GenerateID {
        GenerateID { id: 0 }
    }

    pub fn get_next_id(&mut self) -> i64 {
        self.id += 1;
        self.id
    }
}

pub struct MockChartData {
    id: i64,
    children: Vec<i64>,
}

pub fn build_card() -> MockChartData {
    let mut generate_id = GenerateID::new();
    let id = generate_id.get_next_id();

    MockChartData {
        id,
        children: vec![],
    }
}

pub fn range(min: i64, max: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn mock_org_chart_data(count: i64, max_child: i64, is_range: bool) -> Vec<Rc<RefCell<MockChartData>>> {
    let mut result = vec![];
    let mut queue = VecDeque::new();

    // generated leaf count
    let mut remain_count = count - 1;

    // build the root leaf
    let root = Rc::new(RefCell::new(build_card()));

    result.push(Rc::clone(&root));
    queue.push_back(Rc::clone(&root));

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        let mut children = vec![];
        let mut child_count = max_child.min(remain_count);

        if is_range {
            child_count = range(0, child_count)
        }

        for i in 0..child_count {
            remain_count -= 1;

            let card = Rc::new(RefCell::new(build_card()));
            children.push(card.borrow().id);
            queue.push_back(Rc::clone(&card));
            result.push(Rc::clone(&card));
        }

        node.borrow_mut().children = children;

        if remain_count <= 0 {
            return result;
        }
    }

    result
}
