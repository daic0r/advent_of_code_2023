use std::collections::{HashMap,HashSet};
use regex::Regex;

type Graph = HashMap<String, HashMap<String, usize>>;

struct VertexGroup<'a> {
    vertex_set: &'a Graph,
    vertices: HashSet<&'a str>
}

impl<'a> VertexGroup<'a> {
    fn new(graph: &'a Graph) -> Self {
        VertexGroup {
            vertex_set: graph,
            vertices: HashSet::new()
        }
    }
    fn edges(&self) -> HashMap<&'a str, usize> {
        self.vertex_set
            .iter()
            .filter(|(node, _)| self.vertices.contains(node.as_str()))
            .fold(HashMap::new(), |mut acc,x| {
                //acc.extend(x.1.iter().filter(|&&s| !self.vertices.contains(s)));
                x.1.iter()
                    .filter(|(s,_)| !self.vertices.contains(s.as_str()))
                    .for_each(|(s,w)| {
                        acc.entry(&s).and_modify(|m| *m+=*w).or_insert(*w);
                    });
                acc
            })
    }
}

fn merge_nodes(node1: &str, node2: &str, graph: &mut Graph) {
    let mut edges1 = std::mem::take(graph.get_mut(node1).unwrap());
    let mut edges2 = std::mem::take(graph.get_mut(node2).unwrap());
    edges1.retain(|s,_| *s != node2);
    edges2.retain(|s,_| *s != node1);
    edges2.iter().for_each(|(s,w)| {
        edges1.entry(s.clone()).and_modify(|this_w| *this_w += *w).or_insert(*w);
    });
    let compound_name = format!("{},{}", node1, node2);
    graph.insert(compound_name.clone(), edges1);
    graph.remove(node1);
    graph.remove(node2);
    let keys = graph.keys().map(|k| k.clone()).collect::<Vec<_>>();
    for key in keys {
        graph.entry(key.clone()).and_modify(|edges| {
            if edges.contains_key(node1) || edges.contains_key(node2) {
                let w1 = edges.get(node1).unwrap_or(&0);
                let w2 = edges.get(node2).unwrap_or(&0);
                edges.insert(compound_name.clone(), w1+w2);
                edges.remove(node1);
                edges.remove(node2);
            }
        });
    }
}

fn stoer_wagner(graph: &Graph) -> Vec<String> {
    let mut graph_copy = graph.clone();
    let mut cuts = vec![];
    let mut idx = 0;
    loop {
        let len = graph_copy.len();
        if len == 1 {
            break;
        }
        let mut last_insert: Option<&str> = None;
        let mut second_to_last_insert: Option<&str> = None;
        let mut last_weight: Option<usize> = None;
        let tmp = graph_copy.clone();
        let mut vg = VertexGroup::new(&tmp);
        while vg.vertices.len() < tmp.len() {
            let external_edges = if !vg.edges().is_empty() {
                vg.edges()
            } else {
                let mut ret = HashMap::new();
                ret.insert(tmp.iter().nth(0).unwrap().0.as_str(), 0usize);
                ret
            };
            let max_edge = external_edges.iter().max_by_key(|(_,w)| *w);
            //println!("Adding {}, weight {}", *max_edge.unwrap().0, *max_edge.unwrap().1);
            second_to_last_insert = last_insert;
            last_insert = Some(*max_edge.unwrap().0);
            last_weight = Some(*max_edge.unwrap().1);
            vg.vertices.insert(*max_edge.unwrap().0);
            //println!("Group after {}: {:?}", idx, vg.vertices);
        }
        //println!("{:?} and {:?}", last_insert.unwrap(),  second_to_last_insert.unwrap());
        cuts.push((vg.vertices.iter().map(|s| s.to_string()).collect::<Vec<_>>(), last_weight.unwrap(), last_insert.unwrap().to_string(), second_to_last_insert.unwrap().to_string(), idx));
        merge_nodes(&last_insert.unwrap(), &second_to_last_insert.unwrap(), &mut graph_copy);
        println!("Nodes in graph after {}: {}", idx, graph_copy.len());
        //println!("{:?}", graph_copy);
        //println!("---------------------------------------------");
        idx += 1;
        if last_weight.unwrap() == 3 {
            break;
        }
    }
    let min_cut = cuts.iter().min_by_key(|kvp| kvp.1).unwrap();
    println!("Minimum is index {}", min_cut.4);
    let group_one = min_cut.0.iter().max_by_key(|node_cluster| node_cluster.len()).unwrap();
    group_one.split(",").map(|s| s.to_string()).collect::<Vec<_>>()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    let mut adj_list: Graph = Graph::new();

    let rex = Regex::new(r"([a-z]{3}): (.+)").unwrap();
    for line in contents.lines() {
        let caps = rex.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let connected = caps.get(2).unwrap().as_str();
        let con_set = connected.split_whitespace().map(|s| s.to_string()).collect::<HashSet<_>>();
        for con in &con_set {
            adj_list.entry(con.to_owned()).and_modify(|s| { s.insert(name.to_owned(), 1); }).or_insert_with(|| {
                let mut ret = HashMap::new();
                ret.insert(name.to_owned(), 1);
                ret
            });
        }
        adj_list
            .entry(name.to_string())
            .and_modify(|s| s.extend(con_set.clone().into_iter().map(|s| (s,  1))))
            .or_insert(con_set.into_iter().map(|s| (s,  1)).collect());
    }

    println!("{}", adj_list.len());

    println!("{:?}", adj_list);
    let one_group = stoer_wagner(&adj_list);
    println!("Group: {:?}", one_group);
    let product = one_group.len() * (adj_list.len() - one_group.len());
    println!("Product: {}", product);
}
