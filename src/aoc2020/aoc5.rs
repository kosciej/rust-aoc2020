#![allow(dead_code)]

static SAMPLE_INPUT: &str = "\
FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

static SAMPLE_RESULTS: [usize; 4] = [
    357,
    567,
    119,
    820
];

static MAIN_INPUT: &str = include_str!("../../data/d5.txt");

type Data = (u8, u8);

fn parse(input: &str) -> Vec<Data> {
    input.lines().map(convert).collect()
}

fn convert(input: &str) -> (u8, u8) {
    let row: u8 = input.chars().take(7).map(|c| if c == 'B' || c == 'R' { 1 } else { 0 })
    .fold(0b000_0000, |acc: u8, c: u8| {
        let acc = acc + c;
        acc.rotate_left(1)
    });
    let row = row.rotate_right(1);

    let seat: u8 = input.chars().skip(7).map(|c| if c == 'B' || c == 'R' { 1 } else { 0 })
    .fold(0b000_0000, |acc: u8, c: u8| {
        let acc = acc + c;
        acc.rotate_left(1)
    }).rotate_right(1);
    (row,seat)
}

fn to_id(row: u8, seat:u8) -> usize {
    (row as usize) * 8 + (seat as usize)
}

fn ex1(data: &[Data]) -> usize {
    data.iter().map(|(row, seat)|to_id(*row, *seat)).max().unwrap()
}

fn ex2(data: &[Data]) -> usize {
    let mut p: Vec<_> = data.iter().map(|(row, seat)|to_id(*row, *seat)).collect();
    p.sort_unstable();
    let mut it = p.iter();
    let mut previous = *it.next().unwrap();
    for i in it {
        if previous + 1 != *i {
            return previous + 1;
        } else {
            previous = *i;
        }
    }
    0
}

#[test]
fn parser_test() {
    let (row, seat) = convert("FBFBBFFRLR");
    assert_eq!(44, row);
    assert_eq!(5, seat);
    assert_eq!(357, to_id(row, seat));
}

#[test]
fn ex1_sample() {
    assert_eq!(820, ex1(&parse(SAMPLE_INPUT)));
}

#[test]
fn ex1_main() {
    assert_eq!(951, ex1(&parse(MAIN_INPUT)));
}

#[test]
fn ex2_main() {
    assert_eq!(653, ex2(&parse(MAIN_INPUT)));
}
