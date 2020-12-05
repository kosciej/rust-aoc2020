#![allow(dead_code)]

static SAMPLE_INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

static MAIN_INPUT: &str = include_str!("../../data/d2.txt");

type Entry = (usize, usize, char, String);

fn parse(input: &str) -> Vec<Entry> {
    input.lines().flat_map(parse_line).collect()
}

fn parse_line(input: &str) -> Option<Entry> {
    let spl: Vec<_> = input.split_ascii_whitespace().collect();
    let first: Vec<_> = spl[0].split('-').collect();
    let (min, max) = match &first as &[&str] {
        [min, .., max] => (min, max),
        _ => return None,
    };
    let chr = spl[1].chars().next()?;
    let pass = spl[2];
    Some((min.parse().ok()?, max.parse().ok()?, chr, pass.to_string()))
}

fn ex1_check(entry: &Entry) -> bool {
    let (min, max, chr, pass) = entry;
    let count: usize = pass.chars().filter(|c| c == chr).count();
    count >= *min && count <= *max
}

fn ex2_check(entry: &Entry) -> bool {
    let (pos1, pos2, chr, pass) = entry;
    let count: usize = pass
        .chars()
        .enumerate()
        .filter(|(i, c)| (i + 1 == *pos1 || i + 1 == *pos2) && c == chr)
        .count();
    count == 1
}

fn ex1_count_valid(entries: Vec<Entry>) -> usize {
    entries.iter().filter(|e| ex1_check(e)).count()
}

fn ex2_count_valid(entries: Vec<Entry>) -> usize {
    entries.iter().filter(|e| ex2_check(e)).count()
}

#[test]
fn ex1_sample() {
    assert_eq!(2, ex1_count_valid(parse(SAMPLE_INPUT)));
}

#[test]
fn ex1_main() {
    assert_eq!(422, ex1_count_valid(parse(MAIN_INPUT)));
}

#[test]
fn ex2_sample() {
    assert_eq!(1, ex2_count_valid(parse(SAMPLE_INPUT)));
}

#[test]
fn ex2_main() {
    assert_eq!(451, ex2_count_valid(parse(MAIN_INPUT)));
}
