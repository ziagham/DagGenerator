pub trait DAGInterface<T> {

    /// Adds a vertex to the DAG.
    ///
    /// The weight of the vertex is passed to the method and an identifier for
    /// the vertex is returned.
    fn add_vertex(&mut self, w: T) -> u64;

    /// Adds an edge between two verices to the DAG.
    ///
    /// Should return `Err(_)` if the edge being added somehow introduces a
    /// cycle to the graph. On success `Ok(_)` is returned, the content is of
    /// no importance.
    fn add_edge(&mut self, a: u64, b: u64, w: T) -> Result<bool, &'static str>;

    /// Returns a topological ordering of the vertices in the graph.
    ///
    /// The topological ordering is returned as a `Vec<u64>` conaining the id's
    /// of each node in the graph. The ordering starts at the beginging of the
    /// `Vec`.
    fn topological_order(&self) -> Result<Vec<u64>, &'static str>;

    /// Calculates the weigth of the longest path between two vertices.
    ///
    /// The functions passed will be applied to the weight of each vertex and
    /// edge and the results will then be summed.
    fn weight_of_longest_path<F1, F2>(&self, from: u64, to: u64, v_sum: &F1, e_sum: &F2) -> Option<T>
        where F1: Fn(T) -> T,
              F2: Fn(T) -> T;
}