#![allow(dead_code)]
/*
--- Day 6: Universal Orbit Map ---

You've landed at the Universal Orbit Map facility on Mercury. Because
navigation in space often involves transferring between orbits, the orbit
maps here are useful for finding efficient routes between, for example, you
and Santa. You download a map of the local orbits (your puzzle input).

Except for the universal Center of Mass (COM), every object in space is in
orbit around exactly one other object. An orbit looks roughly like this:
                  \
                   \
                    |
                    |
AAA--> o            o <--BBB
                    |
                    |
                   /
                  /

In this diagram, the object BBB is in orbit around AAA. The path that BBB
takes around AAA (drawn with lines) is only partly shown. In the map data,
this orbital relationship is written AAA)BBB, which means "BBB is in orbit
around AAA".

Before you use your map data to plot a course, you need to make sure it
wasn't corrupted during the download. To verify maps, the Universal Orbit Map
facility uses orbit count checksums - the total number of direct orbits (like
the one shown above) and indirect orbits.

Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain
can be any number of objects long: if A orbits B, B orbits C, and C orbits D,
then A indirectly orbits D.

For example, suppose you have the following map:

COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L

Visually, the above map of orbits looks like this:

        G - H       J - K - L
       /           /
COM - B - C - D - E - F
               \
                I

In this visual representation, when two objects are connected by a line, the
one on the right directly orbits the one on the left.

Here, we can count the total number of orbits as follows:

    D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
    L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
    COM orbits nothing.

The total number of direct and indirect orbits in this example is 42.

What is the total number of direct and indirect orbits in your map data?

*/
use vec_tree::{Index, VecTree};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node { name: String }

impl Node {
    fn new(name: String) -> Node {
        Node {name: name}
    }
}

struct Tree {
    tree: VecTree<Node>,
    pending_nodes: HashMap<String, Vec<String>>
}

impl Tree {
    fn new() -> Tree {
        let mut tree = VecTree::new();
        tree.insert_root(Node::new("COM".to_string()));
        Tree{tree: tree, pending_nodes: HashMap::new()}
    }

    fn from_str(s: &str) -> Tree {
        let mut t = Tree::new();
        for line in s.lines() {
            let line_parts: Vec<&str> = line.split(')').collect();
            assert_eq!(line_parts.len(), 2);

            t.add_by_str(line_parts[0].trim(), line_parts[1].trim());
        }
        //dbg!(t.tree.clone());
        //dbg!(t.pending_nodes.clone());
        t
    }

    fn add_by_str(&mut self, parent: &str, child: &str) -> Option<Index> {
        assert!(self.find_by_str(child) == None);
        if let Some(parent_idx) = self.find_by_str(parent){
            let idx = self.tree.insert(Node::new(child.to_string()), parent_idx);

            if let Some(grandchildren) = self.pending_nodes.remove(child) {
                for grandchild in grandchildren {
                    self.add_by_str(parent, &grandchild).unwrap(); // It's a logic error if this doesn't work
                }
            }
            return Some(idx);
        } else if let Some(sibling_vec) = self.pending_nodes.get_mut(parent) {
            sibling_vec.push(child.to_string());
        } else {
            self.pending_nodes.insert(parent.to_string(), vec!(child.to_string()));
        }
        None
    }

    fn find_by_str(&self, name: &str) -> Option<Index> {
        self.iter()?.find(|i| self.tree[*i].name == name)
    }

    fn count_ancestors(&self, idx: Index) -> usize {
        self.tree.ancestors(idx).count() - 1 // don't include self
    }

    fn iter(&self) -> Option<impl Iterator<Item=Index> + '_> {
        Some(self.tree.descendants(self.tree.get_root_index()?))
    }

    fn tot_orbits(&self) -> usize {
        if self.tree.get_root_index() == None {
            return 0;
        }
        self.iter().unwrap().map(|i: Index| self.count_ancestors(i)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_eg() {
       const SAMPLE: &str = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let t = Tree::from_str(SAMPLE);
        assert_eq!(t.count_ancestors(t.find_by_str("D").unwrap()), 3);
        assert_eq!(t.count_ancestors(t.find_by_str("L").unwrap()), 7);
        assert_eq!(t.count_ancestors(t.find_by_str("COM").unwrap()), 0);
        assert_eq!(t.tot_orbits(), 42);
    }
    #[test]
    fn part1_prob() {
        let _t = Tree::from_str(include_str!("test_input.txt"));
        //assert_eq!(t.tot_orbits(), 0);
    }
}
