use regex::Regex;
use std::fs;
use std::iter::Peekable;
use std::str::Lines;

#[derive(Debug)]
enum Leaf {
    Dir(String, Vec<Leaf>),
    File(String, isize),
}

fn find_sum(leaf: &Leaf,score: &mut isize) ->isize{
    match leaf {
        Leaf::Dir(_, childs) => {
            let mut sum = 0;
            for child in childs {
                sum += find_sum(child,score);
            }
            if sum <= 100000 {
                *score += sum;
                sum
            }else {
                sum
            }
        }
        Leaf::File(_, size) => *size,
    }
}
fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut root = Leaf::Dir("".to_string(), Vec::new());

    let cd_regex = Regex::new(r"\$ cd (.+)").unwrap();
    let lst_regex = Regex::new(r"\$ ls").unwrap();

    parse_structure(
        &mut root,
        &mut contents.lines().peekable(),
        &cd_regex,
        &lst_regex,
    );

    dbg!(&root);

    let mut sum = 0;
    find_sum(&root, &mut sum);

    println!("{sum}");
}

fn parse_structure(
    this: &mut Leaf,
    lines: &mut Peekable<Lines>,
    cd_regex: &Regex,
    lst_regex: &Regex,
) {
    while let Some(line) = lines.next() {
        if cd_regex.is_match(line) {
            let data = cd_regex.captures(line).unwrap();
            let dir_parsed = data.get(1).unwrap().as_str();
            if dir_parsed == "/" {
                continue;
            } else if dir_parsed == ".." {
                return;
            } else {
                match this {
                    Leaf::Dir(_name, children) => {
                        children.push(Leaf::Dir(dir_parsed.to_string(), Vec::new()));
                        parse_structure(children.last_mut().unwrap(), lines, cd_regex, lst_regex);
                    }
                    Leaf::File(_, _) => panic!("should never recurse into a file"),
                }
            }
        } else if lst_regex.is_match(line) {
            while lines.peek().map(|l| !l.starts_with('$')).unwrap_or(false) {
                if let Some(line) = lines.next() {
                    match line.split_once(' ') {
                        Some(("dir", _name)) => {}
                        Some((size, name)) => match this {
                            Leaf::Dir(_name, children) => {
                                children.push(Leaf::File(name.to_string(), size.parse().unwrap()));
                            }
                            Leaf::File(_, _) => panic!("should never recurse into a file"),
                        },
                        None => panic!("invalid `ls` result"),
                    }
                } else {
                    continue;
                }
            }
        } else {
            panic!("invalid line: {line}");
        }
    }
}
