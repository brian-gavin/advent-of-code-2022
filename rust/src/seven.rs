use std::{collections::BTreeMap, iter::Peekable};

#[derive(Debug, Default)]
struct DirView {
    entries: Vec<DirEntry>,
}

#[derive(Debug)]
enum DirEntry {
    Dir(String),
    File(File),
}

#[derive(Debug)]
struct File {
    _name: String,
    size: usize,
}

#[derive(Debug)]
enum Command {
    List(Vec<ListContents>),
    Cd(String),
}

#[derive(Debug)]
enum ListContents {
    Dir(String),
    File(File),
}

impl ListContents {
    fn parse(s: &str) -> ListContents {
        let s = s.split_whitespace().collect::<Vec<_>>();
        match s.as_slice() {
            &["dir", name] => Self::Dir(name.to_string()),
            &[size, name] => Self::File(File {
                size: size.parse().unwrap(),
                _name: name.to_string(),
            }),
            &[..] => panic!("bad ls content input: {:?}", s),
        }
    }
}

fn parse_list<I>(input: &mut Peekable<I>) -> Command
where
    I: Iterator<Item = String>,
{
    let mut v = vec![];
    while input.peek().map_or(false, |c| !c.starts_with('$')) {
        v.push(ListContents::parse(input.next().unwrap().as_str()));
    }
    Command::List(v)
}

fn parse_commands(input: impl Iterator<Item = String>) -> Vec<Command> {
    let mut v = vec![];
    let mut input = input.peekable();
    loop {
        let Some(cmd) = input.next() else { break };
        let cmd = cmd.split_whitespace().collect::<Vec<_>>();
        let cmd = match cmd.as_slice() {
            &["$", "ls"] => parse_list(&mut input),
            &["$", "cd", dir] => Command::Cd(dir.to_string()),
            &[..] => panic!("bad input: {:?}", cmd),
        };
        v.push(cmd);
    }
    v
}

fn parse_fs(input: Vec<String>) -> BTreeMap<String, DirView> {
    let commands = parse_commands(input.into_iter());
    let mut fs = BTreeMap::<String, DirView>::new();
    let mut cwd = "".to_string();
    for cmd in commands {
        match cmd {
            Command::List(v) => {
                let cwd = fs.entry(cwd.to_string()).or_default();
                cwd.entries.extend(v.into_iter().map(|lc| match lc {
                    ListContents::Dir(name) => DirEntry::Dir(name),
                    ListContents::File(file) => DirEntry::File(file),
                }))
            }
            Command::Cd(ref dir) => match dir.as_str() {
                ".." => {
                    cwd = cwd
                        .rsplit_once('/')
                        .unwrap_or_else(|| ("", ""))
                        .0
                        .to_string();
                }
                "/" => {
                    cwd = "/".to_string();
                }
                dir => {
                    if !cwd.ends_with('/') {
                        cwd.push('/');
                    }
                    cwd.push_str(dir);
                }
            },
        }
    }
    fs
}

fn normalize(parent: &str, dir: &str) -> String {
    if parent == "/" {
        [parent, dir].join("")
    } else {
        [parent, dir].join("/")
    }
}

fn sizes(fs: BTreeMap<String, DirView>) -> BTreeMap<String, usize> {
    let mut sizes = BTreeMap::new();
    fs.into_iter().rev().for_each(|(dir, view)| {
        let size = view
            .entries
            .iter()
            .map(|e| match e {
                DirEntry::Dir(d) => *sizes.get(&normalize(&dir, d)).unwrap(),
                DirEntry::File(File { size, .. }) => *size,
            })
            .sum();
        sizes.insert(dir.clone(), size);
    });
    sizes
}

pub fn solve1(input: Vec<String>) -> usize {
    let dir_size = {
        let fs = parse_fs(input);
        sizes(fs)
    };
    dir_size.values().filter(|s| **s <= 100_000).sum()
}

pub fn solve2(input: Vec<String>) -> usize {
    const TOTAL: usize = 70_000_000;
    const MIN_UNUSED: usize = 30_000_000;
    let fs_size = {
        let fs = parse_fs(input);
        sizes(fs)
    };
    let total_used = fs_size.get("/").unwrap();
    let target = MIN_UNUSED - (TOTAL - total_used);
    fs_size
        .into_iter()
        .filter_map(|(_, s)| if s >= target { Some(s) } else { None })
        .min()
        .unwrap()
}
