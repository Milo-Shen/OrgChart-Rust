// use std
use std::cell::{RefCell};
use std::collections::{HashMap, VecDeque};
use std::rc::{Rc, Weak};

//  use local types
use crate::org_chart::CardNode;

pub fn traverse_tree_by_dfs(root: Rc<RefCell<CardNode>>, callback: fn(Rc<RefCell<CardNode>>) -> !) {}