#![allow(dead_code)]

static SAMPLE_INPUT: &str = "\
";

static MAIN_INPUT: &str = include_str!("../../data/d_.txt");

type Data = u32;

fn parse(_input: &str) -> Vec<Data> {
    vec![]
}

fn ex1(_data: &[Data]) -> usize {
    1
}

fn ex2(_data: &[Data]) -> usize {
    1
}

#[test]
fn ex1_sample() {
    assert_eq!(101, ex1(&parse(SAMPLE_INPUT)));
}

#[test]
fn ex1_main() {
    assert_eq!(101, ex1(&parse(MAIN_INPUT)));
}

#[test]
fn ex2_sample() {
    assert_eq!(101, ex2(&parse(SAMPLE_INPUT)));
}

#[test]
fn ex2_main() {
    assert_eq!(101, ex2(&parse(MAIN_INPUT)));
}
