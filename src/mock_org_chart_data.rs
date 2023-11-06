use std::collections::VecDeque;
use std::rc::Rc;

pub struct GenerateID {
    id: i64,
}

impl GenerateID {
    pub fn new() -> GenerateID {
        GenerateID {
            id: 0
        }
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

pub fn mock_org_chart_data(count: i64, max_child: i64, is_range: bool) -> Vec<MockChartData> {
    let mut result = vec![];
    let mut queue = VecDeque::new();

    // generated leaf count
    let mut remain_count = count - 1;

    // build the root leaf
    let root = Rc::new(build_card());

    result.push(Rc::clone(&root));
    queue.push_back(Rc::clone(&root));

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let children = vec![];
        let child_count = max_child.min(remain_count);
    }

    vec![]
}