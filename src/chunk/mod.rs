use std::{cell::RefCell, rc::Rc};

use petgraph::graph::NodeIndex;

use crate::module::ModuleId;

pub type ChunkGroupId = NodeIndex;

#[derive(Debug)]
pub struct Chunk {
    pub(crate) modules: RefCell<Vec<ModuleId>>,
    pub(crate) in_chunk_groups: RefCell<Vec<ChunkGroupId>>,
    pub(crate) size: usize,
}

#[derive(Debug)]
pub struct ChunkGroup {
    pub(crate) chunks: RefCell<Vec<Rc<Chunk>>>,
    pub(crate) children: RefCell<Vec<ChunkGroupId>>,
}
