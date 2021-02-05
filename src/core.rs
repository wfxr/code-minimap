use itertools::Itertools;
use std::cmp;
use std::io::{self, BufRead, Write};
use std::ops::Range;

/// Write minimap to the writer.
pub fn write(
    mut writer: impl Write,
    reader: impl BufRead,
    hscale: f64,
    vscale: f64,
    padding: Option<usize>,
) -> io::Result<()> {
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
        frame.iter_mut().skip(chunk_size).for_each(|row| *row = 0..0);
        scale_frame(&mut frame, hscale);
        write_frame(&mut writer, &frame, padding)?;
    }
    Ok(())
}

/// Print minimap to the stdout.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::io;
/// use std::io::BufReader;
///
/// let stdin = io::stdin();
/// code_minimap::printstd(stdin.lock(), 1.0, 1.0, None).unwrap();
/// ```
pub fn printstd(reader: impl BufRead, hscale: f64, vscale: f64, padding: Option<usize>) -> io::Result<()> {
    write(io::stdout(), reader, hscale, vscale, padding)
}

/// Write minimap to a string.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::io;
/// use std::io::BufReader;
///
/// let stdin = io::stdin();
/// let s = code_minimap::write_to_string(stdin.lock(), 1.0, 1.0, None).unwrap();
/// print!("{}", s);
/// ```
pub fn write_to_string(reader: impl BufRead, hscale: f64, vscale: f64, padding: Option<usize>) -> io::Result<String> {
    let mut buf = Vec::new();
    write(&mut buf, reader, hscale, vscale, padding)?;
    Ok(String::from_utf8(buf).unwrap())
}

fn write_frame(mut output: impl Write, frame: &[Range<usize>], padding: Option<usize>) -> std::io::Result<()> {
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
        Some(padding) => writeln!(output, "{0:<1$}", line, padding),
        None => writeln!(output, "{}", line),
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
        let actual = write_to_string(input.as_bytes(), 1.0, 1.0, None).unwrap();
        assert_eq!(expected, actual.trim());
    }
}
