use itertools::Itertools;
use std::cmp;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let h_scale = args.next().unwrap_or("1.0".into()).parse().unwrap();
    let v_scale = args.next().unwrap_or("1.0".into()).parse().unwrap();
    let mut braille = vec![0..=0; 4];
    let stdin = io::stdin();
    for chunk in &stdin
        .lock()
        .lines()
        .enumerate()
        .map(|(i, line)| (scale(i, v_scale), line))
        .group_by(|(i, _)| *i)
        .into_iter()
        .chunks(4)
    {
        for (i, (_, group)) in chunk.enumerate() {
            let (mut beg, mut end) = (0, 0);
            for (_, line) in group {
                let line: String = line?;
                beg = cmp::min(beg, line.find(|c: char| !c.is_whitespace()).unwrap_or(0));
                end = cmp::max(end, line.rfind(|c: char| !c.is_whitespace()).unwrap_or(line.len()));
            }
            braille[i] = beg..=end;
        }
        braille = adjust_width(&braille, h_scale);
        print_braille(&braille);
    }
    Ok(())
}

fn print_braille(matrix: &Vec<RangeInclusive<usize>>) {
    let end = matrix.iter().max_by_key(|range| range.end()).unwrap().end();
    for i in (0..=*end).step_by(2) {
        let idx1 = char_idx(matrix, i);
        let idx2 = char_idx(matrix, i + 1) << 4;
        print!("{}", BRAILLE_MATRIX[idx1 + idx2]);
    }
    println!();
}

fn char_idx(matrix: &Vec<RangeInclusive<usize>>, pos: usize) -> usize {
    matrix
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| if x.contains(&pos) { acc + (1 << i) } else { acc })
}

fn adjust_width(matrix: &Vec<RangeInclusive<usize>>, factor: f64) -> Vec<RangeInclusive<usize>> {
    matrix
        .iter()
        .map(|x| RangeInclusive::new(scale(*x.start(), factor), scale(*x.end(), factor)))
        .collect()
}

fn scale(x: usize, factor: f64) -> usize {
    (x as f64 * factor).round() as usize
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const BRAILLE_MATRIX : [char; 256] = [
    '⠀', '⠁', '⠂', '⠃', '⠄', '⠅', '⠆', '⠇', '⡀', '⡁', '⡂', '⡃', '⡄', '⡅', '⡆', '⡇',
    '⠈', '⠉', '⠊', '⠋', '⠌', '⠍', '⠎', '⠏', '⡈', '⡉', '⡊', '⡋', '⡌', '⡍', '⡎', '⡏',
    '⠐', '⠑', '⠒', '⠓', '⠔', '⠕', '⠖', '⠗', '⡐', '⡑', '⡒', '⡓', '⡔', '⡕', '⡖', '⡗',
    '⠘', '⠙', '⠚', '⠛', '⠜', '⠝', '⠞', '⠟', '⡘', '⡙', '⡚', '⡛', '⡜', '⡝', '⡞', '⡟',
    '⠠', '⠡', '⠢', '⠣', '⠤', '⠥', '⠦', '⠧', '⡠', '⡡', '⡢', '⡣', '⡤', '⡥', '⡦', '⡧',
    '⠨', '⠩', '⠪', '⠫', '⠬', '⠭', '⠮', '⠯', '⡨', '⡩', '⡪', '⡫', '⡬', '⡭', '⡮', '⡯',
    '⠰', '⠱', '⠲', '⠳', '⠴', '⠵', '⠶', '⠷', '⡰', '⡱', '⡲', '⡳', '⡴', '⡵', '⡶', '⡷',
    '⠸', '⠹', '⠺', '⠻', '⠼', '⠽', '⠾', '⠿', '⡸', '⡹', '⡺', '⡻', '⡼', '⡽', '⡾', '⡿',
    '⢀', '⢁', '⢂', '⢃', '⢄', '⢅', '⢆', '⢇', '⣀', '⣁', '⣂', '⣃', '⣄', '⣅', '⣆', '⣇',
    '⢈', '⢉', '⢊', '⢋', '⢌', '⢍', '⢎', '⢏', '⣈', '⣉', '⣊', '⣋', '⣌', '⣍', '⣎', '⣏',
    '⢐', '⢑', '⢒', '⢓', '⢔', '⢕', '⢖', '⢗', '⣐', '⣑', '⣒', '⣓', '⣔', '⣕', '⣖', '⣗',
    '⢘', '⢙', '⢚', '⢛', '⢜', '⢝', '⢞', '⢟', '⣘', '⣙', '⣚', '⣛', '⣜', '⣝', '⣞', '⣟',
    '⢠', '⢡', '⢢', '⢣', '⢤', '⢥', '⢦', '⢧', '⣠', '⣡', '⣢', '⣣', '⣤', '⣥', '⣦', '⣧',
    '⢨', '⢩', '⢪', '⢫', '⢬', '⢭', '⢮', '⢯', '⣨', '⣩', '⣪', '⣫', '⣬', '⣭', '⣮', '⣯',
    '⢰', '⢱', '⢲', '⢳', '⢴', '⢵', '⢶', '⢷', '⣰', '⣱', '⣲', '⣳', '⣴', '⣵', '⣶', '⣷',
    '⢸', '⢹', '⢺', '⢻', '⢼', '⢽', '⢾', '⢿', '⣸', '⣹', '⣺', '⣻', '⣼', '⣽', '⣾', '⣿',
];
