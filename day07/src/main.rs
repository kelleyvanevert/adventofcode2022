use std::fs;

fn main() {
    let s = fs::read_to_string("./input.txt").unwrap();
    let nodes = parse(&s);
    println!("{}", nodes[0].visualize("", &nodes));
    println!("total of folders up to 100000: {}", solve(&nodes));
}

fn solve(nodes: &Vec<Node>) -> usize {
    let mut smol: Vec<usize> = vec![];

    compute_folder_size(0, nodes, &mut smol);

    smol.iter().sum::<usize>()
}

fn compute_folder_size(curr: usize, nodes: &Vec<Node>, smol: &mut Vec<usize>) -> usize {
    let mut accum = 0;
    for &i in nodes[curr].children.iter() {
        if nodes[i].is_folder {
            let folder_size = compute_folder_size(i, nodes, smol);
            if folder_size <= 100000 {
                smol.push(folder_size);
            }
            accum += folder_size;
        } else {
            accum += nodes[i].size;
        }
    }

    accum
}

#[derive(PartialEq, Debug)]
struct Node {
    is_folder: bool,
    parent: usize,
    name: String,
    size: usize,
    children: Vec<usize>,
}

impl Node {
    fn folder(parent: usize, name: String) -> Self {
        Self {
            is_folder: true,
            parent,
            name,
            size: 0,
            children: vec![],
        }
    }

    fn file(parent: usize, name: String, size: usize) -> Self {
        Self {
            is_folder: false,
            parent,
            name,
            size,
            children: vec![],
        }
    }

    fn visualize(&self, indent: &str, nodes: &Vec<Node>) -> String {
        if self.is_folder {
            let child_indent = &format!("  {}", indent);
            format!(
                "{}- {} (dir)\n{}",
                indent,
                self.name,
                self.children
                    .iter()
                    .map(|&i| nodes[i].visualize(child_indent, nodes))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        } else {
            format!("{}- {} (file, size={})", indent, self.name, self.size)
        }
    }
}

fn parse(s: &str) -> Vec<Node> {
    let mut nodes = vec![Node::folder(0, "/".to_string())];
    let mut curr = 0;

    for line in s.lines() {
        if line.starts_with("$ cd ") {
            match &line[5..] {
                "/" => {
                    curr = 0;
                }
                ".." => {
                    curr = nodes[curr].parent;
                }
                folder_name => {
                    curr = *nodes[curr]
                        .children
                        .iter()
                        .find(|&&i| nodes[i].name == folder_name)
                        .expect("could not find child folder with name");
                }
            }
        } else if line.starts_with("$ ls") {
            // noop
        } else if line.starts_with("dir ") {
            let dir = Node::folder(curr, line[4..].to_string());
            let i = nodes.len();
            nodes.push(dir);
            nodes[curr].children.push(i);
        } else {
            let (size, name) = line.split_once(" ").unwrap();
            let file = Node::file(curr, name.to_string(), size.parse::<usize>().unwrap());
            let i = nodes.len();
            nodes.push(file);
            nodes[curr].children.push(i);
        }
    }

    nodes
}

#[test]
fn test_all() {
    let s = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    let nodes = parse(s);

    assert_eq!(
        "- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)"
            .to_string(),
        nodes[0].visualize("", &nodes)
    );

    assert_eq!(95437, solve(&nodes));
}
