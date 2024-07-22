use std::rc::Rc;

use petgraph::Graph;

use crate::chunk::{ChunkGroup, ChunkGroupId};

#[derive(Debug)]
pub struct ChunkGroupConnection {
    pub(crate) origin: ChunkGroupId,
    pub(crate) target: ChunkGroupId,
}

pub struct ChunkGraph {
    pub(crate) inner: Graph<Rc<ChunkGroup>, ChunkGroupConnection>,
}

impl Default for ChunkGraph {
    fn default() -> Self {
        Self {
            inner: Graph::new(),
        }
    }
}
