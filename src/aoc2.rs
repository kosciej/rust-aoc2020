#![allow(dead_code)]

static TEST_INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

static MAIN_INPUT: &str = include_str!("../data/d2.txt");

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

fn check1(entry: &Entry) -> bool {
    let (min, max, chr, pass) = entry;
    let count: usize = pass.chars().filter(|c| c == chr).count();
    count >= *min && count <= *max
}

fn check2(entry: &Entry) -> bool {
    let (pos1, pos2, chr, pass) = entry;
    let count: usize = pass
        .chars()
        .enumerate()
        .filter(|(i, c)| (i + 1 == *pos1 || i + 1 == *pos2) && c == chr)
        .count();
    count == 1
}

fn count_valid1(entries: Vec<Entry>) -> usize {
    entries.iter().filter(|e| check1(e)).count()
}

fn count_valid2(entries: Vec<Entry>) -> usize {
    entries.iter().filter(|e| check2(e)).count()
}

#[test]
fn test_sample1() {
    println!("{:?}", parse(TEST_INPUT));
    assert_eq!(2, count_valid1(parse(TEST_INPUT)));
}

#[test]
fn test_main1() {
    assert_eq!(422, count_valid1(parse(MAIN_INPUT)));
}

#[test]
fn test_sample2() {
    println!("{:?}", parse(TEST_INPUT));
    assert_eq!(1, count_valid2(parse(TEST_INPUT)));
}

#[test]
fn test_main2() {
    assert_eq!(451, count_valid2(parse(MAIN_INPUT)));
}
