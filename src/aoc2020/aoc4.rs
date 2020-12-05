#![allow(dead_code)]
use std::collections::HashMap;
use std::mem;

static SAMPLE_INPUT: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

static MAIN_INPUT: &str = include_str!("../../data/d4.txt");

type Data = HashMap<String, String>;

fn parse(input: &str) -> Vec<Data> {
    input.lines().scan(HashMap::<String,String>::new(), |state, line| {
        if line.trim().is_empty() {
            let mut hmap = HashMap::new();
            mem::swap(state, &mut hmap);
            Some(Some(hmap))
        } else {
            line.split_whitespace().map(|token| {
                let mut it = token.split(':');
                (it.next().unwrap(), it.next().unwrap())
            }).for_each(|(k, v)| {(*state).insert(k.to_string(),v.to_string());});
            Some(None)
        }
    }).flatten().collect()
}

fn validate(d: &Data) -> bool {
    d.contains_key("ecl") &&
    d.contains_key("pid") &&
    d.contains_key("eyr") &&
    d.contains_key("hcl") &&
    d.contains_key("byr") &&
    d.contains_key("iyr") &&
    d.contains_key("hgt")
}

fn ex1(data: &[Data]) -> usize {
    data.iter().filter(|d| validate(*d)).count()
}

fn ex2(_data: &[Data]) -> usize {
    1
}

#[test]
fn ex1_sample() {
    assert_eq!(2, ex1(&parse(SAMPLE_INPUT)));
}

#[test]
fn ex1_main() {
    assert_eq!(196, ex1(&parse(MAIN_INPUT)));
}

// #[test]
// fn ex2_sample() {
//     assert_eq!(101, ex2(&parse(SAMPLE_INPUT)));
// }

// #[test]
// fn ex2_main() {
//     assert_eq!(101, ex2(&parse(MAIN_INPUT)));
// }
