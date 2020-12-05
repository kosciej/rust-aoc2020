#![allow(dead_code)]
use std::ops::Mul;

static SAMPLE_INPUT: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

static MAIN_INPUT: &str = include_str!("../../data/d3.txt");

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .split_whitespace()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn ex1_count_trees(input: &[Vec<bool>], (x_step, y_step): (usize, usize)) -> usize {
    let mut y = 0;
    let mut x = 0;
    let mut trees = 0;

    while y < input.len() {
        if input[y][x] {
            trees += 1;
        }

        x = (x + x_step) % input[0].len();
        y += y_step;
    }

    trees
}

fn ex2_count_slopes(input: &[Vec<bool>]) -> usize {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|slope| ex1_count_trees(input, *slope))
        .fold(1, Mul::mul)
}

#[test]
fn ex1_sample() {
    assert_eq!(7, ex1_count_trees(&parse(SAMPLE_INPUT), (3, 1)));
}

#[test]
fn ex1_main() {
    assert_eq!(284, ex1_count_trees(&parse(MAIN_INPUT), (3, 1)));
}

#[test]
fn ex2_sample() {
    assert_eq!(336, ex2_count_slopes(&parse(SAMPLE_INPUT)));
}

#[test]
fn ex2_main() {
    assert_eq!(3510149120, ex2_count_slopes(&parse(MAIN_INPUT)));
}
