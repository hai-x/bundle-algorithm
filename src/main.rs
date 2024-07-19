use petgraph::dot::Dot;

mod compilation;

use compilation::Compilation;

fn main() {
    let module_graph = Compilation::build_module_graph();
    println!("module_graph: {:?}", Dot::new(&module_graph));
    let (chunk_graph, chunk_group_graph) = Compilation::build_chunk_graph(module_graph);
    println!("chunk_graph: {:?}", Dot::new(&chunk_graph));
    println!("chunk_group_graph: {:?}", Dot::new(&chunk_group_graph));
}
