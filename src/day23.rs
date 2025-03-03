use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};

use petgraph::graph::{NodeIndex, Graph};
//use petgraph::dot::{Dot, Config};

fn main() {
    let g = parse_input("data/input_23.txt");
    //println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    //println!("{:?}", g);

    let mut triples = HashSet::<(NodeIndex, NodeIndex, NodeIndex)>::new();
    for n0 in g.node_indices() {
        for n1 in g.neighbors(n0) {
            for n2 in g.neighbors(n1) {
                for n3 in g.neighbors(n2) {
                    if n0 == n3 {
                        if n0 < n1 && n1 < n2 {
                            triples.insert((n0, n1, n2));
                        }
                        break;
                    }
                }
            }
        }
    }

    let mut total: usize = 0;
    for triplet in triples {
        if g.node_weight(triplet.0).unwrap().chars().nth(0).unwrap() == 't' 
        || g.node_weight(triplet.1).unwrap().chars().nth(0).unwrap() == 't' 
        || g.node_weight(triplet.2).unwrap().chars().nth(0).unwrap() == 't' {
            total += 1;
        }
    }
    println!("Total Triplets: {total}");

    let maximal_cliques = bron_kerbosch(&g);
    let mut maximal_cliques: Vec<_>  = maximal_cliques.iter().collect();
    maximal_cliques.sort_by(|a, b| a.len().cmp(&b.len()));

    let mut names: Vec<_> = maximal_cliques[maximal_cliques.len() - 1].iter().map(|&n| g.node_weight(n).unwrap().to_string()).collect();
    names.sort();
    let password = names.join(",");
    println!("Password: {}", password);
}

fn bron_kerbosch(g: &Graph<String, (), petgraph::Undirected>) -> Vec<HashSet<NodeIndex>> {
    let mut cliques = Vec::<HashSet<NodeIndex>>::new();
    let r = HashSet::<NodeIndex>::new();
    let p: HashSet<NodeIndex> = g.node_indices().collect();
    let x = HashSet::<NodeIndex>::new();

    bron_kerbosch_recurr(r, p, x, &mut cliques, &g);
    return cliques;
}

fn bron_kerbosch_recurr(
    r: HashSet<NodeIndex>,
    mut p: HashSet<NodeIndex>,
    mut x: HashSet<NodeIndex>,
    c: &mut Vec<HashSet<NodeIndex>>,
    g: &Graph<String, (), petgraph::Undirected>) {

    if p.len() == 0 && x.len() == 0 {
        c.push(r.clone());
    }

    while p.len() > 0{
        let v = p.iter().next().unwrap().clone();
        let n_v: HashSet<NodeIndex> = g.neighbors(v).collect();
        bron_kerbosch_recurr(
            r.union(&HashSet::from([v])).map(|&n| n).collect(),
            p.intersection(&n_v).map(|&n| n).collect(),
            x.intersection(&n_v).map(|&n| n).collect(),
            c,
            g,
        );
        p.remove(&v);
        x.insert(v);
    }
}


fn parse_input(fname: &str) -> Graph<String, (), petgraph::Undirected> {
    let mut lut = HashMap::<&str, NodeIndex>::new();
    let mut g = Graph::<String, (), petgraph::Undirected>::new_undirected();

    for line in read_to_string(fname).unwrap().lines() {
        let fields: Vec<&str> = line.split("-").collect();

        let n1: NodeIndex;
        if lut.contains_key(fields[0]) {
            n1 = *lut.get(fields[0]).unwrap();
        } else {
            n1 = g.add_node(fields[0].to_string());
            lut.insert(fields[0], n1);
        }
        
        let n2: NodeIndex;
        if lut.contains_key(fields[1]) {
            n2 = *lut.get(fields[1]).unwrap();
        } else {
            n2 = g.add_node(fields[1].to_string());
            lut.insert(fields[1], n2);
        }
        g.update_edge(n1, n2, ());
    }

    return g;
}
