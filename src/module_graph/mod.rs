use std::{cell::RefCell, rc::Rc};

use crate::{
    compilation::Compilation,
    module::{
        dependency::{self, Dependency},
        Module, ModuleType,
    },
};
use petgraph::Graph;

#[derive(Clone)]
pub struct ModuleGraph {
    pub(crate) inner: Graph<Rc<Module>, dependency::Dependency>,
}

impl Default for ModuleGraph {
    fn default() -> Self {
        Self {
            inner: Graph::new(),
        }
    }
}

impl ModuleGraph {
    pub(crate) fn mock(compilation: &mut Compilation) {
        // compilation.module_graph = Self::default();
        let mut g = Graph::new();

        let html = g.add_node(Rc::new(Module {
            name: "a.html".to_string(),
            module_type: ModuleType::HTML,
            size: 10,
            is_entry: true,
            in_chunks: RefCell::new(vec![]),
        }));

        let html2 = g.add_node(Rc::new(Module {
            name: "b.html".to_string(),
            module_type: ModuleType::HTML,
            size: 20,
            is_entry: true,
            in_chunks: RefCell::new(vec![]),
        }));

        let html3 = g.add_node(Rc::new(Module {
            name: "c.html".to_string(),
            module_type: ModuleType::HTML,
            size: 30,
            is_entry: true,
            in_chunks: RefCell::new(vec![]),
        }));

        let js = g.add_node(Rc::new(Module {
            name: "a.js".to_string(),
            module_type: ModuleType::JavaScript,
            size: 10,
            is_entry: false,
            in_chunks: RefCell::new(vec![]),
        }));

        let js2 = g.add_node(Rc::new(Module {
            name: "async.js".to_string(),
            module_type: ModuleType::JavaScript,
            size: 10,
            is_entry: false,
            in_chunks: RefCell::new(vec![]),
        }));

        let js3 = g.add_node(Rc::new(Module {
            name: "async2.js".to_string(),
            module_type: ModuleType::JavaScript,
            size: 10,
            is_entry: false,
            in_chunks: RefCell::new(vec![]),
        }));

        let js4 = g.add_node(Rc::new(Module {
            name: "b.js".to_string(),
            module_type: ModuleType::JavaScript,
            size: 10,
            is_entry: false,
            in_chunks: RefCell::new(vec![]),
        }));

        let js5 = g.add_node(Rc::new(Module {
            name: "shared.js".to_string(),
            module_type: ModuleType::JavaScript,
            size: 10,
            is_entry: false,
            in_chunks: RefCell::new(vec![]),
        }));

        let css = g.add_node(Rc::new(Module {
            name: "styles.css".to_string(),
            module_type: ModuleType::CSS,
            size: 10,
            is_entry: false,
            in_chunks: RefCell::new(vec![]),
        }));

        g.add_edge(
            html,
            js,
            Dependency {
                is_async: false,
                origin: html,
                target: js,
            },
        );
        g.add_edge(
            js,
            js2,
            Dependency {
                is_async: true,
                origin: js,
                target: js2,
            },
        );
        g.add_edge(
            js,
            js3,
            Dependency {
                is_async: false,
                origin: js,
                target: js3,
            },
        );
        g.add_edge(
            js2,
            js3,
            Dependency {
                is_async: true,
                origin: js2,
                target: js3,
            },
        );
        g.add_edge(
            js3,
            js5,
            Dependency {
                is_async: false,
                origin: js3,
                target: js5,
            },
        );
        g.add_edge(
            js,
            css,
            Dependency {
                is_async: false,
                origin: js,
                target: css,
            },
        );

        g.add_edge(
            html2,
            js4,
            Dependency {
                is_async: false,
                origin: html2,
                target: js4,
            },
        );

        g.add_edge(
            js4,
            js5,
            Dependency {
                is_async: false,
                origin: js4,
                target: js5,
            },
        );

        compilation.module_graph.inner = g;
    }
}
