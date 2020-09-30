use itertools::Itertools;
use std::cmp;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::PathBuf;

/// Options for printing minimap
pub struct Opt {
    /// File path to read
    pub file: Option<PathBuf>,

    /// horizontal scale factor
    pub hscale: f64,

    /// vertical scale factor
    pub vscale: f64,

    /// padding width
    pub padding: Option<usize>,
}

/// Print the minimap to stdout
pub fn print_minimap(reader: Box<dyn BufRead>, opt: &Opt) -> io::Result<()> {
    let (hscale, vscale) = (opt.hscale, opt.vscale);
    let mut frame = vec![0..0; 4];
    for chunk in &reader
        .lines()
        .enumerate()
        .map(|(i, line)| (scale(i, vscale), line))
        .group_by(|(i, _)| *i)
        .into_iter()
        .chunks(4)
    {
        let mut chunk_size = 0;
        for (i, (_, group)) in chunk.enumerate() {
            let (mut beg, mut end) = (usize::max_value(), 0);
            for (_, line) in group {
                let line: String = line?;
                beg = cmp::min(beg, line.find(|c: char| !c.is_whitespace()).unwrap_or(beg));
                end = cmp::max(end, line.rfind(|c: char| !c.is_whitespace()).unwrap_or(end));
            }
            frame[i] = beg..(end + 1);
            chunk_size += 1;
        }
        for i in chunk_size..4 {
            frame[i] = 0..0;
        }
        scale_frame(&mut frame, hscale);
        print_frame(&frame, opt.padding);
    }
    Ok(())
}

fn print_frame(frame: &Vec<Range<usize>>, padding: Option<usize>) {
    let idx = |pos| {
        frame
            .iter()
            .enumerate()
            .fold(0, |acc, (i, x)| if x.contains(&pos) { acc + (1 << i) } else { acc })
    };
    let end = frame.iter().max_by_key(|range| range.end).unwrap().end;
    let line: String = (0..end)
        .step_by(2)
        .map(|i| BRAILLE_MATRIX[(idx(i)) + (idx(i + 1) << 4)])
        .collect();
    match padding {
        Some(padding) => println!("{0:<1$}", line, padding),
        None => println!("{}", line),
    }
}

fn scale_frame(frame: &mut Vec<Range<usize>>, factor: f64) {
    for x in frame.iter_mut() {
        *x = scale(x.start, factor)..scale(x.end, factor);
    }
}

fn scale(x: usize, factor: f64) -> usize {
    (x as f64 * factor) as usize
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
