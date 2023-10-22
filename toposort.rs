use topo_sort::{SortResults, TopoSort};

fn main() {
    let mut topo_sort = TopoSort::with_capacity(5);
    // read as "C" depends on "A" and "B"
    topo_sort.insert("C", vec!["A", "B"]);
    topo_sort.insert("E", vec!["B", "C"]);
    topo_sort.insert("A", vec![]);
    topo_sort.insert("D", vec!["A", "C", "E"]);
    topo_sort.insert("B", vec!["A"]);

    match topo_sort.into_vec_nodes() {
        SortResults::Full(nodes) => assert_eq!(vec!["A", "B", "C", "E", "D"], nodes),
        SortResults::Partial(_) => panic!("unexpected cycle!"),
    }
}
