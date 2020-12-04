#![allow(dead_code)]
use std::ops::Mul;

static TEST_INPUT: &str = "\
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

fn count_trees(input: &[Vec<bool>], (x_step, y_step): (usize, usize)) -> usize {
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

fn ex_2(input: &[Vec<bool>]) -> usize {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|slope| count_trees(input, *slope))
        .fold(1, Mul::mul)
}

#[test]
fn test_sample1() {
    println!("{:?}", parse(TEST_INPUT));
    assert_eq!(7, count_trees(&parse(TEST_INPUT), (3, 1)));
}

#[test]
fn test_main1() {
    assert_eq!(284, count_trees(&parse(MAIN_INPUT), (3, 1)));
}

#[test]
fn test_sample2() {
    assert_eq!(336, ex_2(&parse(TEST_INPUT)));
}

#[test]
fn test_main2() {
    assert_eq!(3510149120, ex_2(&parse(MAIN_INPUT)));
}
