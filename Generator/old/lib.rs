pub mod interface;
use interface::DAGInterface;
use std::cmp::Ordering;
use std::ops::Add;
type ID = u64;

#[derive(PartialEq,Debug,Clone)]
struct Edge<T> {
    from: ID,
    to: ID,
    weight: T,
}

#[derive(PartialEq,Debug,Clone)]
struct Vertex<T> {
    id: ID,
    weight: T,
}

pub struct DAG<T:Clone> {
    vertices: Vec<Vertex<T>>,
    edges: Vec<Edge<T>>,
    next_id: ID,
}

impl <T:Clone+Add<Output=T>+Ord> DAG<T> {
    /// Creates a new instance of `lab2_dag::DAG`.
    pub fn new() -> DAG<T> {
        DAG {
            vertices: Vec::new(),
            edges: Vec::new(),
            next_id: 0
        }
    }

    /// Splits a `Vec<Vertex<T>>` into two.
    ///
    /// One of the vectors will contain all vertices with no edges **to** it,
    /// the other will contain all other vertices.
    fn split_remaining_v(vec: Vec<Vertex<T>>, edges: & Vec<Edge<T>>) -> (Vec<Vertex<T>>, Vec<Vertex<T>>) {
        let v_iter = vec.clone().into_iter();
        let remaining = vec.into_iter()
            .filter(|&Vertex{id: my_id, weight: _}| {
                         for i in 0..(edges.len()) {
                             let Edge { from: _, to: destination, weight: _} = edges[i];
                             if my_id == destination {
                                 return true
                             }
                         };
                         false})
            .collect();

        let filtered = v_iter
            .filter(|&Vertex{id: my_id, weight: _}| {
                         for i in 0..(edges.len()) {
                             let Edge { from: _, to: destination, weight: _} = edges[i];
                             if my_id == destination {
                                 return false
                             }
                         };
                         true})
            .collect();

        (filtered, remaining)
    }

    /// Returns the weight of a specified vertex
    ///
    /// #Panics
    /// The method will panic if the vertex does not exist in the graph but this
    /// should not be possible.
    fn weight_of_vertex(&self, id: ID) -> T {
        for i in 0..self.vertices.len() {
            let Vertex {id: my_id, weight: ref my_weight} = self.vertices[i];
            if my_id == id {
                return my_weight.clone();
            }
        }
        panic!("Vertex does not exist")
    }

    /// Returns the greatest value in a `Vec<T>`
    fn highest(v: Vec<Option<T>>) -> Option<T> {
        let mut max: Option<T> = None;
        for i in v {
            max = if max.cmp(&i) == Ordering::Greater { max } else { i };
        }
        max
    }

    /// A function that checks wether the graph is cyclic or not.
    ///
    /// Returns true if the graph is cyclic, false if not.
    fn check_cyclicity(target :u64, current :u64, edges :&Vec<Edge<T>>) -> bool {
        for i in 0..edges.len() {
            if edges[i].from == current && edges[i].to == target {
                return true;
            } else {
                if edges[i].from == current {
                    if DAG::check_cyclicity(target, edges[i].to, edges) {
                        return true;
                    }
                }
            }
        }
        false
    }

}

impl <T:Clone+Add<Output=T>+Ord> DAGInterface<T> for DAG<T> {

    fn add_vertex(&mut self, w: T) -> ID {
        self.vertices.push(Vertex{id: self.next_id, weight: w});
        self.next_id += 1;
        self.next_id-1
    }

    fn add_edge(&mut self, a: ID, b: ID, w: T) -> Result<bool, &'static str> {
        self.edges.push(Edge{from: a, to: b, weight: w});
        if a == b {
            self.edges.pop();
            return Err("Cannot add edge to itself");
        }

        if a >= self.next_id || b >= self.next_id {
            return Err("Cannot add an egde between non-existing vertices");
        }

        if DAG::check_cyclicity(a,b,&self.edges) {
            self.edges.pop();
            return Err("Cannot add edge which creates a cycle");
        }

        Ok(true)
    }

    fn topological_order(&self) -> Result<Vec<ID>, &'static str> {
        let mut result: Vec<ID> = Vec::new();
        let mut no_incomming: Vec<Vertex<T>> = Vec::new();
        let mut remaining_v: Vec<Vertex<T>> = self.vertices.clone();
        let mut remaining_e: Vec<Edge<T>> = self.edges.clone();

        let (mut hey, remaining) = DAG::split_remaining_v(remaining_v, &remaining_e);
        remaining_v = remaining;

        no_incomming.append(&mut hey);

        while let Some(current_node) = no_incomming.pop() {
            let Vertex{id: my_id, weight: _} = current_node;
            result.push(my_id);
            let mut i = 0;
            while i < remaining_e.len() {
                let Edge { from: origin, to: _, weight: _} = remaining_e[i];
                if origin == my_id {
                    remaining_e.swap_remove(i);
                    continue;
                }
                i += 1;
            }
            let (mut hey, remaining) = DAG::split_remaining_v(remaining_v, &remaining_e);
            remaining_v = remaining;
            no_incomming.append(&mut hey);
        }

        if !remaining_e.is_empty() {
            return Err("There exists a cycle!");
        }

        Ok(result)
    }


    fn weight_of_longest_path<F1, F2>(&self, from: ID, to: ID, v_sum: &F1, e_sum: &F2) -> Option<T>
        where F1: Fn(T) -> T,
              F2: Fn(T) -> T {

        let mut sum: T = self.weight_of_vertex(from);

        if  from == to {
            return Some(sum);
        }

        let mut weights: Vec<Option<T>> = Vec::new();

        for i in 0..self.edges.len() {
            let Edge {from: a, to: b, ref weight} = self.edges[i];
            if a == from {
                match self.weight_of_longest_path(b, to, v_sum, e_sum) {
                    Some(w) => weights.push(Some(weight.clone() + w)),
                    None    => (),
                }
            }
        }

        if let Some(high) = DAG::highest(weights) {
            sum = sum + high;
        } else {
            return None;
        }

        Some(sum)
    }
}
        

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dag: DAG<u8> = DAG::new();
        assert!(dag.vertices.is_empty());
        assert!(dag.edges.is_empty());
    }

    #[test]
    fn test_add_vertex() {
        let mut dag: DAG<u8> = DAG::new();
        let id = dag.add_vertex(8);

        assert_eq!(1, dag.vertices.len());
        assert_eq!(Some(Vertex {id: id, weight: 8}), dag.vertices.pop());
    }

    #[test]
    fn test_add_edge() {
        let mut dag: DAG<u8> = DAG::new();
        let a = dag.add_vertex(5);
        let b = dag.add_vertex(8);

        let _ = dag.add_edge(a, b, 10);
        assert_eq!(1, dag.edges.len());

        let tmp = Edge { from: a, to: b, weight: 10 };
        assert_eq!(tmp, dag.edges.pop().unwrap());
    }

    #[test]
    fn test_cyclicity_1() {
        let mut dag: DAG<u8> = DAG::new();
        let a = dag.add_vertex(1);
        let b = dag.add_vertex(1);
        let c = dag.add_vertex(1);

        let _ = dag.add_edge(a,b,2);
        let _ = dag.add_edge(b,c,3);
        if let Ok(_) = dag.add_edge(c,a,5) {
            panic!("Returned ok when breaking cyclicity");
        }
    }

    #[test]
    fn test_cyclicity_2() {
        let mut dag: DAG<u8> = DAG::new();
        let a = dag.add_vertex(1);
        let b = dag.add_vertex(1);
        let c = dag.add_vertex(1);
        let d = dag.add_vertex(1);
        let e = dag.add_vertex(1);
        let f = dag.add_vertex(1);
        let g = dag.add_vertex(1);
        let h = dag.add_vertex(1);

        let _ = dag.add_edge(a,b,1);
        let _ = dag.add_edge(b,d,1);
        let _ = dag.add_edge(b,e,1);
        let _ = dag.add_edge(c,b,1);
        let _ = dag.add_edge(d,g,1);
        let _ = dag.add_edge(e,f,1);
        let _ = dag.add_edge(e,h,1);
        let _ = dag.add_edge(g,f,1);
        let _ = dag.add_edge(g,h,1);

        if let Ok(_) = dag.add_edge(h,c,1) {
            panic!("Returned ok when breaking acyclicity");
        }

    }

    #[test]
    fn test_edge_to_self() {
        let mut dag: DAG<u8> = DAG::new();
        let a = dag.add_vertex(5);

        //Err is the desired result from add.
        if let Ok(_) = dag.add_edge(a,a,19) {
            panic!("Should return error on edge to itself");
        }
    }

    #[test]
    fn test_edge_to_nothing() {
        let mut dag: DAG<u8> = DAG::new();
        let a = dag.add_vertex(1);

        if let Ok(_) = dag.add_edge(a+1, a, 1) {
            panic!("Should not add edge from nothing");
        }

        if let Ok(_) = dag.add_edge(a, a+1, 1) {
            panic!("Should not add edge to nothing");
        }
    }

    #[test]
    fn test_topological() {
        let mut dag: DAG<u8> = DAG::new();
        let a = dag.add_vertex(5);
        let b = dag.add_vertex(8);

        let _ = dag.add_edge(a, b, 10);
        assert_eq!(Ok(vec![a,b]), dag.topological_order()); 

        let c = dag.add_vertex(10);
        
        let _ = dag.add_edge(c, a, 5);
        let _ = dag.add_edge(c, b, 7);
        assert_eq!(Ok(vec![c,a,b]), dag.topological_order());
    }

    #[test]
    fn test_longest_path() {
        let mut dag: DAG<u8> = DAG::new();
        let func = &|i| i;
        let a = dag.add_vertex(5);
        let b = dag.add_vertex(8);

        let _ = dag.add_edge(a, b, 10);
        assert_eq!(23, dag.weight_of_longest_path(a, b, func, func).unwrap());

        let c = dag.add_vertex(2);
        let _ = dag.add_edge(a, c, 5);
        let _ = dag.add_edge(c, b, 5);

        assert_eq!(25, dag.weight_of_longest_path(a, b, func, func).unwrap());

        assert_eq!(None, dag.weight_of_longest_path(c, a, func, func));
    }
}