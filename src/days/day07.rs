use crate::days::Problem;

use std::collections::HashMap;

pub struct Day07 {}

impl Problem for Day07 {
    fn part_one(&self, input: &str) -> String {
        let mut tree: Vec<TreeNode> = vec![];
        tree.push(TreeNode::new("/", 0, NodeType::Directory, 0));
        let mut index = 0;
        for line in input.lines() {
            let output = line.split(" ").collect::<Vec<&str>>();

            match output[0] {
                "$" => {
                    match output[1] {
                        "cd" => {
                            if output[2] != "/" {
                                if output[2] == ".." {
                                    index = tree[index].parent;
                                } else {
                                    index = tree[index].children[&String::from(output[2])];
                                }
                            }
                        }
                        _ => { /* do nothing */ }
                    }
                }
                "dir" => {
                    tree.push(TreeNode::new(output[1], 0, NodeType::Directory, index));
                    let spot = tree.len() - 1;
                    tree[index].children.insert(String::from(output[1]), spot);
                }
                _ => {
                    tree.push(TreeNode::new(
                        output[1],
                        output[0].parse::<u32>().unwrap(),
                        NodeType::File,
                        index,
                    ));
                    let spot = tree.len() - 1;
                    tree[index].children.insert(String::from(output[1]), spot);
                }
            }
        }

        let mut tree_object = Tree { array: tree };

        let total_size = tree_object.calculate_size();

        let mut sum = 0;

        for node in tree_object.array.iter() {
            match node.node_type {
                NodeType::Directory => {
                    if node.size < 100000 {
                        sum += node.size;
                    }
                }
                NodeType::File => {
                    sum += 0;
                }
            }
        }

        sum.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut tree: Vec<TreeNode> = vec![];
        tree.push(TreeNode::new("/", 0, NodeType::Directory, 0));
        let mut index = 0;
        for line in input.lines() {
            let output = line.split(" ").collect::<Vec<&str>>();

            match output[0] {
                "$" => {
                    match output[1] {
                        "cd" => {
                            if output[2] != "/" {
                                if output[2] == ".." {
                                    index = tree[index].parent;
                                } else {
                                    index = tree[index].children[&String::from(output[2])];
                                }
                            }
                        }
                        _ => { /* do nothing */ }
                    }
                }
                "dir" => {
                    tree.push(TreeNode::new(output[1], 0, NodeType::Directory, index));
                    let spot = tree.len() - 1;
                    tree[index].children.insert(String::from(output[1]), spot);
                }
                _ => {
                    tree.push(TreeNode::new(
                        output[1],
                        output[0].parse::<u32>().unwrap(),
                        NodeType::File,
                        index,
                    ));
                    let spot = tree.len() - 1;
                    tree[index].children.insert(String::from(output[1]), spot);
                }
            }
        }

        let mut tree_object = Tree { array: tree };

        tree_object.calculate_size();

        let available = 70000000 - tree_object.array[0].size;
        let needed = 30000000 - available;

        let mut possible: Vec<&TreeNode> = vec![];

        for e in tree_object.array.iter() {
            if e.size > needed && e.node_type == NodeType::Directory {
                possible.push(e);
            }
        }

        let mut min = &possible[0];

        for e in possible.iter() {
            if e.size < min.size {
                min = e;
            }
        }

        min.size.to_string()
    }
}

#[derive(Debug, PartialEq)]
enum NodeType {
    File,
    Directory,
}

#[derive(Debug)]
struct TreeNode {
    name: String,
    size: u32,
    node_type: NodeType,
    parent: usize,
    children: HashMap<String, usize>,
}

impl TreeNode {
    pub fn new(name: &str, size: u32, node_type: NodeType, parent: usize) -> Self {
        Self {
            name: String::from(name),
            size,
            node_type,
            parent,
            children: HashMap::new(),
        }
    }
}

struct Tree {
    array: Vec<TreeNode>,
}

impl Tree {
    pub fn calculate_size(&mut self) {
        let mut first_stack = vec![];
        let mut second_stack = vec![];

        first_stack.push(0);

        while !first_stack.is_empty() {
            let index = first_stack.pop().unwrap();
            for child in self.array[index].children.iter() {
                first_stack.push(*child.1);
            }
            second_stack.push(index);
        }

        while !second_stack.is_empty() {
            let index = second_stack.pop().unwrap();
            if self.array[index].name != String::from("/") {
                let parent_index = self.array[index].parent;
                let size = self.array[index].size;
                self.array[parent_index].size += size;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "95437",
            Day07 {}.part_one(&util::input(7, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "24933642",
            Day07 {}.part_two(&util::input(7, util::InputType::Example))
        )
    }
}
