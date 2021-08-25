use std::collections::{HashSet, HashMap, VecDeque};
use advent_2020_common::Error;
#[macro_use]
extern crate lazy_static;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BAG_GRAPH_REGEX: Regex = Regex::new("(\\d+? )?(\\w+? \\w+?) bag(s?)").unwrap();
}

// to: the vertex ID that can contain us
// count: how many of us fit inside the linked vertex
struct Edge{
    to: usize,
    count: usize,
}

// directed graph edges, map from index in vertices list to the 'edges' - vertex id that contains us, and how many
type VertexId = usize;
type Edges = HashMap<VertexId, Vec<Edge>>;

// vertex is a string name of a bag color - e.g. 'light red', 'bright white'
type Vertex = String;

type Vertices = Vec<Vertex>;

pub struct DirectedBagsGraph{
    edges: Edges,
    vertices: Vertices,
}

impl DirectedBagsGraph {
    pub fn new(input: &[String]) -> Result<DirectedBagsGraph, Error> {
        let mut edges = Edges::new();
        let mut vertices = Vertices::new();
        // let mut new_self = DirectedBagsGraph{edges, vertices};

        let mut existing_index_or_insert = |color_name: &str| {
            let index = vertices.iter().position(|s| *s == *color_name);
            // let index = new_self.vertex_id_named(color_name);
            match index {
                Some(i) => i,
                _ => { 
                    let i = vertices.len();
                    vertices.push(String::from(color_name));
                    i
                }
            }
        };

        let mut insert_edge = |index: usize, new_edge: Edge| {
            match edges.get_mut(&index) {
                Some(vec) => vec.push(new_edge),
                _ => {
                    let new_vec = vec!(new_edge);
                    edges.insert(index, new_vec);
                }
            }
        };

        for line in input.iter() {
            let mut captures = BAG_GRAPH_REGEX.captures_iter(line).peekable();
            if captures.peek().is_none() { 
                return Error::from_string(format!("invalid line {}", line));
            }
           
            let mut to_index = 0;

            for (i, capture) in captures.enumerate() {
                match i {
                    0 => to_index = existing_index_or_insert(&capture[2]),
                    _ if capture[0].contains("no") => (),
                    _ => {
                        let from_index = existing_index_or_insert(&capture[2]);
                        let count: usize = capture.get(1)
                            .map(|s| s.as_str().trim()).unwrap_or("")
                            .parse()
                            .or(Error::new("couldn't parse count"))?;
                        insert_edge(from_index, Edge{to: to_index, count});
                    }
                }
            }
        }
       
        Ok(DirectedBagsGraph{edges, vertices})
    }

    fn vertex_id_named(&self, name: &str) -> Option<usize> {
        self.vertices.iter().position(|s| *s == *name)
    }

    fn edges_for_vertex_id(&self, id: VertexId) -> Option<&Vec<Edge>> {
        self.edges.get(&id)
    }
}

pub fn first(graph: &DirectedBagsGraph, target_bag: &String) -> Result<usize, Error> {
    let start_vertex_id = graph.vertex_id_named(target_bag).unwrap();
    let mut queue = VecDeque::new();
    queue.push_back(start_vertex_id);
    let mut seen: HashSet<usize> = HashSet::new();

    while queue.len() > 0 {
        let next: usize = queue.pop_front().unwrap();
        
        if seen.get(&next).is_some() {
            continue;
        }
        seen.insert(next);

        if let Some(edges) = graph.edges_for_vertex_id(next) {
            for edge in edges.iter() {
                queue.push_front(edge.to);
            }
        }
    }

    Ok(seen.len() - 1) // -1 for the original item
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!(
            String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            String::from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
            String::from("bright white bags contain 1 shiny gold bag."),
            String::from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
            String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
            String::from("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
            String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
            String::from("faded blue bags contain no other bags."),
            String::from("dotted black bags contain no other bags."),
        )
    }

    #[test]
    fn test_first() {
        let input = DirectedBagsGraph::new(&example()).unwrap();
        let result = first(&input, &String::from("shiny gold")).unwrap();

        assert_eq!(result, 4);
    }

    #[test]
    fn test_regex() {
        let ex = &example()[0];
        let matches = BAG_GRAPH_REGEX.is_match(ex);
        assert!(matches);

        let mut count = 0;
        for (i, capture) in BAG_GRAPH_REGEX.captures_iter(ex).enumerate() {
            count = count + 1;
            println!("Got a capture {:?}", capture);
            match i {
                0 => {
                    assert_eq!(capture.get(1), None);
                    assert_eq!(&capture[2], "light red");
                },
                1 => {
                    assert_eq!(&capture[1], "1 ");
                    assert_eq!(&capture[2], "bright white");
                },
                2 => {
                    assert_eq!(&capture[1], "2 ");
                    assert_eq!(&capture[2], "muted yellow");
                },
                _ => (),
            }
        }
        assert_eq!(count, 3);
    }
    #[test]
    fn test_parse_graph() {
        let result = DirectedBagsGraph::new(&example()).unwrap();

        let my_i = result.vertex_id_named("muted yellow").unwrap();

        let lr_i = result.vertex_id_named("light red").unwrap();

        let contains_us = result.edges.get(&my_i).unwrap();

        let light_red_edge = contains_us.iter().filter(|&Edge{to, count}| {
            *to == lr_i
        }).next().unwrap().count;
        assert_eq!(light_red_edge, 2);

        let do_i = result.vertex_id_named("dark orange").unwrap();
        let dark_orange_edge = contains_us.iter().filter(|&Edge{to, count}| {
            *to == do_i
        }).next().unwrap().count;
        assert_eq!(dark_orange_edge, 4);
    }
}