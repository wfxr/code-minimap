use std::{
    io::{self, BufRead, Write},
    ops::Range,
};

use itertools::Itertools;

/// Render mode for the minimap output.
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum RenderMode {
    /// Use braille characters (⣿⣿⣿). Each character represents a 4×2 grid.
    #[default]
    Braille,
    /// Use block characters (███). Each character represents a single cell.
    Block,
}

impl RenderMode {
    fn rows(&self) -> usize {
        match self {
            Self::Braille => 4,
            Self::Block => 1,
        }
    }
}

/// Write minimap to the writer.
pub fn write(
    mut writer: impl Write,
    reader: impl BufRead,
    hscale: f64,
    vscale: f64,
    padding: Option<usize>,
    mode: RenderMode,
) -> io::Result<()> {
    let rows = mode.rows();
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
        .chunks(rows)
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
            frame[chunk_size..rows].iter_mut().for_each(|row| *row = 0..0);
            let frame = &mut frame[..rows];
            scale_frame(frame, hscale);
            match mode {
                RenderMode::Braille => write_frame_braille(&mut writer, frame, padding),
                RenderMode::Block => write_frame_block(&mut writer, frame, padding),
            }
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
/// code_minimap::print(stdin.lock(), 1.0, 1.0, None, Default::default()).unwrap();
/// ```
pub fn print(
    reader: impl BufRead,
    hscale: f64,
    vscale: f64,
    padding: Option<usize>,
    mode: RenderMode,
) -> io::Result<()> {
    write(io::stdout(), reader, hscale, vscale, padding, mode)
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
///     code_minimap::write_to_string(stdin.lock(), 1.0, 1.0, None, Default::default()).unwrap();
/// print!("{}", s);
/// ```
pub fn write_to_string(
    reader: impl BufRead,
    hscale: f64,
    vscale: f64,
    padding: Option<usize>,
    mode: RenderMode,
) -> io::Result<String> {
    let mut buf = Vec::new();
    write(&mut buf, reader, hscale, vscale, padding, mode)?;
    Ok(String::from_utf8(buf).unwrap())
}

fn write_frame_braille(mut writer: impl Write, frame: &[Range<usize>], padding: Option<usize>) -> io::Result<()> {
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

fn write_frame_block(mut writer: impl Write, frame: &[Range<usize>], padding: Option<usize>) -> io::Result<()> {
    let range = &frame[0];
    if range.start >= range.end {
        return match padding {
            Some(padding) => writeln!(writer, "{0:<1$}", "", padding),
            None => writeln!(writer),
        };
    }
    let line: String = (0..range.end)
        .map(|i| if range.contains(&i) { '█' } else { ' ' })
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
    fn test_write_to_string_braille(input: &'static str, expected: &str) {
        let actual = write_to_string(input.as_bytes(), 1.0, 1.0, None, RenderMode::Braille).unwrap();
        assert_eq!(expected, actual.trim());
    }

    #[rstest(
        input,
        expected,
        case("", ""),
        case("a", "█"),
        case("aaaa\nbbbb\ncccc\ndddd", "████\n████\n████\n████"),
        case("aaa\n aa\n  a\n   a", "███\n ██\n  █\n   █")
    )]
    fn test_write_to_string_block(input: &'static str, expected: &str) {
        let actual = write_to_string(input.as_bytes(), 1.0, 1.0, None, RenderMode::Block).unwrap();
        assert_eq!(expected, actual.trim());
    }
}
