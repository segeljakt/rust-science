use graphlib::Graph;
fn main() {
    let mut graph: Graph<usize> = Graph::new();

    // Add two vertices to the graph
    let id1 = graph.add_vertex(1);
    let id2 = graph.add_vertex(2);

    // Add an edge between the two vertices
    graph.add_edge(&id1, &id2);

    assert_eq!(*graph.fetch(&id1).unwrap(), 1);
    assert_eq!(*graph.fetch(&id2).unwrap(), 2);

    // The graph has 2 vertices and one edge at this point
    assert_eq!(graph.vertex_count(), 2);
    assert_eq!(graph.edge_count(), 1);

    // Remove one of the connected vertices
    graph.remove(&id1);

    assert_eq!(graph.vertex_count(), 1);
    assert_eq!(graph.edge_count(), 0);
}
