use std::{
    io::{self, BufRead, Write},
    ops::Range,
};

use itertools::Itertools;

/// Write minimap to the writer.
pub fn write(
    mut writer: impl Write,
    reader: impl BufRead,
    hscale: f64,
    vscale: f64,
    padding: Option<usize>,
) -> io::Result<()> {
    let mut frame = [0..0, 0..0, 0..0, 0..0];
    reader
        .lines()
        .map(|line| {
            line.map(|line| {
                let beg = line.find(|c: char| !c.is_whitespace()).unwrap_or(usize::MAX);
                let end = line.rfind(|c: char| !c.is_whitespace()).unwrap_or(0);
                (beg, end)
            })
        })
        .enumerate()
        .map(|(i, line)| (scale(i, vscale), line))
        .chunk_by(|(i, _)| *i)
        .into_iter()
        .chunks(4)
        .into_iter()
        .try_for_each(|chunk| {
            let mut chunk_size = 0;
            for (i, (_, group)) in chunk.enumerate() {
                let (beg, end) = group.into_iter().try_fold((usize::MAX, 0), |(beg, end), (_, line)| {
                    line.map(|(b, e)| (beg.min(b), end.max(e)))
                })?;
                frame[i] = beg..(end + 1);
                chunk_size += 1;
            }
            frame.iter_mut().skip(chunk_size).for_each(|row| *row = 0..0);
            scale_frame(&mut frame, hscale);
            write_frame(&mut writer, &frame, padding)
        })
}

/// Print minimap to the stdout.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::{io, io::BufReader};
///
/// let stdin = io::stdin();
/// code_minimap::print(stdin.lock(), 1.0, 1.0, None).unwrap();
/// ```
pub fn print(reader: impl BufRead, hscale: f64, vscale: f64, padding: Option<usize>) -> io::Result<()> {
    write(io::stdout(), reader, hscale, vscale, padding)
}

/// Write minimap to a string.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::{io, io::BufReader};
///
/// let stdin = io::stdin();
/// let s =
///     code_minimap::write_to_string(stdin.lock(), 1.0, 1.0, None).unwrap();
/// print!("{}", s);
/// ```
pub fn write_to_string(reader: impl BufRead, hscale: f64, vscale: f64, padding: Option<usize>) -> io::Result<String> {
    let mut buf = Vec::new();
    write(&mut buf, reader, hscale, vscale, padding)?;
    Ok(String::from_utf8(buf).unwrap())
}

fn write_frame(mut writer: impl Write, frame: &[Range<usize>], padding: Option<usize>) -> std::io::Result<()> {
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
        Some(padding) => writeln!(writer, "{0:<1$}", line, padding),
        None => writeln!(writer, "{}", line),
    }
}

fn scale_frame(frame: &mut [Range<usize>], factor: f64) {
    for x in frame {
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
    use rstest::*;

    use super::*;

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
