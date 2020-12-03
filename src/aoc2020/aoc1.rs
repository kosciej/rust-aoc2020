#![allow(dead_code)]

static TEST_INPUT: &str = "1721
979
366
299
675
1456";

static MAIN_INPUT: &str = include_str!("../../data/d1.txt");

fn to_nums(s: &str) -> Vec<u32> {
    s.split_whitespace().flat_map(|d| d.parse()).collect()
}

fn solution_two(input: &[u32]) -> u32 {
    for i in input {
        for j in input {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    0
}

fn solution_three(input: &[u32]) -> u32 {
    for i in input {
        for j in input {
            for k in input {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    0
}

#[test]
fn test_sample1() {
    println!("{:?}", to_nums(TEST_INPUT));
    assert_eq!(514579, solution_two(&to_nums(TEST_INPUT)));
}

#[test]
fn test_task1() {
    println!("{:?}", to_nums(MAIN_INPUT));
    assert_eq!(646779, solution_two(&to_nums(MAIN_INPUT)));
}

#[test]
fn test_sample2() {
    println!("{:?}", to_nums(TEST_INPUT));
    assert_eq!(241861950, solution_three(&to_nums(TEST_INPUT)));
}

#[test]
fn test_task2() {
    println!("{:?}", to_nums(MAIN_INPUT));
    assert_eq!(246191688, solution_three(&to_nums(MAIN_INPUT)));
}
