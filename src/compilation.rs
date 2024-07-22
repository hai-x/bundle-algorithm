extern crate petgraph;

use crate::{build_chunk_graph::Builder, chunk_graph::ChunkGraph, module_graph::ModuleGraph};

use petgraph::dot::Dot;

pub struct Compilation {
    pub(crate) chunk_graph: ChunkGraph,
    pub(crate) module_graph: ModuleGraph,
}

impl Compilation {
    pub(super) fn new() -> Self {
        Self {
            chunk_graph: ChunkGraph::default(),
            module_graph: ModuleGraph::default(),
        }
    }

    pub(super) fn build(&mut self) {
        ModuleGraph::mock(self);
        let mut builder = Builder::new(self);
        builder.build();
        println!("module_graph: {:?}", Dot::new(&self.module_graph.inner));
        println!("chunk_graph: {:?}", Dot::new(&self.chunk_graph.inner));
    }
}
