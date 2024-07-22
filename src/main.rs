mod compilation;

pub mod task;

pub mod build_chunk_graph;
pub mod chunk;
pub mod chunk_graph;
pub mod module;
pub mod module_graph;

use compilation::Compilation;

fn main() {
    let mut compilation = Compilation::new();
    compilation.build();
}
