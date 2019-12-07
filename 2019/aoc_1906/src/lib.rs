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
use std::collections::HashMap;
use vec_tree::{Index, VecTree};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    name: String,
}

impl Node {
    fn new(name: String) -> Node {
        Node { name }
    }
}

struct Tree {
    tree: VecTree<Node>,
    pending_nodes: HashMap<String, Vec<String>>,
}

impl Tree {
    fn new() -> Tree {
        let mut tree = VecTree::new();
        tree.insert_root(Node::new("COM".to_string()));
        Tree {
            tree,
            pending_nodes: HashMap::new(),
        }
    }

    fn from_str(s: &str) -> Tree {
        let mut t = Tree::new();
        for line in s.lines() {
            let line_parts: Vec<&str> = line.split(')').collect();
            assert_eq!(line_parts.len(), 2);

            t.add_by_str(line_parts[0].trim(), line_parts[1].trim());
        }
        t
    }

    fn add_by_str(&mut self, parent: &str, child: &str) -> Option<Index> {
        assert!(self.find_by_str(child) == None);
        if let Some(parent_idx) = self.find_by_str(parent) {
            let idx = self.tree.insert(Node::new(child.to_string()), parent_idx);

            if let Some(grandchildren) = self.pending_nodes.remove(child) {
                for grandchild in grandchildren {
                    self.add_by_str(child, &grandchild).unwrap(); // It's a logic error if this doesn't work
                }
            }
            return Some(idx);
        } else if let Some(sibling_vec) = self.pending_nodes.get_mut(parent) {
            sibling_vec.push(child.to_string());
        } else {
            self.pending_nodes
                .insert(parent.to_string(), vec![child.to_string()]);
        }
        None
    }

    fn find_by_str(&self, name: &str) -> Option<Index> {
        self.iter()?.find(|i| self.tree[*i].name == name)
    }

    fn count_ancestors(&self, idx: Index) -> usize {
        self.tree.ancestors(idx).count() - 1 // don't include self
    }

    fn iter(&self) -> Option<impl Iterator<Item = Index> + '_> {
        Some(self.tree.descendants(self.tree.get_root_index()?))
    }

    fn tot_orbits(&self) -> usize {
        if self.tree.get_root_index() == None {
            return 0;
        }
        self.iter()
            .unwrap()
            .map(|i: Index| self.count_ancestors(i))
            .sum()
    }

    fn distance_to(&self, src: Index, dst: Index, prev_node: Option<Index>) -> Option<usize> {
        if src == dst {
            return Some(0);
        }
        for test in self.tree.children(src) {
            if prev_node.is_none() || prev_node.unwrap() != test {
                if let Some(res) = self.distance_to(test, dst, Some(src)) {
                    return Some(res + 1);
                }
            }
        }
        if let Some(parent) = self.tree.parent(src) {
            if prev_node.is_none() || prev_node.unwrap() != parent {
                return self.distance_to(parent, dst, Some(src)).map(|n| n + 1);
            }
        }

        None
    }

    fn orbit_xfer(&self, src_str: &str, dst_str: &str) -> Option<usize> {
        let src = self.tree.parent(self.find_by_str(src_str)?)?;
        let dst = self.tree.parent(self.find_by_str(dst_str)?)?;
        self.distance_to(src, dst, None)
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

    use permutohedron::Heap;

    #[test]
    fn part1_permute() {
        let mut lines: Vec<&str> = "\
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
K)L"
        .lines()
        .collect();
        let gold_t = Tree::from_str(&lines.join("\n"));
        let mut gold_t_nodes: Vec<(u32, &str)> = gold_t
            .tree
            .descendants_with_depth(gold_t.tree.get_root_index().unwrap())
            .map(|(a, b)| (b, gold_t.tree[a].name.as_str()))
            .collect();
        gold_t_nodes.sort();

        let heap = Heap::new(&mut lines);
        const NUM_TESTS: usize = 1000;
        for combo in heap.take(NUM_TESTS) {
            let combo_str = combo.join("\n");
            let t = Tree::from_str(&combo_str);
            let mut t_nodes: Vec<(u32, &str)> = t
                .tree
                .descendants_with_depth(t.tree.get_root_index().unwrap())
                .map(|(a, b)| (b, t.tree[a].name.as_str()))
                .collect();
            t_nodes.sort();
            assert!(t.pending_nodes.is_empty());
            assert_eq!(gold_t_nodes, t_nodes);
            assert_eq!(t.count_ancestors(t.find_by_str("D").unwrap()), 3);
            assert_eq!(t.count_ancestors(t.find_by_str("L").unwrap()), 7);
            assert_eq!(t.count_ancestors(t.find_by_str("COM").unwrap()), 0);
            assert_eq!(t.tot_orbits(), 42);
        }
    }

    #[test]
    fn part1_prob() {
        let t = Tree::from_str(include_str!("test_input.txt"));
        assert_eq!(t.tot_orbits(), 249308);
    }

    #[test]
    fn part2_eg() {
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
K)L
K)YOU
I)SAN";
        let t = Tree::from_str(SAMPLE);
        assert_eq!(t.orbit_xfer("YOU", "SAN"), Some(4));
    }
    #[test]
    fn part2_prob() {
        let t = Tree::from_str(include_str!("test_input.txt"));
        assert_eq!(t.orbit_xfer("YOU", "SAN"), Some(349));
    }
}
