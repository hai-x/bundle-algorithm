# Bundle Algorithm

inspired by Webpack and Parcel.

![alt text](https://github.com/hai-x/bundle-algorithm/blob/master/image/graph.jpg)

```javascript
chunk_graph: digraph {
    0 [ label = "ChunkGroup { chunks: RefCell { value: [Chunk { modules: RefCell { value: [NodeIndex(0)] }, in_chunk_groups: RefCell { value: [NodeIndex(0)] }, size: 10 }] }, children: [], size: 10 }" ]
    1 [ label = "ChunkGroup { chunks: RefCell { value: [Chunk { modules: RefCell { value: [NodeIndex(3)] }, in_chunk_groups: RefCell { value: [NodeIndex(1)] }, size: 10 }] }, children: [], size: 10 }" ]
    2 [ label = "ChunkGroup { chunks: RefCell { value: [Chunk { modules: RefCell { value: [NodeIndex(8), NodeIndex(5), NodeIndex(7)] }, in_chunk_groups: RefCell { value: [NodeIndex(2)] }, size: 10 }] }, children: [], size: 10 }" ]
    3 [ label = "ChunkGroup { chunks: RefCell { value: [Chunk { modules: RefCell { value: [NodeIndex(4)] }, in_chunk_groups: RefCell { value: [NodeIndex(3)] }, size: 10 }] }, children: [], size: 10 }" ]
    4 [ label = "ChunkGroup { chunks: RefCell { value: [Chunk { modules: RefCell { value: [NodeIndex(1)] }, in_chunk_groups: RefCell { value: [NodeIndex(4)] }, size: 20 }] }, children: [], size: 20 }" ]
    5 [ label = "ChunkGroup { chunks: RefCell { value: [Chunk { modules: RefCell { value: [NodeIndex(6), NodeIndex(7)] }, in_chunk_groups: RefCell { value: [NodeIndex(5)] }, size: 10 }] }, children: [], size: 10 }" ]
    6 [ label = "ChunkGroup { chunks: RefCell { value: [Chunk { modules: RefCell { value: [NodeIndex(2)] }, in_chunk_groups: RefCell { value: [NodeIndex(6)] }, size: 30 }] }, children: [], size: 30 }" ]
    0 -> 1 [ label = "ChunkGroupConnection { origin: NodeIndex(0), target: NodeIndex(1) }" ]
    1 -> 2 [ label = "ChunkGroupConnection { origin: NodeIndex(1), target: NodeIndex(2) }" ]
    4 -> 5 [ label = "ChunkGroupConnection { origin: NodeIndex(4), target: NodeIndex(5) }" ]
}
```
