use std::{
    cell::RefCell,
    collections::{HashMap, LinkedList},
};

use petgraph::{
    graph::NodeIndex,
    visit::{depth_first_search, DfsEvent, EdgeRef},
    Graph,
};

extern crate petgraph;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType {
    JavaScript,
    CSS,
    HTML,
}

#[derive(Debug)]
pub struct Dependency {
    is_async: bool,
}

#[derive(Debug, Clone)]
pub struct Module<'a> {
    name: &'a str,
    module_type: ModuleType,
    size: usize,
    is_entry: bool,
}

#[derive(Debug)]
pub struct Chunk {
    module_ids: RefCell<Vec<ModuleId>>,
    size: usize,
    parents: RefCell<Vec<ChunkId>>,
}

#[derive(Debug)]
pub struct ChunkGroup {
    chunks_ids: RefCell<Vec<ChunkId>>,
    size: usize,
    parents: RefCell<Vec<ChunkGroupId>>,
}

impl Chunk {
    fn from_module(id: NodeIndex, module: &Module) -> Self {
        Chunk {
            module_ids: RefCell::new(vec![id]),
            size: module.size,
            parents: RefCell::new(vec![]),
        }
    }
}

impl ChunkGroup {
    fn from_chunk(id: NodeIndex, chunk: &Chunk) -> Self {
        ChunkGroup {
            chunks_ids: RefCell::new(vec![id]),
            size: chunk.size,
            parents: RefCell::new(vec![]),
        }
    }
}

type ChunkId = NodeIndex;
type ChunkGroupId = NodeIndex;
type ModuleId = NodeIndex;

pub struct Compilation;

impl Compilation {
    pub fn build_module_graph<'a>() -> Graph<Module<'a>, Dependency> {
        let mut module_graph: Graph<Module, Dependency> = Graph::new();

        let html = module_graph.add_node(Module {
            name: "a.html",
            module_type: ModuleType::HTML,
            size: 10,
            is_entry: true,
        });

        let html2 = module_graph.add_node(Module {
            name: "b.html",
            module_type: ModuleType::HTML,
            size: 20,
            is_entry: true,
        });

        let html3 = module_graph.add_node(Module {
            name: "c.html",
            module_type: ModuleType::HTML,
            size: 30,
            is_entry: true,
        });

        let js = module_graph.add_node(Module {
            name: "a.js",
            module_type: ModuleType::JavaScript,
            size: 10,
            is_entry: false,
        });

        let js2 = module_graph.add_node(Module {
            name: "async.js",
            module_type: ModuleType::JavaScript,
            size: 10,
            is_entry: false,
        });

        let js3 = module_graph.add_node(Module {
            name: "async2.js",
            module_type: ModuleType::JavaScript,
            size: 10,
            is_entry: false,
        });

        let js4 = module_graph.add_node(Module {
            name: "b.js",
            module_type: ModuleType::JavaScript,
            size: 10,
            is_entry: false,
        });

        let js5 = module_graph.add_node(Module {
            name: "shared.js",
            module_type: ModuleType::JavaScript,
            size: 10,
            is_entry: false,
        });

        let css = module_graph.add_node(Module {
            name: "styles.css",
            module_type: ModuleType::CSS,
            size: 10,
            is_entry: false,
        });

        module_graph.add_edge(html, js, Dependency { is_async: false });
        module_graph.add_edge(js, js2, Dependency { is_async: true });
        module_graph.add_edge(js, js3, Dependency { is_async: true });
        module_graph.add_edge(js2, js3, Dependency { is_async: false });
        module_graph.add_edge(js3, js5, Dependency { is_async: false });
        module_graph.add_edge(js, css, Dependency { is_async: false });

        module_graph.add_edge(html2, js4, Dependency { is_async: false });

        module_graph.add_edge(js4, js5, Dependency { is_async: false });

        return module_graph;
    }

    pub fn build_chunk_graph(
        g: Graph<Module, Dependency>,
    ) -> (Graph<Chunk, u32>, Graph<ChunkGroup, u32>) {
        let mut chunk_group_graph: Graph<ChunkGroup, u32> = Graph::new();
        let mut chunk_graph: Graph<Chunk, u32> = Graph::new();
        let mut module_chunk_map: HashMap<ModuleId, (ChunkId, ChunkGroupId)> = HashMap::new();

        let entries: Vec<ModuleId> = g
            .node_indices()
            .filter(|n| {
                return g[*n].is_entry;
            })
            .collect();
        println!("entries: {:?}", entries);

        for entry in &entries {
            let chunk_id: ChunkId = chunk_graph.add_node(Chunk::from_module(*entry, &g[*entry]));
            let chunk = &chunk_graph[chunk_id];
            let chunk_group_id: ChunkGroupId =
                chunk_group_graph.add_node(ChunkGroup::from_chunk(chunk_id, chunk));
            // bundle_root.insert(*entry, (bundle_id, bundle_id));
            module_chunk_map.insert(*entry, (chunk_id, chunk_group_id));
        }

        let mut stack = LinkedList::new();

        depth_first_search(&g, entries, |evt| {
            match evt {
                DfsEvent::Discover(module_id, _) => {
                    if let Some((chunk_id, chunk_group_id)) = module_chunk_map.get(&module_id) {
                        stack.push_front((module_id, *chunk_id, *chunk_group_id));
                    }
                }
                DfsEvent::TreeEdge(module_id_u, module_id_v) => {
                    let module_u = &g[module_id_u];
                    let module_v = &g[module_id_v];
                    let (_, cur_chunk_id, cur_chunk_group_id) = stack.front().unwrap();

                    let mut new_chunk = || {
                        let new_chunk: ChunkId =
                            chunk_graph.add_node(Chunk::from_module(module_id_v, module_v));
                        let new_chunk_group: ChunkGroupId = chunk_group_graph
                            .add_node(ChunkGroup::from_chunk(new_chunk, &chunk_graph[new_chunk]));
                        let c_g: &ChunkGroup = &chunk_group_graph[new_chunk_group];
                        let c = &chunk_graph[new_chunk];
                        c_g.parents.borrow_mut().push(*cur_chunk_group_id);
                        c.parents.borrow_mut().push(*cur_chunk_id);

                        chunk_graph.add_edge(*cur_chunk_id, new_chunk, 0);

                        module_chunk_map.insert(module_id_v, (new_chunk, new_chunk_group));

                        let edges = chunk_group_graph.edges(*cur_chunk_group_id);
                        let edge = &edges.clone().find(|e| {
                            return e.source() == *cur_chunk_id;
                        });
                        // when source chunk already in chunk_group, just update chunk_group
                        if edge.is_some() {
                            let target = edge.unwrap().target();
                            // println!("target: {:?}", &target);
                            let target_chunk_group = &chunk_group_graph[target];
                            target_chunk_group.chunks_ids.borrow_mut().push(new_chunk);
                        } else {
                            chunk_group_graph.add_edge(*cur_chunk_group_id, new_chunk_group, 0);
                        }
                    };

                    // 1. different module_type => create new chunk
                    // html -> js
                    // js -> css

                    // e.g.
                    // a.html -> a.js
                    // a.js -> styles.css
                    // b.html -> b.js
                    if module_u.module_type != module_v.module_type {
                        new_chunk();
                        return;
                    }

                    // 2. async dependency => create new chunk
                    let dep = &g[g.find_edge(module_id_u, module_id_v).unwrap()];
                    if dep.is_async {
                        new_chunk();
                        return;
                    }

                    // 3. if is_sync and same module type, update chunk graph
                    let chunk = &mut chunk_graph[*cur_chunk_id];
                    let chunk_group = &mut chunk_group_graph[*cur_chunk_group_id];
                    chunk.module_ids.borrow_mut().push(module_id_v);
                    chunk.size += module_v.size;
                    chunk_group.size += module_v.size;
                }
                DfsEvent::Finish(module_id, _) => {
                    let (cur_module_id, _, _) = stack.front().unwrap();
                    if module_id == *cur_module_id {
                        stack.pop_front();
                    }
                }
                _ => {}
            }
        });

        (chunk_graph, chunk_group_graph)
    }
}
