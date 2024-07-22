use std::{borrow::BorrowMut, cell::RefCell, collections::HashSet, rc::Rc};

use petgraph::Direction;

use crate::{
    chunk::{Chunk, ChunkGroup, ChunkGroupId},
    chunk_graph::ChunkGroupConnection,
    compilation::Compilation,
    module::{dependency::Dependency, Module, ModuleId, ModuleType},
};

pub struct Builder<'a> {
    pub(crate) compilation: &'a mut Compilation,
    pub(crate) queue: Vec<Dependency>,
    pub(crate) queue_delay: Vec<Dependency>,
    pub(crate) cur_chunk_group: Option<ChunkGroupId>,
    pub(crate) cur_chunk: Option<Rc<Chunk>>,
    // roughly use `visited_modules` to record module which has been processed
    // in order to remove duplicate module
    pub(crate) visited_modules: HashSet<ModuleId>,
}

impl<'a> Builder<'a> {
    pub(crate) fn new(compilation: &'a mut Compilation) -> Self {
        Self {
            compilation,
            queue: Default::default(),
            queue_delay: Default::default(),
            cur_chunk_group: None,
            cur_chunk: None,
            visited_modules: HashSet::new(),
        }
    }

    fn process_connection(&mut self, dep: &Dependency) {
        let chunk_graph = &mut self.compilation.chunk_graph.inner;
        let module_graph = &mut self.compilation.module_graph.inner;
        let origin = &module_graph[dep.origin];
        let target = &module_graph[dep.target];
        let is_async = dep.is_async;

        if self.visited_modules.contains(&dep.target) {
            return;
        }

        if origin.module_type != target.module_type || is_async {
            let c = Rc::new(Chunk {
                modules: RefCell::new(vec![dep.target]),
                in_chunk_groups: RefCell::new(vec![]),
                size: target.size,
            });

            target.in_chunks.borrow_mut().push(Rc::downgrade(&c));

            let c_g = Rc::new(ChunkGroup {
                chunks: RefCell::new(vec![c.clone()]),
                children: RefCell::new(vec![]),
            });

            // update chunk graph
            let c_g = chunk_graph.add_node(c_g);

            // update chunk
            c.in_chunk_groups.borrow_mut().push(c_g);

            // connect chunk group
            if self.cur_chunk_group.is_some() {
                let origin_c_g = self.cur_chunk_group.unwrap();
                chunk_graph[origin_c_g].children.borrow_mut().push(c_g);
                chunk_graph.add_edge(
                    origin_c_g,
                    c_g,
                    ChunkGroupConnection {
                        origin: origin_c_g,
                        target: c_g,
                    },
                );
            }
            self.cur_chunk_group = Some(c_g);
            self.cur_chunk = Some(c);
        } else {
            // if is_sync and same module type, update chunk graph
            let cur_chunk = self.cur_chunk.as_ref().unwrap();
            cur_chunk.modules.borrow_mut().push(dep.target);
        }
    }

    fn visit_module(&mut self, module_id: ModuleId) {
        self.visited_modules.insert(module_id);

        for connection in self
            .compilation
            .module_graph
            .inner
            .edges_directed(module_id, Direction::Outgoing)
        {
            let weight = connection.weight();
            if weight.is_async {
                self.queue_delay.push(weight.clone());
            } else {
                self.queue.push(weight.clone());
            }
        }
    }

    pub(crate) fn build(&mut self) {
        let g = &mut self.compilation.module_graph.inner;
        let entries: Vec<ModuleId> = g
            .node_indices()
            .filter(|n| {
                return g[*n].is_entry;
            })
            .collect();

        println!("entries: {:?}", &entries);

        let mut initial_dep = Vec::new();

        for entry in entries {
            let dummy_node = g.add_node(Rc::new(Module {
                name: format!("dummy-{:?}", g[entry].name),
                module_type: ModuleType::Dummy,
                size: 0,
                in_chunks: RefCell::new(Vec::new()),
                is_entry: true,
            }));

            initial_dep.push(Dependency {
                is_async: false,
                origin: dummy_node,
                target: entry,
            });
        }

        self.queue = initial_dep;

        self.queue.reverse();
        while let Some(dep) = self.queue.pop() {
            self.process_connection(&dep);
            self.visit_module(dep.target);

            if self.queue.is_empty() {
                let temp = self.queue.clone();
                self.queue = self.queue_delay.clone();
                self.queue_delay = temp;
                self.queue.reverse();
            };
        }
    }
}
