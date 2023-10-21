fn main() {
    let s = get_input();
    let nodes = parse(&s);
    println!("{}", nodes[0].visualize("", &nodes));
    println!("(total, smol_total, rm_size) = {:?}", solve(&nodes));
}

fn solve(nodes: &Vec<Node>) -> (usize, usize, usize) {
    let mut folder_sizes: Vec<usize> = vec![];

    let total = compute_folder_size(0, nodes, &mut folder_sizes);
    let unused = 70000000 - total;
    let needed = 30000000 - unused;

    folder_sizes.sort();

    let smol_total = folder_sizes
        .iter()
        .filter(|&&size| size <= 100000)
        .sum::<usize>();

    let rm_size = *folder_sizes.iter().find(|&&s| s >= needed).unwrap();

    (total, smol_total, rm_size)
}

fn compute_folder_size(curr: usize, nodes: &Vec<Node>, folder_sizes: &mut Vec<usize>) -> usize {
    let mut accum = 0;
    for &i in nodes[curr].children.iter() {
        if nodes[i].is_folder {
            let folder_size = compute_folder_size(i, nodes, folder_sizes);
            folder_sizes.push(folder_size);
            accum += folder_size;
        } else {
            accum += nodes[i].size;
        }
    }

    accum
}

// There's redundancy in here, but, it's pragmatic
#[derive(PartialEq, Debug)]
struct Node<'a> {
    is_folder: bool,
    parent: usize,
    name: &'a str,
    size: usize,
    children: Vec<usize>,
}

impl<'a> Node<'a> {
    fn new_folder(parent: usize, name: &'a str) -> Self {
        Self {
            is_folder: true,
            parent,
            name,
            size: 0,
            children: vec![],
        }
    }

    fn new_file(parent: usize, name: &'a str, size: usize) -> Self {
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

fn parse<'a>(s: &'a str) -> Vec<Node<'a>> {
    let mut nodes = vec![Node::new_folder(0, "/")];
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
            let dir = Node::new_folder(curr, &line[4..]);
            let i = nodes.len();
            nodes.push(dir);
            nodes[curr].children.push(i);
        } else {
            let (size, name) = line.split_once(" ").unwrap();
            let file = Node::new_file(curr, name, size.parse::<usize>().unwrap());
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

    assert_eq!((48381165, 95437, 24933642), solve(&nodes));
}

fn get_input() -> String {
    dotenv::dotenv().ok();
    let key = std::env::var("KEY").expect("Missing env var KEY");

    let bytes = std::fs::read("./input.txt.encrypted").unwrap();
    decrypt(key.as_bytes(), &bytes)
}

fn decrypt(key: &[u8], enc: &[u8]) -> String {
    String::from_utf8(
        enc.iter()
            .enumerate()
            .map(|(i, &b)| b.wrapping_sub(key[i % key.len()]))
            .collect(),
    )
    .unwrap()
}
