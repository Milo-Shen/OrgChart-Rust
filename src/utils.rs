// use std
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

//  use local types
use crate::org_chart::CardNode;

pub fn traverse_tree_by_dfs<F>(root: Option<Rc<RefCell<CardNode>>>, mut callback: F)
where
    F: FnMut(Rc<RefCell<CardNode>>) -> (),
{
    if root.is_none() {
        return;
    }

    let mut pre = root.unwrap();
    let mut stack = VecDeque::from([Rc::clone(&pre)]);

    while !stack.is_empty() {
        let node = Rc::clone(stack.back().unwrap());
        if node.borrow().children.is_empty() || pre.borrow().id == node.borrow().children.last().unwrap().borrow().id {
            stack.pop_back();
            callback(Rc::clone(&node));
        } else {
            let child_len = node.borrow().children.len();
            for i in (0..child_len).rev() {
                stack.push_back(Rc::clone(&node.borrow().children[i]));
            }
        }

        pre = node;
    }
}

pub fn traverse_tree_by_level<F>(root: Option<Rc<RefCell<CardNode>>>, mut callback: F)
where
    F: FnMut(Rc<RefCell<CardNode>>) -> (),
{
    let mut queue = VecDeque::from([root.unwrap()]);

    while !queue.is_empty() {
        let card = queue.pop_front().unwrap();
        callback(Rc::clone(&card));

        for child in &card.borrow().children {
            queue.push_back(Rc::clone(child));
        }
    }
}

pub fn is_leaf(node: &Rc<RefCell<CardNode>>) -> bool {
    node.borrow().children.is_empty()
}

pub fn is_even(num: usize) -> bool {
    num % 2 == 0
}
