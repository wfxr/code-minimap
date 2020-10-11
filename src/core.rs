use itertools::Itertools;
use std::cell::RefCell;
use std::cmp;
use std::io::{self, BufRead, Write};
use std::ops::Range;
use std::rc::Rc;
use std::vec::Vec;

/// Write minimap to output
pub fn write(
    output: Rc<RefCell<dyn Write>>,
    reader: Box<dyn BufRead>,
    start_line: Option<usize>,
    end_line: Option<usize>,
    hscale: f64,
    vscale: f64,
    padding: Option<usize>,
) -> io::Result<()> {
    let mut frame = vec![0..0; 4];

    let start = start_line.map(|s| cmp::max(s, 1)).unwrap_or(1);
    let lines = reader.lines().skip(start - 1);
    let lines: Box<dyn Iterator<Item=io::Result<String>>> = if let Some(end) = end_line {
        let take = if end >= start { end - start + 1 } else { 0 };
        Box::new(lines.take(take))
    } else {
        Box::new(lines)
    };

    for chunk in &lines.enumerate()
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
        frame.iter_mut().skip(chunk_size).for_each(|row| *row = 0..0);
        scale_frame(&mut frame, hscale);
        write_frame(output.clone(), &frame, padding)?;
    }
    Ok(())
}

/// Print minimap to stdout
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::io;
/// use std::io::BufReader;
///
/// let reader = Box::new(BufReader::new(io::stdin()));
/// code_minimap::print(reader, None, None, 1.0, 1.0, None).unwrap();
/// ```
pub fn print(reader: Box<dyn BufRead>, start_line: Option<usize>, end_line: Option<usize>, hscale: f64, vscale: f64, padding: Option<usize>) -> io::Result<()> {
    write(Rc::new(RefCell::new(io::stdout())), reader, start_line, end_line, hscale, vscale, padding)
}

/// Write minimap to string
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::io;
/// use std::io::BufReader;
///
/// let reader = Box::new(BufReader::new(io::stdin()));
/// let s = code_minimap::write_to_string(reader, None, None, 1.0, 1.0, None).unwrap();
/// print!("{}", s);
/// ```
pub fn write_to_string(
    reader: Box<dyn BufRead>,
    start_line: Option<usize>,
    end_line: Option<usize>,
    hscale: f64,
    vscale: f64,
    padding: Option<usize>,
) -> io::Result<String> {
    let buf = Rc::new(RefCell::new(Vec::new()));
    write(buf.clone(), reader, start_line, end_line, hscale, vscale, padding)?;
    let buf = Rc::try_unwrap(buf).unwrap().into_inner();
    Ok(String::from_utf8(buf).unwrap())
}

fn write_frame(output: Rc<RefCell<dyn Write>>, frame: &[Range<usize>], padding: Option<usize>) -> std::io::Result<()> {
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
        Some(padding) => writeln!(output.borrow_mut(), "{0:<1$}", line, padding),
        None => writeln!(output.borrow_mut(), "{}", line),
    }
}

fn scale_frame(frame: &mut [Range<usize>], factor: f64) {
    for x in frame.iter_mut() {
        *x = scale(x.start, factor)..scale(x.end, factor);
    }
}

fn scale(x: usize, factor: f64) -> usize {
    (x as f64 * factor) as usize
}

#[rustfmt::skip]
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

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest(
        input,
        expected,
        case("", ""),
        case("a", "⠁"),
        case("aaaa\nbbbb\ncccc\ndddd", "⣿⣿"),
        case("aaa\n aa\n  a\n   a", "⠙⢇"),
        case("  a  b c\n d efg  \n    h  i\n jk", "⢐⡛⠿⠭")
    )]
    fn test_write_to_string(input: &'static str, expected: &str) {
        let actual = write_to_string(Box::new(input.as_bytes()), None, None, 1.0, 1.0, None).unwrap();
        assert_eq!(expected, actual.trim());
    }
}
