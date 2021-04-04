mod data_struct;
use data_struct::Graph;

fn main() {
    println!("Hello, world!");
    let mut graph: Graph = Graph::new();
    let filename = "database.txt".to_string();
    graph.parse(filename);
    graph.show();

    let avg_depth = graph.get_avg_depth();
    println!("avg_depth={}", avg_depth);

    let avg_tx = graph.get_avg_tx_per_depth();
    println!("avg_tx={}", avg_tx);

    let avg_ref = graph.get_avg_reference_per_node();
    println!("avg_ref={}", avg_ref);

    let avg_depth_of_leaves = graph.get_avg_depth_of_leaves();
    println!("avg_depth_of_leaves={}", avg_depth_of_leaves);

    let access_matrix = graph.get_graph_matrix();
    for v in access_matrix {
        println!("{:?}", v);
    }

    let distance = graph.get_depth_between_nodes(2, 4);
    println!("distance 2;4 = {:?}", distance);
    let distance = graph.get_depth_between_nodes(1, 4);
    println!("distance 1;4 = {:?}", distance);
    let distance = graph.get_depth_between_nodes(2, 6);
    println!("distance 2;6 = {:?}", distance);
    let distance = graph.get_depth_between_nodes(0, 5);
    println!("distance 0;5 = {:?}", distance);
    let distance = graph.get_depth_between_nodes(4, 6);
    println!("distance 4;6 = {:?}", distance);
}
