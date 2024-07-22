use std::{cell::RefCell, rc::Weak};

use petgraph::graph::NodeIndex;

use crate::chunk::Chunk;

pub type ModuleId = NodeIndex;

pub mod dependency;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ModuleType {
    JavaScript,
    CSS,
    HTML,
    Dummy,
}

#[derive(Debug)]
pub struct Module {
    pub(crate) name: String,
    pub(crate) module_type: ModuleType,
    pub(crate) size: usize,
    pub(crate) is_entry: bool,
    pub(crate) in_chunks: RefCell<Vec<Weak<Chunk>>>,
}
